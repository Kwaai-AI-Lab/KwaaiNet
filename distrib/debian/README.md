# Debian packaging

Builds `kwaainet_<version>_<arch>.deb` for amd64 and arm64.

The package **does not build the Rust code**. It repackages the
`kwaainet-<triple>.tar.xz` that `release.yml`'s `build-local-artifacts` job has
already produced. The binary in the `.deb` is therefore byte-identical to what
tarball and installer users get — the package is a different delivery mechanism
for the same artifact, not a second build of it.

## Layout

| Path | Mode | Notes |
| --- | --- | --- |
| `/usr/bin/kwaainet` | 0755 | |
| `/usr/share/man/man1/kwaainet.1.gz` | 0644 | |
| `/usr/share/doc/kwaainet/copyright` | 0644 | DEP-5, MIT |
| `/usr/share/doc/kwaainet/changelog.Debian.gz` | 0644 | |
| `/usr/share/doc/kwaainet/README.Debian` | 0644 | |
| `/usr/lib/kwaainet/packaged` | 0644 | Contains `deb` |
| `/usr/share/lintian/overrides/kwaainet` | 0644 | |

`~/.kwaainet` is never touched on remove or purge. It holds the node identity
key; deleting it loses the node's identity on the network.

## No p2pd

`p2pd` is not packaged. The Go libp2p daemon is being removed upstream — nodes
run rust-libp2p in process — so shipping it would package something on its way
out. The payload archives still contain it today, so `stage-payload.sh`
*ignores* it rather than rejecting it; that keeps the build working on both
sides of the upstream removal. `test-install.sh` asserts the package ships no
`p2pd`, so it cannot come back by accident.

## The packaged marker

`/usr/lib/kwaainet/packaged` contains `deb`. The Rust updater reads it and
refuses to self-update, because a self-update would overwrite files dpkg owns
and leave the package database disagreeing with the filesystem.
The marker is the whole mechanism: there is no `/etc` file to keep in step
with it, and `KWAAINET_NO_AUTO_UPDATE` remains purely a developer escape hatch.

## No service, deliberately

This package installs **no systemd unit and no `/etc` configuration**. Run
`kwaainet start` exactly as with a tarball install. The node runs as the
invoking user, with config, logs and identity key under `~/.kwaainet`, so each
user on a machine has their own node — which is also what the KwaaiNetGUI
package needs, since the GUI's daemon runs as the desktop user and writes that
directory.

A packaged **system** service is the right end state for headless installs, but
it is a bigger change than packaging and is deliberately deferred:

- it needs a `kwaainet` system user, state under `/var/lib/kwaainet` and
  configuration under `/etc/kwaainet`;
- `kwaainet_dir()` in `core/crates/kwaai-cli/src/config.rs` is a *single* root —
  `config_file()`, `run_dir()`, `log_dir()`, `identity.key`, `rag/` and
  `storage/` all hang off it, so there is no config/state split to package
  against;
- `config.yaml` is read-**write** at runtime. 19 call sites invoke `save()`,
  including the first-run create, the `initial_peers` migration, and a
  mid-startup write in `main.rs` that persists map-derived model settings.
  Several are `let _ = cfg.save()`, so against a root-owned `/etc` file they
  would fail *silently*.

A user unit is not a workaround: systemd stops user units when the session ends
unless `loginctl enable-linger` is set, so it would not survive logout on a
server. `kwaainet service install` still writes one for desktop users who want
start-at-login, and that path is unchanged by this package.

## Depends is derived, never written

`build-deb.sh` runs `dpkg-shlibdeps` against `usr/bin/kwaainet` inside
`debian:bookworm`.

Currently derived, for both amd64 and arm64:

```
libc6 (>= 2.34), libgcc-s1 (>= 4.2)
```

`expected-depends.amd64` and `expected-depends.arm64` pin that string per
architecture and `package-linux` diffs against them on every arch, so the
supported-distro floor cannot move without someone deciding to move it. The two
are identical today, but they are derived from different ELFs and can diverge.
Regenerate it from a real build's `depends.amd64` when it legitimately changes.

The container's architecture must match the payload's, or `dpkg-shlibdeps`
cannot resolve the ELF's libraries at all — hence `docker/setup-qemu-action`
for the arm64 matrix leg.

### Supported distributions

`libc6 (>= 2.34)` means **Debian 12 (bookworm) or newer** and **Ubuntu 22.04 or
newer**. Verified: installs and runs on Debian 12 and Ubuntu 22.04 (glibc
2.35, the floor); on Debian 11 (glibc 2.31) apt refuses it with an unmet
dependency, which is the point of deriving `Depends` rather than guessing it —
the user gets a dependency error at install time, not a `GLIBC_2.34 not found`
crash at run time.

