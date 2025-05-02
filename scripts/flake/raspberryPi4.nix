{ self
, pkgs
, config
, nixos-hardware
, sops-nix
, ...
}:

let
  secrets = self.lib.secrets."raspberryPi4-aarch64-linux";
  secretKeys = secrets.keys;

  keycloakSetupScript = pkgs.writeShellApplication {
    name = "keycloak-setup";
    runtimeInputs = [ pkgs.jq config.services.keycloak.package ];
    text = ''
      kcadm.sh config credentials \
        --server http://localhost:${builtins.toString config.services.keycloak.settings.http-port}/ \
        --realm master \
        --user admin \
        --password "$(cat ${config.services.keycloak.initialAdminPassword})" \
        || { echo "Failed to authenticate with initial admin"; exit 1; }

      user_id=$(kcadm.sh get users \
        -r master \
        -q username=aide-ed \
        --fields id \
        | jq -r '.[0].id // empty')
      if [ -z "$user_id" ]; then
        kcadm.sh create users \
          -r master \
          -s username=aide-ed \
          -s enabled=true \
          || { echo "Failed to create user 'aide-ed'"; exit 1; }
      fi

      kcadm.sh set-password \
        -r master \
        --username aide-ed \
        --new-password "$(cat ${config.sops.secrets."${secretKeys.keycloakPassword}".path})" \
        || { echo "Failed to set password for 'aide-ed'"; exit 1; }

      admin_id=$(kcadm.sh get users \
        -r master \
        -q username=admin \
        --fields id \
        | ${pkgs.jq}/bin/jq -r '.[0].id')
      kcadm.sh update \
        "users/$admin_id" \
        -r master \
        -s enabled=false \
        || { echo "Failed to disable user 'admin'"; exit 1; }
    '';
  };
