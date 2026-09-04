{
  description = "KwaaiNet — Sovereign AI Infrastructure";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      crane,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        lib = pkgs.lib;
        craneLib = crane.mkLib pkgs;
        packages = import ./distrib/nix/packages.nix { inherit pkgs; };
        p2pd = pkgs.callPackage ./distrib/nix/p2pd.nix { };
        protoRs = pkgs.callPackage ./distrib/nix/proto.nix { };
        cranePkgs = import ./distrib/nix/crane.nix {
          inherit
            craneLib
            p2pd
            protoRs
            packages
            pkgs
            ;
          inherit (pkgs) lib makeWrapper;
        };
        containers = lib.optionalAttrs pkgs.stdenv.hostPlatform.isLinux (
          import ./distrib/nix/containers.nix {
            inherit pkgs;
            inherit (cranePkgs) kwaainet;
          }
        );

        # --- Cross-compilation (x86_64-linux only) ---
        crossTargets = lib.optionalAttrs (system == "x86_64-linux") {
          aarch64-linux-gnu = import ./distrib/nix/cross.nix {
            inherit
              nixpkgs
              crane
              system
              protoRs
              ;
            targetName = "aarch64-linux-gnu";
            crossSystem = {
              config = "aarch64-unknown-linux-gnu";
            };
            cargoTarget = "aarch64-unknown-linux-gnu";
          };
          aarch64-linux-musl = import ./distrib/nix/cross.nix {
            inherit
              nixpkgs
              crane
              system
              protoRs
              ;
            targetName = "aarch64-linux-musl";
            crossSystem = {
              config = "aarch64-unknown-linux-musl";
            };
            cargoTarget = "aarch64-unknown-linux-musl";
          };
          x86_64-linux-musl = import ./distrib/nix/cross.nix {
            inherit
              nixpkgs
              crane
              system
              protoRs
              ;
            targetName = "x86_64-linux-musl";
            crossSystem = {
              config = "x86_64-unknown-linux-musl";
            };
            cargoTarget = "x86_64-unknown-linux-musl";
          };
          riscv64-linux-gnu = import ./distrib/nix/cross.nix {
            inherit
              nixpkgs
              crane
              system
              protoRs
              ;
            targetName = "riscv64-linux-gnu";
            crossSystem = {
              config = "riscv64-unknown-linux-gnu";
            };
            cargoTarget = "riscv64gc-unknown-linux-gnu";
          };
        };

        # Flatten cross targets into suffixed package names.
        crossPackages = lib.concatMapAttrs (targetName: cross: {
          "kwaainet-${targetName}" = cross.kwaainet;
          "p2pd-${targetName}" = cross.p2pd;
          "kwaainet-container-${targetName}" = cross.kwaainet-container;
          "kwaainet-all-container-${targetName}" = cross.kwaainet-all-container;
        }) crossTargets;

        # Cross smoke tests — verify cross-compiled binaries run under QEMU.
        crossTests = lib.concatMapAttrs (
          targetName: cross:
          let
            parts = lib.splitString "-" targetName;
            arch = builtins.head parts;
            isMusl = lib.hasSuffix "musl" targetName;
          in
          {
            "test-cross-smoke-${targetName}" = import ./distrib/nix/tests/cross-smoke.nix {
              inherit pkgs arch;
              kwaainet = cross.kwaainet;
              isStatic = isMusl;
            };
          }
        ) crossTargets;

        tests = import ./distrib/nix/tests {
          inherit pkgs containers crossTests;
          kwaainet = cranePkgs.kwaainet;
        };
      in
      {
        packages = {
          default = cranePkgs.kwaainet;
          inherit (cranePkgs)
            kwaainet
            cargoArtifacts
            ;
          inherit p2pd protoRs;
        }
        // containers
        // crossPackages
        // tests.packages;

        devShells.default = import ./distrib/nix/devshell.nix { inherit pkgs packages; };

        checks = {
          inherit (cranePkgs) clippy cargoTest;
        }
        // tests.checks;

        formatter = pkgs.nixfmt;
      }
    );
}
