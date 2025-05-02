{ self, pkgs, ... }:

let
  package = self.packages.${pkgs.system}.aideEdServer;
in
{
  integrate.package.nixpkgs.overlays = self.lib.rust.overlays;
  integrate.package.package = pkgs.dockerTools.buildImage {
    name = "aide-ed";
    tag = "latest";
    created = "now";
    copyToRoot = pkgs.buildEnv {
      name = "image-root";
      paths = [ package ];
      pathsToLink = [ "/bin" ];
    };
    config = {
      Cmd = [ "aide-ed-server" ];
    };
  };
}
