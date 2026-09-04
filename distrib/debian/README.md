# Debian packaging

Builds `kwaainet_<version>_<arch>.deb` for amd64 and arm64.

The package **does not build the Rust code**. It repackages the
`kwaainet-<triple>.tar.xz` that `release.yml`'s `build-local-artifacts` job has
already produced, with `p2pd` already injected. The binaries in the `.deb` are
therefore byte-identical to what tarball and installer users get — the package
is a different delivery mechanism for the same artifact, not a second build of
it.

## Layout

| Path | Mode | Notes |
| --- | --- | --- |
| `/usr/bin/kwaainet` | 0755 | |
| `/usr/bin/p2pd` | 0755 | On `PATH` deliberately — see below |
| `/usr/share/man/man1/kwaainet.1.gz` | 0644 | |
| `/usr/share/doc/kwaainet/copyright` | 0644 | DEP-5, MIT |
| `/usr/share/doc/kwaainet/changelog.Debian.gz` | 0644 | |
| `/usr/share/doc/kwaainet/README.Debian` | 0644 | |
| `/usr/lib/systemd/user/kwaainet.service` | 0644 | Installed, **not** enabled |
| `/etc/default/kwaainet` | 0644 | dpkg conffile |
| `/usr/lib/kwaainet/packaged` | 0644 | Contains `deb` |
| `/usr/share/lintian/overrides/kwaainet` | 0644 | |

`~/.kwaainet` is never touched on remove or purge. It holds the node identity
key; deleting it loses the node's identity on the network.

## Why `p2pd` is in `/usr/bin`, not a private libexec dir

`find_p2pd_binary()` (`core/crates/kwaai-cli/src/node.rs`) searches exactly
three places: next to `current_exe()`, a cargo target dir, and `$PATH`. A
private `/usr/libexec/kwaainet/` would be invisible to all three, and
`kwaainet start` would fall through to *downloading* p2pd into `/usr/bin` as a
non-root user. `/usr/bin/p2pd` is the only layout the existing lookup supports
without a code change.

## The packaged marker

`/usr/lib/kwaainet/packaged` contains `deb`. The Rust updater reads it and
refuses to self-update, because a self-update would overwrite files dpkg owns
and leave the package database disagreeing with the filesystem.
`/etc/default/kwaainet` additionally sets `KWAAINET_NO_AUTO_UPDATE=1`.

## systemd: a user unit, not a system one

Configuration, logs and the identity key all live in `~/.kwaainet`, and
`core/crates/kwaai-cli/src/service.rs` already installs a *user* unit. The
packaged unit matches it, with two deliberate differences:

- `ExecStart=/usr/bin/kwaainet run-node` — a fixed path, where `service.rs`
  uses `current_exe()`.
- `EnvironmentFile=-/etc/default/kwaainet` and `-/etc/sysconfig/kwaainet`, both
  `-`-prefixed, so the identical unit file also serves the future RPM.

Nothing enables it: starting a node is the admin's decision, and a user unit
has no meaningful system-wide enable anyway. The package ships no maintainer
scripts at all — man-db has its own dpkg trigger, and conffile handling is
dpkg-internal, so there is nothing for a postinst to do.

**`kwaainet service install` keeps winning.** It writes
`~/.config/systemd/user/kwaainet.service`, and systemd gives a unit in the
user's own directory precedence over `/usr/lib/systemd/user/`. Anyone who has
ever run that command keeps their own unit, and edits to the packaged one have
no effect until they remove it. `README.Debian` says so to the user.

## Depends is derived, never written

`build-deb.sh` runs `dpkg-shlibdeps` against `usr/bin/kwaainet` inside
`debian:bookworm`. Only `kwaainet` is passed: `p2pd` is Go built
`CGO_ENABLED=0` (`scripts/build-p2pd.sh`), so it is a static ELF and
`dpkg-shlibdeps` errors on it.

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
| `statically-linked-binary [usr/bin/p2pd]` | Go, `CGO_ENABLED=0`, static by design |
| `unstripped-binary-or-object` | Symbols kept: Rust panic backtraces in user bug reports beat the saved size |
| `embedded-library` | `libyaml`, vendored by a Rust `-sys` crate; unbundling means rebuilding, which this design excludes |
| `no-manual-page [usr/bin/p2pd]` | Internal helper the node spawns, not a user-facing command |
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
permissions, conffiles, lintian and a full install/remove cycle — everything
except the parts that need a real binary, which `package-linux` covers.
