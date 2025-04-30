{ self, pkgs, ... }:

{
  integrate.package.package = (self.lib.rust.mkPackage pkgs "mobile").overrideAttrs (final: prev: {
    buildInputs = (prev.buildInputs or [ ]) ++ [
      pkgs.webkitgtk_4_1
    ];
  });
}
