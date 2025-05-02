{ self, pkgs, ... }:

{
  integrate.devShell.nixpkgs.overlays = self.lib.rust.overlays;
  integrate.devShell.devShell = pkgs.mkShell {
    inputsFrom = [
      (self.lib.vcs.mkDevShell pkgs)
      (self.lib.scripts.mkDevShell pkgs)
      (self.lib.rust.mkDevShell pkgs)
    ];

    packages = with pkgs; [
      mdbook
      mdbook-plantuml
      plantuml
      openjdk
    ];
  };
}

