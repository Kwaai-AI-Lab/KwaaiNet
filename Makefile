# KwaaiNet build targets
#
# Each target writes its output to a dedicated result-<name> symlink,
# so builds don't overwrite each other.
#
# Parallel builds:
#   make -j all          build both binaries in parallel
#   make -j containers   build both OCI containers in parallel
#   make -j cross        cross-compile all targets in parallel
#   make -j all containers cross   build everything in parallel

.PHONY: all kwaainet map-server p2pd proto \
        containers kwaainet-container map-server-container kwaainet-all-container \
        cross cross-aarch64-gnu cross-aarch64-musl cross-x86_64-musl cross-riscv64-gnu \
        cross-containers cross-containers-aarch64-gnu cross-containers-aarch64-musl cross-containers-x86_64-musl cross-containers-riscv64-gnu \
        check test test-containers test-cross fmt develop clean \
        ci-check ci-fmt ci-lint ci-test githooks

all: kwaainet map-server

# --- Binaries ---

kwaainet:
	nix build .#kwaainet -o result-kwaainet

map-server:
	nix build .#map-server -o result-map-server

p2pd:
	nix build .#p2pd -o result-p2pd

proto:
	nix build .#protoRs -o result-proto

# --- OCI Containers (Linux only) ---

containers: kwaainet-container map-server-container kwaainet-all-container

kwaainet-container:
	nix build .#kwaainet-container -o result-kwaainet-container

map-server-container:
	nix build .#map-server-container -o result-map-server-container

kwaainet-all-container:
	nix build .#kwaainet-all-container -o result-kwaainet-all-container

# --- Cross-compilation (x86_64-linux only) ---

cross: cross-aarch64-gnu cross-aarch64-musl cross-x86_64-musl cross-riscv64-gnu

cross-aarch64-gnu:
	nix build .#kwaainet-aarch64-linux-gnu -o result-kwaainet-aarch64-linux-gnu
	nix build .#map-server-aarch64-linux-gnu -o result-map-server-aarch64-linux-gnu
	nix build .#p2pd-aarch64-linux-gnu -o result-p2pd-aarch64-linux-gnu

cross-aarch64-musl:
	nix build .#kwaainet-aarch64-linux-musl -o result-kwaainet-aarch64-linux-musl
	nix build .#map-server-aarch64-linux-musl -o result-map-server-aarch64-linux-musl
	nix build .#p2pd-aarch64-linux-musl -o result-p2pd-aarch64-linux-musl

cross-x86_64-musl:
	nix build .#kwaainet-x86_64-linux-musl -o result-kwaainet-x86_64-linux-musl
	nix build .#map-server-x86_64-linux-musl -o result-map-server-x86_64-linux-musl
	nix build .#p2pd-x86_64-linux-musl -o result-p2pd-x86_64-linux-musl

cross-riscv64-gnu:
	nix build .#kwaainet-riscv64-linux-gnu -o result-kwaainet-riscv64-linux-gnu
	nix build .#map-server-riscv64-linux-gnu -o result-map-server-riscv64-linux-gnu
	nix build .#p2pd-riscv64-linux-gnu -o result-p2pd-riscv64-linux-gnu

# --- Cross OCI Containers ---

cross-containers: cross-containers-aarch64-gnu cross-containers-aarch64-musl cross-containers-x86_64-musl cross-containers-riscv64-gnu

cross-containers-aarch64-gnu:
	nix build .#kwaainet-container-aarch64-linux-gnu -o result-kwaainet-container-aarch64-linux-gnu
	nix build .#map-server-container-aarch64-linux-gnu -o result-map-server-container-aarch64-linux-gnu
	nix build .#kwaainet-all-container-aarch64-linux-gnu -o result-kwaainet-all-container-aarch64-linux-gnu

cross-containers-aarch64-musl:
	nix build .#kwaainet-container-aarch64-linux-musl -o result-kwaainet-container-aarch64-linux-musl
	nix build .#map-server-container-aarch64-linux-musl -o result-map-server-container-aarch64-linux-musl
	nix build .#kwaainet-all-container-aarch64-linux-musl -o result-kwaainet-all-container-aarch64-linux-musl

