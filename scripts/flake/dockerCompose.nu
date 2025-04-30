#!/usr/bin/env nu

let self = [ $env.FILE_PWD "dockerCompose.nu" ] | path join

def "main" [] {
  nu -c $"($self) -h" 
}

def --wrapped "main up" [...args] {
  if (is cuda) {
    docker compose --profile "cuda" up -d ...($args)
  } else {
    docker compose --profile "cpu" up -d ...($args)
  }
}

def --wrapped "main down" [...args] {
  docker compose --profile "*" down ...($args)
}

def "main ready" [] {
  loop {
    try {
      let postgres_container_id = (docker compose ps --format json
        | lines
        | each { $in | from json }
        | filter { $in.Image | str starts-with "postgres" }
        | first
        | get id);
      docker exec $postgres_container_id pg_isready --host localhost;
      let ollama_container_id = (docker compose ps --format json
        | lines
        | each { $in | from json }
        | filter { $in.Image | str starts-with "ollama" }
        | first
        | get id)
      (docker exec $ollama_container_id
        ollama pull deepseek-r1:14b-qwen-distill-q4_K_M)
      (docker exec $ollama_container_id
        ollama cp deepseek-r1:14b-qwen-distill-q4_K_M deepseek-chat)
      let keycloak_container_id = (docker compose ps --format json
        | lines
        | each { $in | from json }
        | filter { $in.Image | str starts-with "keycloak" }
        | first
        | get id)
      break
    } catch {
      sleep 1sec;
      continue
    }
  }
}

def "main postgres host" [] {
  let postgres_host = inspect aide-ed-postgres-1
    | get NetworkSettings.Networks.aide-ed-network.IPAddress
  return $postgres_host
}

def "main postgres port" [] {
  let postgres_port = inspect aide-ed-postgres-1
    | get NetworkSettings.Ports
    | transpose key value
    | get key
    | parse "{port}/tcp"
    | get port
    | first
  return $postgres_port
}

def "main postgres database" [] {
  let postgres_database = inspect aide-ed-postgres-1
    | get Config.Env
    | parse "{key}={value}"
    | where $it.key == "POSTGRES_DB"
    | get value
    | first
  return $postgres_database
}

def "main postgres user" [] {
  let postgres_user = inspect aide-ed-postgres-1
    | get Config.Env
    | parse "{key}={value}"
    | where $it.key == "POSTGRES_USER"
    | get value
    | first
  return $postgres_user
}

def "main postgres password" [] {
  let postgres_password = inspect aide-ed-postgres-1
    | get Config.Env
    | parse "{key}={value}"
    | where $it.key == "POSTGRES_PASSWORD"
    | get value
    | first
  return $postgres_password
}

def "main openai host" [] {
  let openai_host = inspect (ollama host)
    | get NetworkSettings.Networks.aide-ed-network.IPAddress
  return $openai_host
}

def "main openai port" [] {
  let openai_port = inspect (ollama host)
    | get NetworkSettings.Ports
    | transpose key value
    | get key
    | parse "{port}/tcp"
    | get port
    | first
  return $openai_port
}

def "main keycloak url" [] {
  let keycloak_host = inspect aide-ed-keycloak-1
    | get NetworkSettings.Networks.aide-ed-network.IPAddress
  return $keycloak_host
}

def "main keycloak user" [] {
  let keycloak_user = inspect aide-ed-keycloak-1
    | get Config.Env
    | parse "{key}={value}"
    | where $it.key == "KC_BOOTSTRAP_ADMIN_USERNAME"
    | get value
    | first
  return $keycloak_user
}

def "main keycloak password" [] {
  let keycloak_password = inspect aide-ed-keycloak-1
    | get Config.Env
    | parse "{key}={value}"
    | where $it.key == "KC_BOOTSTRAP_ADMIN_PASSWORD"
    | get value
    | first
  return $keycloak_password
}

def "main openai api-key" [] {
  return "ollama"
}

def "ollama host" [] {
  if (is cuda) {
    "aide-ed-ollama-cuda-1"
  } else {
    "aide-ed-ollama-cpu-1"
  }
}

def "is cuda" [] {
  (docker run --rm --device=nvidia.com/gpu=all hello-world
      | complete
      | get exit_code) == 0
}

def "inspect" [container: string] {
  docker container inspect $container
    | from json
    | first
}
