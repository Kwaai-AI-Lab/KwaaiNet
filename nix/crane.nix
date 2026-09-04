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

  # The patched dependencies, materialized the way core/patches/*.sh do it:
  # pristine crates.io tarball (same pinned sha256 as each script) plus the
  # tracked patch file. Running the scripts is not an option here — their
  # output is gitignored, and a flake build reads the git tree, not the
  # working copy. See core/patches/README.md.
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

  # libp2p-kad with `set_protocol_names` restored, for the kad protocol
  # migration. Same shape as multistream-select above; sha256 matches
  # core/patches/fetch-libp2p-kad.sh.
  libp2pKadPatched =
    pkgs.runCommand "libp2p-kad-0.48.0-multi-protocol"
      {
        crate = pkgs.fetchurl {
          url = "https://static.crates.io/crates/libp2p-kad/libp2p-kad-0.48.0.crate";
          sha256 = "13d3fd632a5872ec804d37e7413ceea20588f69d027a0fa3c46f82574f4dee60";
        };
      }
      ''
        tar -xzf "$crate"
        mv libp2p-kad-0.48.0 $out
        cd $out
        patch -p1 < ${./.. + "/core/patches/libp2p-kad.patch"}
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
    rm -rf $out/patches/libp2p-kad
    cp -r ${libp2pKadPatched} $out/patches/libp2p-kad
  '';

  commonArgs = {
    inherit src version;
    pname = "kwaainet";

    strictDeps = true;

    nativeBuildInputs = packages.nativeBuildInputs ++ [ makeWrapper ];
    inherit (packages) buildInputs;

    # Environment variables consumed by the patched build.rs.
    P2PD_PROTO_RS = "${protoRs}/p2pd.pb.rs";

    # Replace build.rs: skip protoc/prost_build, use the pre-generated code.
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

  kwaainet = mkBin "kwaainet" { };

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
