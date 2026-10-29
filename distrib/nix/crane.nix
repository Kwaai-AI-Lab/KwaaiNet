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
  cudaFeature ? false, # build kwaainet with `--features cuda` against `pkgs.cudaPackages`
  # Directory holding the *host's* real libcuda.so.1 (the NVIDIA driver's
  # userspace library, installed outside Nix by the system package manager or
  # the driver .run installer) — e.g. "/usr/lib64" on RHEL/Fedora,
  # "/usr/lib/x86_64-linux-gnu" on Debian/Ubuntu. Required with cudaFeature:
  # without it the binary resolves cudatoolkit's linker stub at runtime and
  # every CUDA call fails with CUDA_ERROR_STUB_LIBRARY. NixOS
  # (/run/opengl-driver/lib) isn't wired up. See the postFixup comment below.
  cudaDriverLibPath ? null,
  # GPU architecture the kernels are compiled for (86 = Ampere, e.g. RTX
  # A5000). Must be set explicitly: the build sandbox has no GPU to detect.
  cudaComputeCap ? "86",
  # Parallel nvcc/cicc processes for the CUDA kernel crates. Each takes ~3 GB,
  # and uncapped they scale with core count — on a 72-core host that ran the
  # machine out of memory.
  cudaBuildThreads ? 8,
}:

assert lib.assertMsg (
  cudaFeature -> cudaDriverLibPath != null
) "crane.nix: cudaFeature requires cudaDriverLibPath (the host's NVIDIA driver library directory)";

