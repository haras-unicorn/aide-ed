{ rumor, unstableNixpkgs, ... }:

# TODO: pin dioxus-cli to the same version as in Cargo.lock

{
  flake.lib.tools.mkDevShell = pkgs:
    let
      unstablePkgs = import unstableNixpkgs {
        system = pkgs.system;
      };
    in
    pkgs.mkShell {
      packages = with pkgs; [
        # client
        unstablePkgs.dioxus-cli

        # postgres
        usql
        postgresql_17
        diesel-cli

        # keycloak
        keycloak

        # ollama
        ollama

        # documentation
        mdbook
        mdbook-plantuml
        plantuml
        openjdk
        simple-http-server
        pandoc
        pandoc-plantuml-filter

        # rpi
        s3cmd
        rumor.packages.${pkgs.system}.default
        vault
        vault-medusa
        zstd
        nixos-generators
        deploy-rs
        sshpass
      ] ++ lib.optionals
        (
          pkgs.stdenv.hostPlatform.isLinux
            && pkgs.stdenv.hostPlatform.isx86_64
        ) [
        libguestfs-with-appliance
      ];
    };
}
