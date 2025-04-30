{ self, pkgs, ... }:

{
  integrate.package.package = (self.lib.rust.mkPackage pkgs "desktop").overrideAttrs (final: prev: {
    buildInputs = (prev.buildInputs or [ ]) ++ [
      pkgs.webkitgtk_4_1
    ];
  });
}
