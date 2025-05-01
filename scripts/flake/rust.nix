{ self, naersk, lib, ... }:

let
  mkNaerskLib = pkgs: pkgs.callPackage naersk { };
in
{
  flake.lib.rust.mkPackage = pkgs: crate: features:
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
        pkgs.libiconv
        pkgs.pkg-config
      ] ++ lib.optionals pkgs.stdenv.isLinux [
        pkgs.glib
        pkgs.gtk3
        pkgs.libsoup_3
        pkgs.webkitgtk_4_1
        pkgs.xdotool
      ] ++ lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [
        IOKit
        Carbon
        WebKit
        Security
        Cocoa
      ]);

      cargoBuildOptions = prev: prev
        ++ [ "--features" ]
        ++ features;
    };

  flake.lib.rust.mkDevShell = pkgs:
    pkgs.mkShell {
      shellHook = ''
        export RUST_BACKTRACE="full";
      '';

      buildInputs = [
        pkgs.openssl
        pkgs.libiconv
        pkgs.pkg-config
      ] ++ lib.optionals pkgs.stdenv.isLinux [
        pkgs.glib
        pkgs.gtk3
        pkgs.libsoup_3
        pkgs.webkitgtk_4_1
        pkgs.xdotool
      ] ++ lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [
        IOKit
        Carbon
        WebKit
        Security
        Cocoa
      ]);

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