cross-containers-x86_64-musl:
	nix build .#kwaainet-container-x86_64-linux-musl -o result-kwaainet-container-x86_64-linux-musl
	nix build .#map-server-container-x86_64-linux-musl -o result-map-server-container-x86_64-linux-musl
	nix build .#kwaainet-all-container-x86_64-linux-musl -o result-kwaainet-all-container-x86_64-linux-musl

cross-containers-riscv64-gnu:
	nix build .#kwaainet-container-riscv64-linux-gnu -o result-kwaainet-container-riscv64-linux-gnu
	nix build .#map-server-container-riscv64-linux-gnu -o result-map-server-container-riscv64-linux-gnu
	nix build .#kwaainet-all-container-riscv64-linux-gnu -o result-kwaainet-all-container-riscv64-linux-gnu

# --- Tests & checks ---

check:
	nix flake check

test:
	nix run .#test-two-node

test-containers:
	nix run .#test-containers

test-cross:
	nix build .#test-cross-smoke-aarch64-linux-gnu -o result-test-cross-aarch64-gnu
	nix build .#test-cross-smoke-aarch64-linux-musl -o result-test-cross-aarch64-musl
	nix build .#test-cross-smoke-x86_64-linux-musl -o result-test-cross-x86_64-musl
	nix build .#test-cross-smoke-riscv64-linux-gnu -o result-test-cross-riscv64-gnu

# --- Native cargo checks (mirror CI; work without Nix) ---
#
# `make check` above runs the Nix-based gate (smoke test + clippy + tests).
# These targets are the cargo-direct equivalent — faster, no Nix required,
# matching the exact flags CI uses in .github/workflows/ci.yml.

ci-check: ci-fmt ci-lint ci-test

ci-fmt:
	cd core && cargo fmt --all -- --check

ci-lint:
	cd core && cargo clippy --all-targets -- -D warnings

ci-test:
	cd core && cargo test --all

# --- Git hooks ---
#
# Run `make githooks` once per clone to enable the tracked hooks under
# .githooks/. Sets core.hooksPath, so any later updates to the hook
# scripts ship with the repo — no need to re-run install.

githooks:
	git config core.hooksPath .githooks
	@echo "Enabled tracked git hooks → .githooks/"

# --- Utilities ---

fmt:
	nix fmt

develop:
	nix develop

clean:
	rm -f result result-kwaainet result-map-server \
	      result-p2pd result-proto \
	      result-kwaainet-container result-map-server-container result-kwaainet-all-container \
	      result-kwaainet-aarch64-linux-gnu result-map-server-aarch64-linux-gnu result-p2pd-aarch64-linux-gnu \
	      result-kwaainet-aarch64-linux-musl result-map-server-aarch64-linux-musl result-p2pd-aarch64-linux-musl \
	      result-kwaainet-x86_64-linux-musl result-map-server-x86_64-linux-musl result-p2pd-x86_64-linux-musl \
	      result-kwaainet-container-aarch64-linux-gnu result-map-server-container-aarch64-linux-gnu result-kwaainet-all-container-aarch64-linux-gnu \
	      result-kwaainet-container-aarch64-linux-musl result-map-server-container-aarch64-linux-musl result-kwaainet-all-container-aarch64-linux-musl \
	      result-kwaainet-container-x86_64-linux-musl result-map-server-container-x86_64-linux-musl result-kwaainet-all-container-x86_64-linux-musl \
	      result-kwaainet-riscv64-linux-gnu result-map-server-riscv64-linux-gnu result-p2pd-riscv64-linux-gnu \
	      result-kwaainet-container-riscv64-linux-gnu result-map-server-container-riscv64-linux-gnu result-kwaainet-all-container-riscv64-linux-gnu \
	      result-test-cross-aarch64-gnu result-test-cross-aarch64-musl result-test-cross-x86_64-musl result-test-cross-riscv64-gnu
