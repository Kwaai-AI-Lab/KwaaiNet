# Building KwaaiNet from source on metro-linux

For validating `main` on the A5000 box before a release tag exists. Run
directly on the machine — there is no SSH to metro (Tailscale trial ended
2026-07-25).

Roughly **60–90 minutes** for a clean release build.

**The box:** HP Z8 G4, dual Xeon Gold 6154 (36 cores / 72 threads), 96 GB RAM,
RTX A5000 (24 GB), 850 GB root with ~450 GB free. The core-count-to-RAM ratio
matters for build parallelism — see the CUDA section below.

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
go version               # kwaai-p2p-daemon's build.rs builds p2pd
protoc --version         # optional: build.rs downloads it if absent
nvcc --version           # only if you want the CUDA feature — see below
```

`nvcc` missing means the `cuda` feature will not build; nothing in a network
regression pass needs it. Everything else has a fallback.

## Build

```bash
cd ~/Source/KwaaiNet          # adjust if the checkout lives elsewhere
git fetch origin
git checkout main
git pull

git log --oneline -1          # expect 7acebeaf or later
```

Then:

```bash
cd core
cargo build --release -p kwaainet
```

**A CPU build is the right choice for a network regression pass**, and this is
not a fallback. Everything validated here — routed dials, relay circuits, config
handling — is network behaviour and never touches the GPU. Building CUDA buys
nothing for these tests and carries a real risk, below.

### If you do need the CUDA build

Matching what CI does for this target (`.github/workflows/release.yml`,
*Build kwaainet with CUDA (Linux)*).

**Cap the parallelism.** On 2026-08-25 an uncapped CUDA build took this machine
down for roughly six hours:

```
oom-kill:constraint=CONSTRAINT_NONE,...,global_oom,task=cicc,pid=1796144
Out of memory: Killed process 1796144 (cicc) total-vm:3690824kB anon-rss:2989432kB
```

`cicc` is nvcc's device-code compiler, and it holds **~3 GB** per translation
unit. Cargo defaults to one job per *logical* CPU, and this box is a dual Xeon
Gold 6154 — **36 cores / 72 threads against 96 GB of RAM**. That ratio is the
whole problem: an uncapped build asks for up to 72 × 3 GB ≈ 216 GB on a machine
with 96. It is not a question of whether it OOMs, only how far in.

The machine did not crash — it wedged: the kernel and NIC stayed healthy and the
listening sockets stayed open, so TCP connects still completed, while every
surviving process sat starved of memory and serviced nothing. `sshd` accepted
connections and never wrote its banner. It looked from outside like a dead host
and needed console access to recover.

**Use `-j 8`** — roughly 24 GB of `cicc` at peak, leaving ample headroom for
rustc's own codegen and for the node itself if it is still running. Going wider
buys little: the CUDA kernels are a small part of the build and the Rust link
step is serial regardless.

```bash
cd core
export CUDA_COMPUTE_CAP=80    # A5000 is compute capability 8.6; CI pins 80
cargo build --release -p kwaainet --features cuda -j 8
```

Confirm the headroom before starting, and note whether there is swap — without
it the OOM killer fires with no warning shot:

```bash
free -h; nproc; swapon --show
```

Budget ~3 GB per job against *available* memory, not total.

`CUDA_COMPUTE_CAP=80` is what CI uses and what the published CUDA artifact is
built with, so this reproduces the shipped binary rather than a local variant.

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

## Enable a persistent journal

Worth doing once, before anything else goes wrong. `journalctl -b -1` on this
machine reports *"no persistent journal was found"* — the journal is volatile,
so every boot discards the previous one and a post-mortem has nothing to read.
The 2026-08-25 OOM was only diagnosable because the machine never actually
rebooted.

```bash
sudo mkdir -p /var/log/journal
sudo systemd-tmpfiles --create --prefix /var/log/journal
sudo systemctl restart systemd-journald
journalctl --list-boots        # should start accumulating entries
```

## Do not enable `decentralized_dht` yet

It defaults to `false` and should stay there for this pass. Turning it on
changes where records are placed — the *k* nearest peers rather than the
bootstraps — and if only some nodes have it on, publishers and readers can
disagree about where a record lives. That needs a coordinated set of at least
three nodes, not one machine.
