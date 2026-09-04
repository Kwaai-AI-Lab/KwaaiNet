# Test orchestration — exposes checks (sandboxed) and runnable test scripts.
{
  pkgs,
  kwaainet,
  containers ? { },
  crossTests ? { },
}:

let
  lib = pkgs.lib;
  smoke = import ./smoke.nix { inherit pkgs kwaainet; };
  twoNode = import ./two-node.nix { inherit pkgs kwaainet; };
  containerTest = import ./containers.nix { inherit pkgs containers; };
in
{
  # Sandboxed checks — run via `nix flake check`
  checks = {
    kwaainet-smoke = smoke;
  };

  # Runnable test scripts — run via `nix run .#test-<name>`
  packages = {
    test-two-node = twoNode;
  }
  // lib.optionalAttrs (containers != { }) {
    test-containers = containerTest;
  }
  // crossTests;
}
