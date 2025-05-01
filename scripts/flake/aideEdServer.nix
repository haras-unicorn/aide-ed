{ self, pkgs, config, lib, unstableNixpkgs, ... }:

{
  seal.defaults.package = "aideEdServer";
  seal.defaults.app = "aideEdServer";
  integrate.package.package =
    let
      unstablePkgs = import unstableNixpkgs {
        system = pkgs.system;
      };
    in
    (self.lib.rust.mkPackage pkgs "web" [ "server" ]).overrideAttrs (final: prev: {
      buildInputs = (prev.buildInputs or [ ]) ++ [
        unstablePkgs.wasm-bindgen-cli_0_2_100
      ];
    });

  seal.defaults.nixosModule = "aideEdServer";
  branch.nixosModule.nixosModule =
    let
      cfg = config.services.aideEd;
      package = self.packages.${pkgs.system}.aideEdServer;
    in
    {
      options.services.aideEd = {
        enable = lib.mkEnableOption "aideEd";

        package = lib.mkOption {
          type = lib.types.package;
          default = package;
          description = "Package to use for the aideEd service.";
        };

        user = lib.mkOption {
          type = lib.types.str;
          description = "User under which the aideEd server runs.";
          default = "aideEd";
        };

        group = lib.mkOption {
          type = lib.types.str;
          description = "Group under which the aideEd server runs.";
          default = "aideEd";
        };

        environment = lib.mkOption {
          type = lib.types.attrsOf (
            lib.types.oneOf [
              lib.types.bool
              lib.types.int
              lib.types.str
            ]
          );
          default = { };
          description = "Extra environment variables passed to aideEd server";
        };

        environmentFile = lib.mkOption {
          type = lib.types.str;
          default = null;
          description = "Path to environment variables file to set for the aideEd service.";
        };

        httpPort = lib.mkOption {
          type = lib.types.port;
          default = 5000;
          description = "HTTP port to listen on";
        };

        httpsPort = lib.mkOption {
          type = lib.types.port;
          default = 5001;
          description = "HTTPS port to listen on";
        };

        openFirewall = lib.mkOption {
          type = lib.types.bool;
          default = false;
          description = "Open ports needed for the aideEd service.";
        };
      };

      config = lib.mkIf cfg.enable {
        environment.systemPackages = [
          self.packages.${pkgs.system}.default
        ];

        users.groups.aideEd = {
          name = cfg.group;
        };
        users.users.aideEd = {
          isSystemUser = true;
          name = cfg.user;
          group = cfg.group;
        };

        networking.firewall.allowedTCPPorts =
          lib.mkIf cfg.openFirewall [
            cfg.httpPort
            # cfg.httpsPort
          ];

        systemd.services.aideEd = {
          description = "aideEd";
          after = [ "network.target" ];
          wantedBy = [ "multi-user.target" ];
          environment =
            lib.mapAttrs
              (_: value:
                if lib.isBool value
                then lib.boolToString value
                else builtins.toString value)
              (cfg.environment // {
                AIDE_ED_HOST = "0.0.0.0";
                AIDE_ED_HTTP_PORT = cfg.httpPort;
                AIDE_ED_HTTPS_PORT = cfg.httpsPort;
              });
          serviceConfig = {
            EnvironmentFile = cfg.environmentFile;
            ExecStart = lib.getExe cfg.package;
            Restart = "always";
            User = cfg.user;
            Group = cfg.group;
            AmbientCapabilities = lib.mkIf
              (cfg.httpPort < 1024 || cfg.httpsPort < 1024)
              [ "CAP_NET_BIND_SERVICE" ];
            UMask = "0077";
          } // lib.optionalAttrs
            (cfg.environmentFile != null)
            {
              EnvironmentFile = cfg.environmentFile;
            } // {
            ProtectSystem = "strict";
            ProtectHome = true;
            PrivateTmp = true;
            NoNewPrivileges = true;
            RestrictSUIDSGID = true;
            ProtectKernelTunables = true;
            ProtectKernelModules = true;
            ProtectControlGroups = true;
            ReadWritePaths = [ "/tmp" ];
            RestrictAddressFamilies = "AF_INET AF_INET6";
            PrivateDevices = true;
            LockPersonality = true;
            RestrictRealtime = true;
            CapabilityBoundingSet = [ "CAP_NET_BIND_SERVICE" ];
            ProtectClock = true;
            ProtectHostname = true;
          };
        };
      };
    };
}
