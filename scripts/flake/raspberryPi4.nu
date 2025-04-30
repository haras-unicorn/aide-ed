#!/usr/bin/env nu

let dir = $env.FILE_PWD
let self = [ $dir "raspberryPi4.nu" ] | path join
let root = $dir | path dirname --num-levels 2
let artifacts = [ $root "artifacts" ] | path join
let flake = $"path:($root)"
let system = "aarch64-linux"
let format = "sd-aarch64"
let configuration = $"raspberryPi4-($system)"
let uri = $"($flake)#($configuration)"

def "main" [] {
  nu $self --help
}

def "main secrets" [] {
  rm -rf $artifacts
  mkdir $artifacts
  cd $artifacts

  spec | to json | rumor stdin json --stay
}

def "main image" [] {
  rm -rf $artifacts
  mkdir $artifacts
  cd $artifacts

  let raw = (nixos-generate
    --system $system
    --format $format
    --flake $uri)

  let compressed = ls ($raw
    | path dirname --num-levels 2
    | path join "sd-image")
    | get name
    | first
  unzstd $compressed -o image.img
  chmod 644 image.img

  let age = vault secrets
    | get age-priv
    | str replace -a "\\" "\\\\"
    | str replace -a "\n" "\\n"
    | str replace -a "\"" "\\\""

  let commands = $"run
mount /dev/sda2 /
mkdir-p /root
chmod 700 /root
write /root/.sops.age \"($age)\"
chmod 400 /root/.sops.age
exit"

  echo $commands | guestfish --rw -a image.img
}

def "main ssh" [] {
  let key = vault secrets
    | get user-ssh-priv
    | str trim

  let hostname = lib secrets
    | get hostname
    | str trim

  ssh-agent bash -c $"echo '($key)' \\
    | ssh-add - \\
    && ssh ($env.USER)@($hostname)"
}

def "main pass" [] {
  vault secrets
    | get user-pass-priv
    | str trim
}

def "main deploy" [] {
  let secrets = vault secrets

  let key = $secrets
    | get user-ssh-priv
    | str trim

  let pass = $secrets
    | get user-pass-priv
    | str trim

  let hostname = lib secrets
    | get hostname
    | str trim

  ssh-agent bash -c $"echo '($key)' \\
    | ssh-add - \\
    && export SSHPASS='($pass)' \\
    && sshpass -e deploy \\
      --skip-checks \\
      --interactive-sudo true \\
      --hostname ($hostname) \\
      -- \\
      '($root)#($configuration)'"
}

def "main db user" [] {
  let pass = vault secrets
    | get postgres-user-pass
    | str trim

  let hostname = lib secrets
    | get hostname
    | str trim

  let auth = $"($env.USER):($pass)"
  let conn = $"($hostname):5432"

  usql $"postgres://($auth)@($conn)/aide-ed"
}

def "main db admin" [] {
  let pass = vault secrets
    | get postgres-pass
    | str trim

  let hostname = lib secrets
    | get hostname
    | str trim

  let auth = $"($env.USER):($pass)"
  let conn = $"($hostname):5432"

  usql $"postgres://($auth)@($conn)/postgres"
}

def "spec" [] {
  let expr = $"\(builtins.getFlake \"($flake)\"\).lib.rumor.\"($configuration)\""
  let spec = nix eval --json --impure --expr $expr
  $spec | from json
}

def "lib secrets" [] {
  let expr = $"\(builtins.getFlake \"($flake)\"\).lib.secrets.\"($configuration)\""
  let secrets = nix eval --json --impure --expr $expr
  $secrets | from json
}

def "vault secrets" [] {
  let hostname = lib secrets | get hostname

  vault kv get -format=json $"kv/aide-ed/aide-ed/($hostname)/current"
    | from json
    | get data.data
}
