# Two-phase Rust build using crane.
#
# Phase 1 (buildDepsOnly): compile all external dependencies — cached until
#   Cargo.lock changes.  No cargoHash needed; crane reads Cargo.lock directly.
# Phase 2 (buildPackage): compile workspace source against cached deps.
#
# Each workspace binary is a separate derivation so changes to one don't
# rebuild the others.
{
  lib,
  craneLib,
  p2pd,
  protoRs,
  packages,
  makeWrapper,
  pkgs,
  cargoTarget ? null, # e.g., "aarch64-unknown-linux-gnu" — null for native builds
}:

let
  # Read version from Cargo.toml so it stays in sync automatically.
  cargoToml = builtins.fromTOML (builtins.readFile (./.. + "/core/Cargo.toml"));
  version = cargoToml.package.version;

  # multistream-select with the slash-less protocol-ID patch. The working-tree
  # copy is produced by core/patches/fetch-multistream-select.sh and is
  # gitignored, so the flake materializes it the same way: pristine crates.io
  # tarball (same pinned sha256 as the script) plus the tracked patch file.
  # See core/patches/README.md.
  multistreamSelectPatched =
    pkgs.runCommand "multistream-select-0.13.0-slashless"
      {
        # tar/gzip/patch all come with stdenv's default build utilities.
        crate = pkgs.fetchurl {
          url = "https://static.crates.io/crates/multistream-select/multistream-select-0.13.0.crate";
          sha256 = "ea0df8e5eec2298a62b326ee4f0d7fe1a6b90a09dfcf9df37b38f947a8c42f19";
        };
      }
      ''
        tar -xzf "$crate"
        mv multistream-select-0.13.0 $out
        cd $out
        patch -p1 < ${./.. + "/core/patches/multistream-select.patch"}
      '';

  # Source filter: keep .rs, .toml, .lock, .proto, and non-code assets
  # that are embedded at compile time via include_str!() (.html, .sql).
  filteredSrc =
    let
      extraFilter = path: _type: builtins.match ".*\\.(proto|html|sql)$" path != null;
      sourceFilter = path: type: (extraFilter path type) || (craneLib.filterCargoSources path type);
    in
    lib.cleanSourceWith {
      src = craneLib.path (./.. + "/core");
      filter = sourceFilter;
    };

  # The patched crate must live inside the source derivation itself (not be
  # copied in by a build hook): crane's deps-only phase parses the manifest
  # from a dummified copy of `src`, and `[patch.crates-io]` needs the path
  # present there too.
  src = pkgs.runCommand "kwaainet-src-with-patches" { } ''
    cp -r ${filteredSrc} $out
    chmod -R u+w $out
    mkdir -p $out/patches
    rm -rf $out/patches/multistream-select
    cp -r ${multistreamSelectPatched} $out/patches/multistream-select
  '';

  commonArgs = {
    inherit src version;
    pname = "kwaainet";

    strictDeps = true;

    nativeBuildInputs = packages.nativeBuildInputs ++ [ makeWrapper ];
    inherit (packages) buildInputs;

    # Environment variables consumed by the patched build.rs.
    P2PD_BIN = "${p2pd}/bin/p2pd";
    P2PD_PROTO_RS = "${protoRs}/p2pd.pb.rs";

    # Replace build.rs: skip Go clone/build AND protoc/prost_build.
    postPatch = ''
      cat > crates/kwaai-p2p-daemon/build.rs << 'BUILDRS'
      fn main() {
          println!("cargo:rerun-if-changed=proto/p2pd.proto");

          // Copy pre-generated protobuf Rust code into OUT_DIR.
          let out_dir = std::env::var("OUT_DIR").unwrap();
          let pre_gen = std::env::var("P2PD_PROTO_RS")
              .expect("P2PD_PROTO_RS must point to pre-generated p2pd.pb.rs");
          std::fs::copy(&pre_gen, std::path::Path::new(&out_dir).join("p2pd.pb.rs"))
              .expect("failed to copy pre-generated p2pd.pb.rs");

          // p2pd is provided by Nix — bake the store path into the binary.
          let p2pd_bin = std::env::var("P2PD_BIN")
              .unwrap_or_else(|_| "p2pd".to_string());
          println!("cargo:rustc-env=P2PD_PATH={}", p2pd_bin);
          println!("cargo:rustc-env=P2PD_REPO=nix-provided");
      }
      BUILDRS
    '';
  }
  // lib.optionalAttrs (cargoTarget != null) {
    CARGO_BUILD_TARGET = cargoTarget;
    HOST_CC = "cc"; # ensure build scripts use host compiler
  };

  # Phase 1: compile all workspace dependencies.
  cargoArtifacts = craneLib.buildDepsOnly (
    commonArgs
    // {
      cargoExtraArgs = "--workspace";
    }
  );

  # Helper to build a single binary from the workspace.
  mkBin =
    pname: extra:
    craneLib.buildPackage (
      commonArgs
      // {
        inherit pname cargoArtifacts;
        cargoExtraArgs = "-p ${pname}";
        doCheck = false; # tests run separately below
      }
      // extra
    );

in
{
  inherit cargoArtifacts;

  kwaainet = mkBin "kwaainet" {
    postInstall = ''
      # Bundle p2pd next to kwaainet so find_p2pd_binary() finds it.
      ln -sf ${p2pd}/bin/p2pd $out/bin/p2pd
    '';
  };

  # Clippy lint check — run via `nix flake check`.
  clippy = craneLib.cargoClippy (
    commonArgs
    // {
      inherit cargoArtifacts;
      cargoClippyExtraArgs = "--workspace -- --deny warnings";
    }
  );

  # Cargo test check — run via `nix flake check`.
  cargoTest = craneLib.cargoTest (
    commonArgs
    // {
      inherit cargoArtifacts;
      cargoTestExtraArgs = "--workspace";
    }
  );
}
