# Nix support for KwaaiNet

Nix provides reproducible builds, a development shell with all dependencies
pinned, and automated tests — all from a single `flake.nix`.

## Goals

The goals of using Nix in this repository are to:
- **Simplify onboarding** — make it easy to get started with KwaaiNet on any
  Linux or macOS system
- **Improve reproducibility** — ensure consistent build environments across
  developers (no more "it worked on my machine")
- **Reduce setup friction** — eliminate dependency conflicts and version
  mismatches; a single `nix build` or `nix develop` is all you need

Feedback and pull requests are welcome.  If we're missing a tool, please open
an issue or PR.  See `nix/packages.nix` for package definitions.

---

## Getting started

### 1. Install Nix

Choose **multi-user** (daemon) or **single-user**:

- **Multi-user install** (recommended on most distros):
  ```bash
  bash <(curl -L https://nixos.org/nix/install) --daemon
  ```

- **Single-user install**:
  ```bash
  bash <(curl -L https://nixos.org/nix/install) --no-daemon
  ```

See also: [Nix installation manual](https://nix.dev/manual/nix/2.24/installation/)

#### Video tutorials

| Platform | Video |
|----------|-------|
| Ubuntu | [Installing Nix on Ubuntu](https://youtu.be/cb7BBZLhuUY) |
| Fedora | [Installing Nix on Fedora](https://youtu.be/RvaTxMa4IiY) |

### 2. Enable flakes (if needed)

Nix flakes are an opt-in feature.  If you haven't enabled them yet:

**Option A — one-time flag** (no config change):
```bash
nix --extra-experimental-features 'nix-command flakes' develop .
```

**Option B — permanent** (recommended):
```bash
test -d /etc/nix || sudo mkdir /etc/nix
echo 'experimental-features = nix-command flakes' | sudo tee -a /etc/nix/nix.conf
```

After this, all `nix build`, `nix develop`, and `nix flake` commands work
without extra flags.

See also: [Nix Flakes Wiki](https://nixos.wiki/wiki/flakes)

### 3. Build or enter the dev shell

```bash
# Build kwaainet (produces ./result/bin/kwaainet)
nix build

# Or enter a development shell with Rust, Go, protobuf, and formatters
nix develop
```

### 4. First run considerations

On first execution, Nix will download and build all dependencies — this can
take several minutes depending on your internet speed.  On subsequent runs Nix
reuses its cache in `/nix/store/` and startup is essentially instantaneous.

Nix will **not** interact with any system packages you already have installed.
The Nix versions are isolated and effectively "disappear" when you exit the
development shell.

---

## Quick reference

By default, `nix build` writes every output to a single `./result` symlink —
building a different package overwrites the previous one.  A root `Makefile`
solves this by passing `-o result-<name>` to each build, giving every package
its own symlink (`result-kwaainet`, `result-p2pd`, etc.) so multiple
outputs coexist.  Use `make -j` for parallel builds.

### Binaries

| Make target | Nix equivalent | Output |
|-------------|---------------|--------|
| `make` | build the kwaainet binary | `result-kwaainet` |
| `make kwaainet` | `nix build .#kwaainet` | `result-kwaainet` |
| `make p2pd` | `nix build .#p2pd` | `result-p2pd` |
| `make proto` | `nix build .#protoRs` | `result-proto` |

### OCI containers (Linux only)

| Make target | Nix equivalent | Output |
|-------------|---------------|--------|
| `make containers` | build all containers | `result-*-container` |
| `make kwaainet-container` | `nix build .#kwaainet-container` | `result-kwaainet-container` |
| `make kwaainet-all-container` | `nix build .#kwaainet-all-container` | `result-kwaainet-all-container` |

> **Note:** Container images use `dockerTools.streamLayeredImage` which is
> Linux-only.  On macOS (Darwin), container packages are not available.

### Tests, checks, and utilities

| Make target | Nix equivalent | Description |
|-------------|---------------|-------------|
| `make check` | `nix flake check` | smoke test + clippy + cargo test |
| `make test` | `nix run .#test-two-node` | two-node integration test |
| `make test-containers` | `nix run .#test-containers` | container image test suite |
| `make fmt` | `nix fmt` | format all Nix files |
| `make develop` | `nix develop` | enter development shell |
| `make clean` | — | remove all result symlinks |

### Nix-only targets

These have no Makefile wrapper — call `nix` directly:

| Command | Description |
|---------|-------------|
| `nix build .#cargoArtifacts` | Build only workspace dependencies (crane cache layer) |

## Outputs

```
packages.default                kwaainet CLI + bundled p2pd
packages.kwaainet               (same as default)
packages.cargoArtifacts         cached workspace dependency artifacts (crane)
packages.p2pd                   go-libp2p-daemon (Hivemind fork)
packages.protoRs                pre-generated prost Rust code from p2pd.proto
packages.kwaainet-container     OCI image stream script — kwaainet + p2pd (Linux only)
packages.kwaainet-all-container OCI image — all binaries in one image (Linux only)
packages.test-two-node          two-node integration test script
packages.test-containers        container image test suite (Linux only)

# Cross-compiled packages (x86_64-linux only):
packages.kwaainet-aarch64-linux-gnu         ARM64 dynamic binary
packages.kwaainet-aarch64-linux-musl        ARM64 static binary
packages.kwaainet-x86_64-linux-musl         x86_64 static binary
packages.p2pd-<target>                      cross-compiled p2pd
packages.*-container-<target>               cross-arch OCI images
packages.test-cross-smoke-<target>          QEMU smoke tests

devShells.default               full dev environment (Rust, Go, protobuf, etc.)

checks.clippy                   workspace-wide clippy (--deny warnings)
checks.cargoTest                workspace-wide cargo test
checks.kwaainet-smoke           sandboxed smoke test (--help, setup, identity)

formatter                       nixfmt
```

## Understanding the Nix environment

### Nix build vs development shell

Nix provides two distinct modes of operation.  Understanding the difference
helps avoid confusion:

**`nix build` (derivation)**:
- Source code is copied into `/nix/store/` (read-only, sandboxed)
- Build happens in a pure, isolated environment (no network, no home dir)
- Output is a deterministic, immutable store path
- Ideal for CI, releases, and reproducible artifacts

**`nix develop` (development shell)**:
- Works with your actual source code in your working directory
- Provides the same toolchain as the build, but in an interactive shell
- You can edit files, run `cargo build`, iterate, and use git normally
- Tools "disappear" when you exit the shell — nothing is installed globally

The `flake.nix` provides both: `nix build .#kwaainet` for the former,
`nix develop` for the latter.

See also: [Development environment with nix-shell](https://nixos.wiki/wiki/Development_environment_with_nix-shell)

### What this repository provides

The Nix setup consists of `flake.nix`, `flake.lock`, and modular files in
`nix/`:

- **`flake.nix`** — main entry point; wires up builds, containers, tests,
  cross-compilation, and the dev shell
- **`flake.lock`** — pins exact versions so all developers use identical inputs
- **`nix/packages.nix`** — shared dependency lists (DRY across build + devshell)
- **`nix/crane.nix`** — two-phase Rust build (cached deps + source)
- **`nix/p2pd.nix`** — go-libp2p-daemon Hivemind fork
- **`nix/proto.nix`** — protobuf codegen derivation
- **`nix/containers.nix`** — OCI container images
- **`nix/cross.nix`** — cross-compilation module
- **`nix/devshell.nix`** — development shell configuration
- **`nix/tests/`** — test infrastructure (smoke, two-node, containers, cross)
- **`Makefile`** — convenience targets wrapping nix commands

All Nix packages are sourced from [nixpkgs](https://github.com/NixOS/nixpkgs/)
and are searchable at [search.nixos.org](https://search.nixos.org/packages?channel=unstable).

---

## Architecture

```
flake.nix                 orchestrator — wires modules together
Makefile                  build targets with dedicated output symlinks
nix/
  packages.nix            shared dependency lists (DRY across build + devshell)
  p2pd.nix                go-libp2p-daemon Hivemind fork (buildGoModule)
  proto.nix               protobuf codegen derivation (protoc + protoc-gen-prost)
  crane.nix               two-phase Rust build (crane: buildDepsOnly + buildPackage)
  cross.nix               cross-compilation module (reuses crane/p2pd/containers with cross pkgs)
  containers.nix          OCI container images (streamLayeredImage)
  devshell.nix            nix develop environment
  overlays/
    cross-fixes.nix       overlay to disable broken tests under cross-compilation
  shell-functions/
    ascii-art.nix         logo display on nix develop entry
  tests/
    default.nix           test orchestration — maps checks + runnable packages
    containers.nix        container image tests (load, size, run)
    cross-smoke.nix       QEMU user-mode smoke test for cross-compiled binaries
    smoke.nix             sandboxed: --help, setup, identity show
    two-node.nix          integration: two nodes, distinct identities, port config
```

### Design decisions

- **Crane two-phase build** — dependencies are compiled once in `buildDepsOnly`
  (keyed on `Cargo.lock`) and cached.  Source-only changes skip dependency
  compilation entirely, cutting rebuild times from ~5-10 min to ~1-2 min.
  Crane reads `Cargo.lock` directly — no `cargoHash` to maintain.
- **Per-binary packages** — the workspace binaries (kwaainet, p2pd) are
  separate derivations so changes to one don't rebuild the other.
- **Minimal OCI containers** — each container includes only the binary, CA
  certificates (`cacert`), and timezone data (`tzdata`).  No shell, no
  coreutils — minimal attack surface.  Built with `streamLayeredImage` so
  there is no intermediate tarball; the output is a script that streams
  directly to `docker load` or `podman load`.
- **Modular `nix/` layout** — the flake delegates to single-purpose modules
  (following the pattern used by the redpanda and xdp2 Nix setups).
- **Separate p2pd derivation** — the upstream `build.rs` clones a Git repo and
  runs `go build` at Rust compile time.  Nix builds are sandboxed (no network),
  so we build p2pd as an independent `buildGoModule` and patch `build.rs` to
  point at it.  This also means `p2pd` is cached independently.
- **Protobuf codegen as a separate derivation** — the upstream `build.rs` runs
  `protoc` via `prost_build` to generate Rust types from `p2pd.proto`.  The
  `proto.nix` derivation runs `protoc` + `protoc-gen-prost` (both from nixpkgs)
  to produce `p2pd.pb.rs`.  Because Nix tracks the hash of the proto source
  directory, the derivation automatically rebuilds whenever `p2pd.proto` changes.
  The `crane.nix` patches `build.rs` to copy this pre-generated file instead of
  running `prost_build`, eliminating `protobuf` as a build-time dependency of
  the Rust package.
- **`packages.nix` as single source of truth** — build inputs are defined once
  and shared by `crane.nix` and `devshell.nix`.
- **Smoke test in sandbox, integration tests outside** — the smoke test verifies
  the binary works without network access (`nix flake check`).  The two-node
  test and container tests need runtime resources (localhost networking, container
  runtime) so they are runnable scripts, not sandboxed checks.
- **Version from Cargo.toml** — `crane.nix` reads the workspace version via
  `builtins.fromTOML`, so the Nix package version stays in sync automatically.

## First build — filling in hashes

Nix requires content hashes for reproducibility.  On the first build, only the
p2pd Go module hashes need to be set:

1. **`nix/p2pd.nix` → `vendorHash`** — run `nix build .#p2pd` and copy the
   expected hash from the error message.

The Rust build uses crane, which reads `Cargo.lock` directly — no hash to
maintain.  When Cargo dependencies change, rebuild is automatic.

## Multi-architecture support

The flake uses `flake-utils.lib.eachDefaultSystem`, which covers native builds
on `x86_64-linux`, `aarch64-linux`, `x86_64-darwin`, and `aarch64-darwin`.

### Cross-compilation (x86_64-linux only)

From an x86_64-linux host, you can cross-compile for four additional targets:

| Target | Suffix | Notes |
|--------|--------|-------|
| `aarch64-unknown-linux-gnu` | `-aarch64-linux-gnu` | ARM64, dynamic (glibc) |
| `aarch64-unknown-linux-musl` | `-aarch64-linux-musl` | ARM64, static |
| `x86_64-unknown-linux-musl` | `-x86_64-linux-musl` | x86_64, static |
| `riscv64gc-unknown-linux-gnu` | `-riscv64-linux-gnu` | RISC-V 64-bit, dynamic (glibc) |

#### How Nix cross-compilation works

Nix handles cross-compilation at the package-set level rather than requiring
per-project toolchain configuration.  The key mechanism is importing nixpkgs
with a `crossSystem`:

```nix
pkgsCross = import nixpkgs {
  localSystem = "x86_64-linux";           # build host
  crossSystem = { config = "aarch64-unknown-linux-gnu"; };  # target
};
```

This produces a complete package set (`pkgsCross`) where every package —
compilers, libraries, build tools — is configured to run on the build host
but produce binaries for the target architecture.  nixpkgs handles the
cross-toolchain setup (GCC/binutils for the target, pkg-config wiring,
sysroot paths) automatically.

KwaaiNet's cross-compilation reuses the same build modules unchanged:

```
flake.nix
  ├─ native build (existing)
  │   └─ crane.nix → kwaainet
  │
  └─ cross builds (x86_64-linux only)
      └─ nix/cross.nix (for each crossSystem)
          ├─ pkgsCross = import nixpkgs { localSystem; crossSystem; overlays; }
          ├─ crane.nix  (reused — craneLib built from pkgsCross)
          ├─ p2pd.nix   (reused — buildGoModule sets GOOS/GOARCH via pkgsCross)
          └─ containers.nix (reused — streamLayeredImage for target arch, Linux only)
```

Each language toolchain picks up the cross configuration differently:

- **Rust (crane)** — `crane.mkLib pkgsCross` produces a crane library that uses
  the cross Rust toolchain.  `CARGO_BUILD_TARGET` is set to the Rust target
  triple (e.g., `aarch64-unknown-linux-gnu`).  Cargo reads per-target rustflags
  from `core/.cargo/config.toml` automatically (e.g., fp16 flags for aarch64).
- **Go (p2pd)** — `pkgsCross.callPackage ./p2pd.nix {}` invokes `buildGoModule`
  from the cross package set, which automatically sets `GOOS=linux` and
  `GOARCH=arm64` (or the appropriate values).  p2pd uses `CGO_ENABLED=0` so
  there are no C dependencies to cross-compile.
- **OCI containers** — `dockerTools.streamLayeredImage` from `pkgsCross` produces
  images with the correct architecture metadata (e.g., `linux/arm64`).

The `nix/overlays/cross-fixes.nix` overlay disables test suites for a few
nixpkgs packages (`boehmgc`, `libuv`) that fail under cross-compilation because
they try to execute target-architecture binaries on the build host.

#### What has been tested

All cross-compiled outputs have been built and verified from an x86_64-linux
host:

**Binaries** (4 per target, 16 total):

| Binary | aarch64-gnu | aarch64-musl | x86_64-musl | riscv64-gnu |
|--------|:-----------:|:------------:|:-----------:|:-----------:|
| kwaainet | PASS | PASS | PASS | PASS |
| p2pd | PASS | PASS | PASS | PASS |

**OCI container images** (3 per target, 12 total):

| Container | aarch64-gnu | aarch64-musl | x86_64-musl | riscv64-gnu |
|-----------|:-----------:|:------------:|:-----------:|:-----------:|
| kwaainet-container | PASS | PASS | PASS | PASS |

**QEMU user-mode smoke tests** (1 per target, 4 total):

| Target | Emulation | `--help` | `--version` |
|--------|-----------|:--------:|:-----------:|
| aarch64-gnu | qemu-aarch64 + glibc sysroot | PASS | PASS |
| aarch64-musl | qemu-aarch64 (static binary) | PASS | PASS |
| x86_64-musl | native (same CPU arch) | PASS | PASS |
| riscv64-gnu | qemu-riscv64 + glibc sysroot | PASS | PASS |

**Binary verification:**

| Target | `file` output | `ldd` output |
|--------|---------------|--------------|
| aarch64-gnu | `ELF 64-bit LSB pie executable, ARM aarch64` | dynamically linked (glibc) |
| aarch64-musl | `ELF 64-bit LSB pie executable, ARM aarch64` | `not a dynamic executable` |
| x86_64-musl | `ELF 64-bit LSB pie executable, x86-64` | musl-linked |
| riscv64-gnu | `ELF 64-bit LSB pie executable, UCB RISC-V` | dynamically linked (glibc) |

**Native regression:** `nix flake check` (clippy + cargo test + smoke test)
passes with no regressions after the cross-compilation changes.

#### Building cross targets

```bash
# Build a single cross target
nix build .#kwaainet-aarch64-linux-gnu
nix build .#kwaainet-aarch64-linux-musl
nix build .#kwaainet-x86_64-linux-musl
nix build .#kwaainet-riscv64-linux-gnu

# Build all binaries for a target (sequential within target)
make cross-aarch64-gnu
make cross-aarch64-musl
make cross-x86_64-musl
make cross-riscv64-gnu

# Build all cross targets in parallel
make -j cross

# Build cross OCI container images
make -j cross-containers

# Verify the binary architecture
file result-kwaainet-aarch64-linux-gnu/bin/kwaainet
# → ELF 64-bit LSB pie executable, ARM aarch64 ...

# Verify static linking (musl targets)
ldd result-kwaainet-aarch64-linux-musl/bin/kwaainet
# → not a dynamic executable
```

#### Cross smoke tests

Cross-compiled binaries are verified using QEMU user-mode emulation.  For
aarch64 targets, `qemu-aarch64` runs the ARM64 binary on the x86_64 host.
For riscv64, `qemu-riscv64` does the same.  For x86_64-musl, no emulation
is needed since it's the same CPU architecture.

Static (musl) binaries run under QEMU without any sysroot — they have no
dynamic library dependencies.  Dynamic (glibc) binaries need the cross libc
as a `QEMU_LD_PREFIX` so the dynamic linker can resolve shared libraries.

```bash
# Run all cross smoke tests
make test-cross

# Or individually
nix build .#test-cross-smoke-aarch64-linux-gnu
nix build .#test-cross-smoke-aarch64-linux-musl
nix build .#test-cross-smoke-x86_64-linux-musl
nix build .#test-cross-smoke-riscv64-linux-gnu
```

#### Limitations

- Cross-compilation is only available from `x86_64-linux` hosts
- Darwin targets (macOS) require Xcode SDK — not supported from Linux
- Windows MSVC targets require the MSVC linker — not supported from Linux
- First cross build is slow (compiles cross toolchain + all deps); subsequent
  builds use the Nix cache

## OCI containers

Each binary is available as a minimal OCI container image built with
`streamLayeredImage`.  Images contain only the binary, CA certificates, and
timezone data — no shell, no coreutils.

```bash
# Build and load a container image
make kwaainet-container
./result-kwaainet-container | docker load    # or: podman load
docker run --rm kwaainet:0.3.25 --help

# Build all containers in parallel
make -j containers

# Run the container test suite (requires podman or docker)
make test-containers
```

Container images are tagged with the workspace version from `core/Cargo.toml`
(e.g., `kwaainet:0.3.25`).

| Container | Exposed port | Contents |
|-----------|-------------|----------|
| `kwaainet` | — | kwaainet binary + bundled p2pd |
| `kwaainet-all` | — | all binaries (kwaainet + p2pd) |

All containers include `SSL_CERT_FILE` and `TZDIR` environment variables
pre-configured for TLS and log timestamps.

## Development workflow

```bash
# Enter dev shell with Rust, Go, protobuf, and formatters
nix develop

# Build from source (inside dev shell, same as non-Nix workflow)
cd core
cargo build
cargo test --all

# Format Nix files
nix fmt

# Build via Makefile (preferred — uses dedicated output symlinks)
make kwaainet
./result-kwaainet/bin/kwaainet --help
```

## Tests

### Smoke test (sandboxed)

```bash
make check    # or: nix flake check
```

Verifies `kwaainet --help`, `setup`, `identity show`, and `config show` all
succeed without network access.  Also runs workspace-wide clippy and cargo test.

### Two-node integration test

```bash
make test    # or: nix run .#test-two-node
```

Creates two isolated node environments with separate `HOME` directories,
generates distinct identities, configures different P2P ports, starts both
nodes, and verifies they are running.

### Container image test

```bash
make test-containers    # or: nix run .#test-containers
```

Requires `podman` or `docker`.  For each container image:
1. Streams and loads the image into the container runtime
2. Verifies the image size is under 200 MB
3. Runs the binary inside the container to verify it starts

## Updating dependencies

Nix pins every input via `flake.lock`.  The sections below cover the kinds of
update you will encounter.

### Updating nixpkgs (and other flake inputs)

```bash
# Update all flake inputs (nixpkgs, flake-utils, crane) to latest
nix flake update

# Update only nixpkgs
nix flake update nixpkgs

# Pin nixpkgs to a specific commit
nix flake update nixpkgs --url github:NixOS/nixpkgs/<commit>
```

After updating, rebuild to verify nothing broke:

```bash
make all && make check
```

Commit the updated `flake.lock` alongside any other changes.

### Updating Rust (Cargo) dependencies

When `core/Cargo.lock` changes (new crate versions, added/removed deps), crane
picks up the changes automatically — no hash to update.  Just rebuild:

```bash
cd core && cargo update && cd ..
make all
```

### Updating the p2pd (Go) dependency

If the go-libp2p-daemon fork bumps its version or Go module dependencies:

1. Update `version`, `rev`, and `hash` in `nix/p2pd.nix`.
   - To get the new source `hash`, temporarily set it to `""` and run
     `nix build .#p2pd` — Nix prints the correct hash.
2. Update `vendorHash` the same way — set to `""`, build, copy the hash.
3. Rebuild: `make all`

### Updating protobuf definitions

When `core/crates/kwaai-p2p-daemon/proto/p2pd.proto` changes, no manual action
is needed — Nix tracks the source hash and automatically rebuilds the `protoRs`
derivation (and anything that depends on it) on the next build.

To verify the generated code independently:

```bash
make proto
ls result-proto/
```

### Quick-reference: which hash lives where

| What changed | File to update | Field |
|---|---|---|
| `flake.lock` inputs | (auto via `nix flake update`) | — |
| `core/Cargo.lock` | (automatic — crane reads Cargo.lock) | — |
| p2pd Go source | `nix/p2pd.nix` | `hash` |
| p2pd Go modules | `nix/p2pd.nix` | `vendorHash` |
| `p2pd.proto` | (automatic — no hash to update) | — |