in
{
  seal.overlays.raspberryPi4 = (final: prev: {
    # NOTE: https://github.com/NixOS/nixpkgs/issues/154163#issuecomment-1008362877  
    makeModulesClosure = x: prev.makeModulesClosure
      (x // { allowMissing = true; });
  });

  seal.deploy.nodes.raspberryPi4 = {
    hostname = secrets.hostname;
    sshUser = "haras";
  };

  integrate.nixosConfiguration = {
    systems = [ "aarch64-linux" ];
    nixpkgs.overlays = self.lib.rust.overlays;

    nixosConfiguration = {
      nixpkgs.overlays = [
        self.overlays.raspberryPi4
      ];

      nixpkgs.config = {
        allowUnfree = true;
      };

      imports = [
        nixos-hardware.nixosModules.raspberry-pi-4
        sops-nix.nixosModules.default
        self.nixosModules.default
      ];

      # system 

      system.stateVersion = "24.11";

      nix.extraOptions = "experimental-features = nix-command flakes";
      nix.gc.automatic = true;
      nix.gc.options = "--delete-older-than 30d";
      nix.settings.auto-optimise-store = true;
      nix.settings.trusted-users = [ "@wheel" ];
      nix.package = pkgs.nixVersions.stable;

      sops.defaultSopsFile = "${self}/${secrets.filePrefix}";
      sops.age.keyFile = secrets.ageKeyFile;

      networking.hostName = secrets.hostname;

      fileSystems."/firmware" = {
        device = "/dev/disk/by-label/FIRMWARE";
        fsType = "vfat";
      };
      fileSystems."/" = {
        device = "/dev/disk/by-label/NIXOS_SD";
        fsType = "ext4";
      };

      environment.systemPackages = with pkgs; [
        libraspberrypi
        raspberrypi-eeprom
        man-pages
        man-pages-posix
      ];

      services.fstrim.enable = true;

      # postgresql

      services.postgresql.enable = true;
      services.postgresql.package = pkgs.postgresql_16;
      services.postgresql.authentication = pkgs.lib.mkOverride 10 ''
        # NOTE: do not remove local privileges because that breaks timescaledb
        # TYPE    DATABASE    USER        ADDRESS         METHOD        OPTIONS
        local     all         all                         trust
        host      all         all         samehost        trust
        hostssl   all         all         192.168.0.0/16  scram-sha-256
        hostssl   all         all         10.0.0.0/8      scram-sha-256
      '';
      services.postgresql.enableTCPIP = true;
      services.postgresql.settings.port = 5432;
      networking.firewall.allowedTCPPorts = [ 5432 ];
      services.postgresql.settings.ssl = "on";
      services.postgresql.settings.ssl_cert_file =
        config.sops.secrets.${secretKeys.postgresSslCertFile}.path;
      sops.secrets.${secretKeys.postgresSslCertFile} = {
        owner = config.systemd.services.postgresql.serviceConfig.User;
        group = config.systemd.services.postgresql.serviceConfig.Group;
      };
      services.postgresql.settings.ssl_key_file =
        config.sops.secrets.${secretKeys.postgresSslKeyFile}.path;
      sops.secrets.${secretKeys.postgresSslKeyFile} = {
        owner = config.systemd.services.postgresql.serviceConfig.User;
        group = config.systemd.services.postgresql.serviceConfig.Group;
      };
      services.postgresql.initialScript =
        config.sops.secrets.${secretKeys.postgresInitialScript}.path;
      sops.secrets.${secretKeys.postgresInitialScript} = {
        owner = config.systemd.services.postgresql.serviceConfig.User;
        group = config.systemd.services.postgresql.serviceConfig.Group;
      };

      # keycloak

      services.keycloak.enable = true;
      services.keycloak.database.caCert = config.sops.secrets.${secretKeys.caCertFile}.path;
      sops.secrets.${secretKeys.caCertFile} = { };
      services.keycloak.database.type = "postgresql";
      services.keycloak.database.host = "localhost";
      services.keycloak.database.port = 5432;
      services.keycloak.database.name = "keycloak";
      services.keycloak.database.username = "kaycloak";
      services.keycloak.database.passwordFile = config.sops.secrets.${secretKeys.keycloakPostgresPassword}.path;
      sops.secrets.${secretKeys.keycloakPostgresPassword} = {
        owner = config.systemd.services.keycloak.serviceConfig.User;
        group = config.systemd.services.keycloak.serviceConfig.Group;
      };
      services.keycloak.settings.hostname = secrets.hostname;
      services.keycloak.settings.http-host = "localhost";
      services.keycloak.settings.http-port = 8080;
      services.keycloak.settings.https-port = 8443;
      services.keycloak.sslCertificate =
        config.sops.secrets.${secretKeys.keycloakSslCertFile}.path;
      sops.secrets.${secretKeys.keycloakSslCertFile} = {
        owner = config.systemd.services.keycloak.serviceConfig.User;
        group = config.systemd.services.keycloak.serviceConfig.Group;
      };
      services.keycloak.sslCertificateKey =
        config.sops.secrets.${secretKeys.keycloakSslKeyFile}.path;
      sops.secrets.${secretKeys.keycloakSslKeyFile} = {
        owner = config.systemd.services.keycloak.serviceConfig.User;
        group = config.systemd.services.keycloak.serviceConfig.Group;
      };
      services.keycloak.initialAdminPassword = "admin";
      sops.secrets.${secretKeys.keycloakPassword} = { };
      systemd.services.setup-keycloak = {
        description = "Set up Keycloak";
        after = [ "keycloak.service" ];
        requires = [ "keycloak.service" ];
        wantedBy = [ "multi-user.target" ];
        serviceConfig = {
          Type = "oneshot";
          RemainAfterExit = true;
          User = config.systemd.services.keycloak.serviceConfig.User;
          Group = config.systemd.services.keycloak.serviceConfig.Group;
          ExecStart = "${keycloakSetupScript}/bin/keycloak-setup";
        };
      };

      # network

      networking.firewall.enable = true;
      networking.networkmanager.enable = true;
      networking.nameservers = [ "1.1.1.1" "1.0.0.1" ];

      # user

      services.openssh.enable = true;
      services.openssh.settings.PasswordAuthentication = false;

      programs.direnv.enable = true;
      programs.direnv.nix-direnv.enable = true;

      users.mutableUsers = false;
      users.groups.haras = { };
      users.users.haras = {
        group = "haras";
        isNormalUser = true;
        hashedPasswordFile =
          config.sops.secrets.${secretKeys.userHashedPasswordFile}.path;
        extraGroups = [ "wheel" ];
        packages = [
          pkgs.kitty
          pkgs.git
          pkgs.helix
          pkgs.yazi
          pkgs.lazygit
          pkgs.nushell
        ];
      };
      sops.secrets.${secretKeys.userHashedPasswordFile}.neededForUsers = true;

      sops.secrets.${secretKeys.userAuthorizedKeys} = {
        path = "${config.users.users.haras.home}/.ssh/authorized_keys";
        owner = config.users.users.haras.name;
        group = config.users.users.haras.group;
      };

      # service

      services.aideEd.enable = true;
      services.aideEd.environmentFile =
        config.sops.secrets.${secretKeys.aideEdEnv}.path;
      services.aideEd.openFirewall = true;
      services.aideEd.httpPort = 80;
      services.aideEd.httpsPort = 443;
      sops.secrets.${secretKeys.aideEdEnv} = { };
    };
  };
}
