{ self, pkgs, ... }:

{
  integrate.package.nixpkgs.overlays = self.lib.rust.overlays;
  integrate.package.package = (self.lib.rust.mkPackage pkgs "mobile" [ "mobile" ]);
}
