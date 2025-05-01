{ self, ... }:

{
  flake.lib.wasmBindgenCli = pkgs:
    let
      cargoLock = builtins.fromTOML
        (builtins.readFile "${self}/Cargo.lock");

      wasmBindgen = pkgs.lib.findFirst
        (pkg: pkg.name == "wasm-bindgen")
        (throw "Could not find wasm-bindgen package")
        cargoLock.package;

      wasm-bindgen-cli = pkgs.wasm-bindgen-cli.override {
        version = wasmBindgen.version;
        hash = pkgs.lib.fakeHash;
        cargoHash = pkgs.lib.fakeHash;
      };
    in
    wasm-bindgen-cli;
}