## Signing

This package ships `.sha256` sidecars and nothing else. **Per-`.deb`
signatures are deliberately not produced.**

apt does not verify individual `.deb` signatures. It verifies the
*repository*'s `InRelease` signature, and trusts the checksums inside it.
`debsigs`/`debsig-verify` exist, but require a per-key policy XML installed on
every client, and apt never invokes them — so a signed `.deb` would be
reassuring and unverified, which is worse than an unsigned one.

Signing therefore belongs at the repository level, and is out of scope here
because no apt repository exists yet. See the `TODO(packages-repo)` block at
the end of `release.yml`: when `Kwaai-AI-Lab/packages` exists, the repo's
`InRelease` gets signed there, and that is what actually protects users.

## lintian

Target is zero **errors** after overrides, not a lintian-clean package.

As of this writing the package emits **no tags at all** on
`debian:bookworm` — errors, warnings and info alike. The overrides in
`lintian-overrides` are what silence the following, each of which is expected
and correct:

| Tag | Why it is overridden |
| --- | --- |
| `unstripped-binary-or-object` | Symbols kept: Rust panic backtraces in user bug reports beat the saved size |
| `embedded-library` | `libyaml`, vendored by a Rust `-sys` crate; unbundling means rebuilding, which this design excludes |
| `initial-upload-closes-no-bugs` | Not distributed via the Debian archive; there is no BTS to close |

If a *new* tag appears, that is a real change — read it rather than adding an
override reflexively.

## Building locally

Everything runs in a container; `dpkg` and `lintian` are not needed on the host.

```sh
# 1. Get a payload without building (any released version)
gh release download v0.6.7 --repo Kwaai-AI-Lab/KwaaiNet \
  --pattern 'kwaainet-x86_64-unknown-linux-gnu.tar.xz' --dir /tmp/kwaai-pkg
distrib/packaging/stage-payload.sh \
  /tmp/kwaai-pkg/kwaainet-x86_64-unknown-linux-gnu.tar.xz /tmp/kwaai-pkg/payload

# 2. Build + lint
docker run --rm --platform linux/amd64 -v "$PWD:/src:ro" -v /tmp/kwaai-pkg:/pkg \
  debian:bookworm bash -c '
    apt-get update -qq >/dev/null && apt-get install -y -qq dpkg-dev lintian >/dev/null
    cp -r /src /work && cd /work
    bash distrib/debian/build-deb.sh --payload /pkg/payload --arch amd64 \
      --outdir /pkg/out --lintian'

# 3. Install smoke test
docker run --rm --platform linux/amd64 -v "$PWD:/src:ro" -v /tmp/kwaai-pkg:/pkg:ro \
  debian:bookworm bash /src/distrib/debian/test-install.sh \
  /pkg/out/kwaainet_0.6.8-1_amd64.deb
```

`build-deb.sh --depends '<string>'` skips `dpkg-shlibdeps` entirely. It exists
for the CI stub payload — shell scripts have no ELF to analyze — and must not
be used for a real package.

## Versioning

`distrib/packaging/version.sh` maps upstream to `${UPSTREAM//-/~}-1`. The `~`
is required twice over: a bare `-` is illegal in a Debian version that carries
a revision, and `~` sorts a prerelease *before* the final release, so
`0.7.0~rc.1` correctly precedes `0.7.0` where `0.7.0-rc.1` would follow it.

The version comes from the git tag where there is one, falling back to
`[workspace.package] version` in `core/Cargo.toml`, because `release.yml`'s
`plan.outputs.tag` is `github.ref_name` — a *branch* under `workflow_dispatch`.
Only a tag that looks like a version (`v1.2.3` or `1.2.3`) is accepted; a
branch name falls through to `Cargo.toml`.

## CI

`release.yml` gained `package-linux` and `upload-packages-to-release`, inside a
`BEGIN/END hand-added` comment block. They are **not** cargo-dist output:
running `dist init` or `dist generate` without `--allow-dirty ci` regenerates
release.yml and deletes them. `ci.yml`'s `packaging-guard` job fails the build
if they disappear.

Two safety properties are deliberate:

- The artifact is named **`packages-<arch>`**, not `artifacts-<arch>`. The
  `host` job globs `artifacts-*` into the release-creation step, so an
  `artifacts-` prefix would let a packaging bug break the release itself.
- `upload-packages-to-release` is `continue-on-error: true` and is **not** in
  `host`'s `needs`, so a packaging failure never blocks or delays a release.

`ci.yml`'s `package-smoke` builds from a stub payload (two shell scripts) on
every PR, in roughly 25 seconds. It validates control generation, layout,
permissions, lintian and a full install/remove/purge cycle — everything
except the parts that need a real binary, which `package-linux` covers.
