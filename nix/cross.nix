# Cross-compilation support — builds KwaaiNet binaries for foreign architectures.
#
# Called once per target from flake.nix.  Reuses crane.nix, p2pd.nix, and
# containers.nix unchanged — cross-compilation is handled by the cross pkgs.
{
  nixpkgs,
  crane,
  system,
  targetName, # e.g., "aarch64-linux-gnu"
  crossSystem, # e.g., { config = "aarch64-unknown-linux-gnu"; }
  cargoTarget, # e.g., "aarch64-unknown-linux-gnu"
  protoRs, # host-built protobuf (arch-independent)
}:

let
  pkgsCross = import nixpkgs {
    localSystem = system;
    inherit crossSystem;
    overlays = [ (import ./overlays/cross-fixes.nix) ];
  };

  craneLib = crane.mkLib pkgsCross;

  packages = import ./packages.nix { pkgs = pkgsCross; };

  p2pd = pkgsCross.callPackage ./p2pd.nix { };

  cranePkgs = import ./crane.nix {
    inherit
      craneLib
      p2pd
      protoRs
      packages
      cargoTarget
      ;
    inherit (pkgsCross) lib makeWrapper;
    # fetchurl/runCommand for the patched multistream-select source; the
    # native package set is correct here — it is build-time-only source prep.
    pkgs = pkgsCross;
  };

  containers = import ./containers.nix {
    pkgs = pkgsCross;
    inherit (cranePkgs) kwaainet map-server;
  };
in
{
  inherit (cranePkgs)
    kwaainet
    map-server
    cargoArtifacts
    ;
  inherit p2pd;
  inherit (containers) kwaainet-container map-server-container kwaainet-all-container;
}
