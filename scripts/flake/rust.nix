{ self, naersk, ... }:

let
  mkNaerskLib = pkgs: pkgs.callPackage naersk { };
in
{
  flake.lib.rust.mkPackage = pkgs: crate:
    let
      naerskLib = mkNaerskLib pkgs;
    in
    naerskLib.buildPackage {
      name = "aide-ed-${crate}";
      pname = "aide-ed-${crate}";
      version = "0.1.0";
      src = self;

      nativeBuildInputs = [
        pkgs.pkg-config
      ];

      buildInputs = [
        pkgs.openssl
      ];
    };

  flake.lib.rust.mkDevShell = pkgs:
    pkgs.mkShell {
      shellHook = ''
        export RUST_BACKTRACE="full";
      '';

      buildInputs = [
        pkgs.pkg-config
        pkgs.openssl
        pkgs.webkitgtk_4_1
      ];

      packages = with pkgs; [
        llvmPackages.clangNoLibcxx
        lldb
        rustc
        cargo
        clippy
        rustfmt
        rust-analyzer
        cargo-edit
        evcxr
      ];
    };
}

