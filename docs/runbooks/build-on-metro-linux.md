# Building KwaaiNet from source on metro-linux

For validating `main` on the A6000 box before a release tag exists. Run
directly on the machine — there is no SSH to metro (Tailscale trial ended
2026-07-25).

Roughly **60–90 minutes** for a clean release build.

## Why build rather than wait for a release

Tagging a release *is* the fleet rollout: `updater.rs` fetches
`releases/latest`, `contribute.auto_update` defaults to `true`, and there is no
prerelease filter — so every auto-updating node takes the tag. Building here
validates the network-facing fixes (#137 routed dials, #138 relay circuit
sizing) on a real NATed peer before that happens.

## Prerequisites

Almost certainly already present from the last build; check rather than assume.

```bash
rustc --version          # 1.75+
nvcc --version           # CUDA toolkit, for the GPU feature
go version               # kwaai-p2p-daemon's build.rs builds p2pd
protoc --version         # optional: build.rs downloads it if absent
```

`nvcc` missing means the `cuda` feature will not build. Everything else has a
fallback.

## Build

```bash
cd ~/Source/KwaaiNet          # adjust if the checkout lives elsewhere
git fetch origin
git checkout main
git pull

git log --oneline -1          # expect 7acebeaf or later
```

Then, matching what CI does for this target
(`.github/workflows/release.yml`, *Build kwaainet with CUDA (Linux)*):

```bash
cd core
export CUDA_COMPUTE_CAP=80    # A6000 is compute capability 8.6; CI pins 80
cargo build --release -p kwaainet --features cuda
```

`CUDA_COMPUTE_CAP=80` is what CI uses and what the published CUDA artifact is
built with, so this reproduces the shipped binary rather than a local variant.

Without a working CUDA toolchain, a CPU build still exercises everything
network-related — which is the point of this exercise:

```bash
cargo build --release -p kwaainet
```

## Install

Back up the running binary first; a running process keeps its inode, so
replacing the file does not disturb the live node until it restarts.

```bash
cp ~/.cargo/bin/kwaainet ~/.cargo/bin/kwaainet-0.6.2.bak
cp target/release/kwaainet ~/.cargo/bin/kwaainet
kwaainet --version
```

## Restart and verify

```bash
kwaainet stop
kwaainet start --daemon
sleep 20

kwaainet p2p info | head -20
kwaainet shard chain | head -12
```

Expect: the node reappears in `shard chain`, and `p2p info` shows a confirmed
external address or a relay reservation.

## What this build is being tested for

Nothing here changes behaviour by default — `decentralized_dht` is `false` and
`announce_self` is `true`, so this node keeps acting exactly as it does today.
What is being checked is that the fixes hold under real traffic:

**#137 — routed dials.** A bare-`PeerId` request to a NATed peer used to fail on
the first call, because kad stores *listen* addresses and a NATed peer's are
unreachable. Cold-path test, from the Mac or another node:

```bash
# no connection held first — this is the case that used to fail
kwaainet p2p probe --peer <metro-linux-peer-id> \
  --proto /kwaai/storage/1.0.0 --count 3
```

**#138 — relay circuit sizing.** libp2p's default caps a circuit at 128 KiB,
which killed any bulk transfer mid-flight. A NAT→NAT VPK batch is the test:

```bash
kwaainet vpk bench --batch-size 500 --dimensions 384
```

Before the fix this died partway with `yamux … connection is closed`, surfaced
as `Network not initialized`.

**#139 — config reload before save.** Watch for KB registrations disappearing
from `config.yaml` while the node runs. They should not.

```bash
kwaainet rag list          # count before
sleep 600
kwaainet rag list          # same count
```

## Rolling back

```bash
kwaainet stop
cp ~/.cargo/bin/kwaainet-0.6.2.bak ~/.cargo/bin/kwaainet
kwaainet start --daemon
```

## Do not enable `decentralized_dht` yet

It defaults to `false` and should stay there for this pass. Turning it on
changes where records are placed — the *k* nearest peers rather than the
bootstraps — and if only some nodes have it on, publishers and readers can
disagree about where a record lives. That needs a coordinated set of at least
three nodes, not one machine.
