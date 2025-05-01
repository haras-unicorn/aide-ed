{ self, naersk, lib, ... }:

let
  mkNaerskLib = pkgs: pkgs.callPackage naersk { };

  nativeBuildInputs = pkgs: [
    pkgs.pkg-config
    pkgs.lld
  ];

  buildInputs = pkgs: [
    pkgs.openssl
    pkgs.libiconv
    pkgs.pkg-config
  ] ++ lib.optionals
    pkgs.stdenv.isLinux
    [
      pkgs.glib
      pkgs.gtk3
      pkgs.libsoup_3
      pkgs.webkitgtk_4_1
      pkgs.xdotool
    ] ++ lib.optionals
    pkgs.stdenv.isDarwin
    (with pkgs.darwin.apple_sdk.frameworks; [
      IOKit
      Carbon
      WebKit
      Security
      Cocoa
    ]);
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

      nativeBuildInputs = nativeBuildInputs pkgs;

      buildInputs = buildInputs pkgs;

      cargoBuildOptions = prev: prev
        ++ [ "--features" ]
        ++ features;
    };

  flake.lib.rust.mkDevShell = pkgs:
    pkgs.mkShell {
      shellHook = ''
        export RUST_BACKTRACE="full";
      '';

      nativeBuildInputs = nativeBuildInputs pkgs;

      buildInputs = buildInputs pkgs;

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
        (self.lib.wasmBindgenCli pkgs)
      ];
    };
}