let
  # Read version from Cargo.toml so it stays in sync automatically.
  cargoToml = builtins.fromTOML (builtins.readFile (./../.. + "/core/Cargo.toml"));
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
        patch -p1 < ${./../.. + "/core/patches/multistream-select.patch"}
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
        patch -p1 < ${./../.. + "/core/patches/libp2p-kad.patch"}
      '';

  # cudarc with the CUDARC_DISABLE_ASYNC_ALLOC opt-out (Jetson). Only the CUDA
  # features pull it in, but `[patch.crates-io]` must resolve for every build.
  # sha256 matches core/patches/fetch-cudarc.sh.
  cudarcPatched =
    pkgs.runCommand "cudarc-0.19.7-sync-alloc-opt-out"
      {
        crate = pkgs.fetchurl {
          url = "https://static.crates.io/crates/cudarc/cudarc-0.19.7.crate";
          sha256 = "1cea5f10a99e025c1b44ae2354c2d8326b25ddbd0baf76bde8e55cfd4018a2cc";
        };
      }
      ''
        tar -xzf "$crate"
        mv cudarc-0.19.7 $out
        cd $out
        patch -p1 < ${./../.. + "/core/patches/cudarc.patch"}
      '';

  # Source filter: keep .rs, .toml, .lock, .proto, and non-code assets
  # that are embedded at compile time via include_str!() (.html, .sql, .yaml).
  filteredSrc =
    let
      extraFilter = path: _type: builtins.match ".*\\.(proto|html|sql|yaml)$" path != null;
      sourceFilter = path: type: (extraFilter path type) || (craneLib.filterCargoSources path type);
    in
    lib.cleanSourceWith {
      src = craneLib.path (./../.. + "/core");
      filter = sourceFilter;
    };

  # Drops the three patched crates into $out/patches/*, overwriting whatever
  # (if anything) is already there. Shared between the real `src` derivation
  # below and `extraDummyScript` (see commonArgs): crane's `mkDummySrc` has no
  # concept of `[workspace] exclude` — it discovers every Cargo.toml reachable
  # from `src`, including these excluded-but-still-a-crate patch directories,
  # and replaces each one's real source with a generic placeholder, same as
  # it does for actual workspace members. That silently truncated
  # multistream-select's ~300-line lib.rs (real exports: Version,
  # NegotiationError, etc.) down to a 14-line stub during buildDepsOnly,
  # which is why libp2p-core failed to compile against it with unresolved
  # imports (https://github.com/Kwaai-AI-Lab/KwaaiNet/issues/237). Re-running
  # this after crane's own dummy generation restores the real patched source
  # for exactly these three crates without disabling dummying for anything
  # crane got right (i.e. the actual workspace members).
  restorePatchedDeps = ''
    mkdir -p $out/patches
    rm -rf $out/patches/multistream-select
    cp -r ${multistreamSelectPatched} $out/patches/multistream-select
    rm -rf $out/patches/libp2p-kad
    cp -r ${libp2pKadPatched} $out/patches/libp2p-kad
    rm -rf $out/patches/cudarc
    cp -r ${cudarcPatched} $out/patches/cudarc
  '';

  # The patched crate must live inside the source derivation itself (not be
  # copied in by a build hook): crane's deps-only phase parses the manifest
  # from a dummified copy of `src`, and `[patch.crates-io]` needs the path
  # present there too.
  src = pkgs.runCommand "kwaainet-src-with-patches" { } ''
    cp -r ${filteredSrc} $out
    chmod -R u+w $out
    ${restorePatchedDeps}
  '';

  # candle-core's "cuda" feature (pulled in transitively by kwaainet's own
  # `cuda` feature) needs nvcc at build time and libcudart/libcublas/libcurand
  # at both build and run time. `pkgs` is expected to already be a
  # cudaSupport-enabled instance (config.cudaSupport = true;
  # config.allowUnfree = true;) when cudaFeature is set — see flake.nix.
  cudaBuildInputs = lib.optionals cudaFeature (
    [ pkgs.git ] # cudaforge (candle-flash-attn's build helper) shells out to git
    ++ (with pkgs.cudaPackages; [
      cudatoolkit
      libcublas
      libcurand
      cuda_nvrtc
    ])
  );

  # candle-flash-attn's build.rs (via the `cudaforge` crate) shells out to
  # `git clone https://github.com/NVIDIA/cutlass.git`, which the Nix sandbox
  # blocks (no network during the build phase). `cudaforge` checks a cache
  # dir keyed by commit prefix *before* cloning (src/dependency.rs,
  # `fetch_with_lock`) and honours `$CUDAFORGE_HOME` as an override for where
  # that cache lives — pre-seeding it here with a Nix-fetched checkout at the
  # exact pinned commit (network access is fine in fetchgit's own fixed-output
  # derivation) means the real build never needs to reach the network.
  cutlassCommit = "7d49e6c7e2f8896c47f586706e67e1fb215529dc";
  cutlassSrc = pkgs.fetchgit {
    url = "https://github.com/NVIDIA/cutlass.git";
    rev = cutlassCommit;
    leaveDotGit = true;
    fetchSubmodules = true;
    sha256 = "sha256-cSWVzyuDC8EidTAZzHbVz0jUNK4zx5AAwfUV6lUXTXs=";
  };

  commonArgs = {
    inherit src version;
    pname = "kwaainet";

    strictDeps = true;

    nativeBuildInputs = packages.nativeBuildInputs ++ [ makeWrapper ] ++ cudaBuildInputs;
    buildInputs = packages.buildInputs ++ cudaBuildInputs;

    # Runs after crane's own `mkDummySrc` stubbing (buildDepsOnly and
    # anything else that dummies `src`) — see restorePatchedDeps above.
    extraDummyScript = restorePatchedDeps;

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
  }
  // lib.optionalAttrs cudaFeature {
    # cudarc's build.rs (core/patches/cudarc) looks for the toolkit under any
    # of these; cudatoolkit is nixpkgs' traditional bin/include/lib64 layout.
    CUDA_PATH = "${pkgs.cudaPackages.cudatoolkit}";
    CUDA_HOME = "${pkgs.cudaPackages.cudatoolkit}";
    CUDA_ROOT = "${pkgs.cudaPackages.cudatoolkit}";
    # nvcc needs a host C++ compiler it actually supports — same reasoning as
    # release.yml's manylinux CUDA job (nvcc 12.x doesn't support the very
    # latest gcc). Point it at cudaPackages' own known-compatible compiler
    # rather than swapping the whole stdenv (which self-recurses: cudaPackages
    # is itself built with the ordinary stdenv).
    NVCC_CCBIN = "${pkgs.cudaPackages.backendStdenv.cc}/bin/cc";
    CC = "${pkgs.cudaPackages.backendStdenv.cc}/bin/cc";
    CXX = "${pkgs.cudaPackages.backendStdenv.cc}/bin/c++";

    # candle-kernels' build.rs shells out to nvidia-smi to detect the target
    # GPU's compute capability; there's no GPU (and no nvidia-smi) in the
    # build sandbox by design, so it must be given explicitly.
    CUDA_COMPUTE_CAP = cudaComputeCap;

    # cudaforge (candle-flash-attn's build helper) reads CUDAFORGE_THREADS,
    # then RAYON_NUM_THREADS, and otherwise uses half the host's cores.
    CUDAFORGE_THREADS = toString cudaBuildThreads;
    RAYON_NUM_THREADS = toString cudaBuildThreads;

    preBuild = ''
      export CUDAFORGE_HOME="$TMPDIR/cudaforge-home"
      cutlass_cache="$CUDAFORGE_HOME/git/checkouts/cutlass-${builtins.substring 0 16 cutlassCommit}"
      mkdir -p "$cutlass_cache"
      cp -r --no-preserve=mode,ownership ${cutlassSrc}/. "$cutlass_cache"
      chmod -R u+w "$cutlass_cache"
    '';
  };

  # Phase 1: compile all workspace dependencies.
  cargoArtifacts = craneLib.buildDepsOnly (
    commonArgs
    // {
      # `--features kwaainet/cuda`, not `--features cuda`: cargo requires the
      # pkg/feature form to target one workspace member's feature under
      # `--workspace`, since `cuda` isn't a feature of every crate being built.
      cargoExtraArgs = if cudaFeature then "--workspace --features kwaainet/cuda" else "--workspace";
    }
  );

  # Helper to build a single binary from the workspace.
  mkBin =
    pname: extra:
    craneLib.buildPackage (
      commonArgs
      // {
        inherit pname cargoArtifacts;
        cargoExtraArgs = if cudaFeature then "-p ${pname} --features cuda" else "-p ${pname}";
        doCheck = false; # tests run separately below
      }
      // lib.optionalAttrs cudaFeature {
        # `cudatoolkit` only carries a linker *stub* libcuda.so.1 (there's no
        # real driver in the Nix build sandbox), which lands on the binary's
        # RUNPATH — so without this, the binary loads the stub at runtime and
        # every CUDA call fails with CUDA_ERROR_STUB_LIBRARY. On NixOS,
        # nixpkgs papers over this with `/run/opengl-driver/lib`, populated
        # by the OS from the host driver; off NixOS that path doesn't exist,
        # and — because this is Nix's own dynamic loader, not the host's —
        # there's no ld.so.cache fallback to catch it either.
        #
        # Prepending `cudaDriverLibPath` itself (e.g. `/usr/lib64`) to the
        # RUNPATH is *not* safe: that directory also holds the host's own
        # libc/libpthread/librt, and those shadow the Nix-built glibc this
        # binary actually needs (GLIBC_2.39/2.36/GLIBC_ABI_DT_RELR not found).
        # So symlink only the driver libraries we actually need into a
        # private shim dir and point the RUNPATH at that instead — narrow
        # enough that nothing but these ever resolves from it:
        #   - libcuda.so.1: the driver itself.
        #   - libnvidia-ml.so.1: NVML, in case it's dlopened for device
        #     queries.
        #   - libnvidia-ptxjitcompiler.so.1: candle/cudarc kernels are
        #     compiled to PTX and JIT'd by the driver at load time; without
        #     this, that JIT step fails with CUDA_ERROR_JIT_COMPILER_NOT_FOUND
        #     even though libcuda.so.1 itself loaded fine.
        # Note: the build itself runs sandboxed with no access to
        # `cudaDriverLibPath` (that's the host's filesystem, not the Nix
        # store), so this can't check the target exists — `ln -s` doesn't
        # need to, it just records the path string, and it's only resolved
        # later when the binary actually runs outside the sandbox.
        # RUNPATH only covers the *executable's own* direct dependencies —
        # libcuda.so.1 itself, found via the patchelf step below. It does
        # nothing for libnvidia-ptxjitcompiler.so.1, which libcuda.so.1
        # dlopen's on its own account at PTX JIT time: since the real
        # (unmodified, system-installed) libcuda.so.1 carries no RUNPATH of
        # its own, that nested lookup never sees ours. So make it a direct
        # dependency of the executable (--add-needed): it then loads at
        # startup through our RUNPATH, and libcuda's later dlopen matches
        # the already-loaded SONAME. Not a wrapper setting LD_LIBRARY_PATH:
        # `kwaainet service install` writes current_exe() — the unwrapped
        # binary — into the systemd unit, which would bypass it.
        postFixup = ''
          mkdir -p $out/libexec/cuda-driver-shim
          for lib in libcuda.so.1 libnvidia-ml.so.1 libnvidia-ptxjitcompiler.so.1; do
            ln -s "${cudaDriverLibPath}/$lib" "$out/libexec/cuda-driver-shim/$lib"
          done
          origRpath=$(patchelf --print-rpath $out/bin/${pname})
          patchelf --set-rpath "$out/libexec/cuda-driver-shim:$origRpath" $out/bin/${pname}
          patchelf --add-needed libnvidia-ptxjitcompiler.so.1 $out/bin/${pname}
        '';
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
