# Native-p2p cross-OS stress test — metro machine setup

Everything below runs **on the metro machine itself** (there is no SSH to them).
Run the Linux block on metro-linux and the Windows block on metro-win.

The change is a **runtime config flag**, not a rebuild-to-revert: `native_p2p: true`
swaps the Go `p2pd` child process for the in-process rust-libp2p stack. Rollback is
one line plus a restart, so this is safe to try on the production node.

---

## 0. What you need installed first

| Tool | Why | Note |
|------|-----|------|
| Rust toolchain | builds `kwaainet` | already present if you've built KwaaiNet before |
| **Go** | `kwaai-p2p-daemon/build.rs` still clones+builds go-libp2p-daemon | **still required on this branch** — the Go removal is Phase 5, not done yet |
| git, curl, bash | the `multistream-select` patch fetch | on Windows use **Git Bash**, not PowerShell/cmd |
| protoc | proto codegen | build.rs downloads it automatically if missing |

The build needs Go even though the native path never launches `p2pd` at runtime.
That surprised me too — worth knowing before you start.

---

## 1. metro-linux

```bash
cd ~/Source/KwaaiNet        # wherever your checkout lives

# NOTE: there is no `origin/native-p2p` branch. The stack tip (all 8 PRs, #79-#86)
# is the head of PR #86: native-p2p-pr7-nat-traversal
git fetch origin native-p2p-pr7-nat-traversal
git checkout -B native-p2p origin/native-p2p-pr7-nat-traversal
git log --oneline -1        # expect: 8822d4d chore: verification-run metrics...

# fetch + patch multistream-select (required — the build fails without it)
bash core/patches/fetch-multistream-select.sh

# build (thin LTO keeps this to minutes instead of ~1.5h)
cd core
CARGO_PROFILE_RELEASE_LTO=thin CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 \
  cargo build -p kwaainet --release

# install
kwaainet stop
cp target/release/kwaainet ~/.cargo/bin/kwaainet

# flip to the native stack
kwaainet config set native_p2p true
kwaainet start --daemon
```

### What good looks like

```bash
kwaainet logs -n 60 | grep -iE "peer id|control socket|relay reservation|reachability"
```

You want to see, within ~10s of start:

- `Peer ID: 12D3KooWCzuhpXrZ…` — **the same peer ID as before** (identity key is reused)
- `control socket listening addr=/unix//tmp/kwaai-p2pd.sock`
- `relay reservation confirmed relay=Qm…` — ideally two, one per bootstrap
- `announce state changed reachability=Private using_relay=true announceable=true`
- `STORE response from Qm…: 1/1 stored`

And **no** `p2pd` process should exist any more:

```bash
pgrep -af p2pd     # expect: no output
kwaainet p2p peers list
```

---

## 2. metro-win  ← the high-value one

Windows exercises a code path that has **never been run**: on Windows the control
socket is not a unix socket but TCP on `/ip4/127.0.0.1/tcp/5005`. The migration doc
lists "Windows TCP parity for pipe mode is untested" as an open item — this run is
the test.

Use **Git Bash** (the patch fetch script is bash-only; there is no PowerShell equivalent).

```bash
cd ~/Source/KwaaiNet
git fetch origin native-p2p-pr7-nat-traversal
git checkout -B native-p2p origin/native-p2p-pr7-nat-traversal
git log --oneline -1        # expect: 8822d4d

# Git Bash needs: curl, shasum, tar, patch, mktemp — all ship with Git for Windows
bash core/patches/fetch-multistream-select.sh

cd core
CARGO_PROFILE_RELEASE_LTO=thin CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 \
  cargo build -p kwaainet --release
```

If `fetch-multistream-select.sh` fails on a missing tool, tell me which one — that
is itself a finding (the script has no Windows equivalent and CI only runs it on
ubuntu-latest, so Windows build portability is currently unproven).

Then stop the node, replace the binary, and flip the flag:

```bash
kwaainet stop
cp target/release/kwaainet.exe "$(dirname "$(which kwaainet)")/kwaainet.exe"
kwaainet config set native_p2p true
kwaainet start --daemon
```

### What good looks like on Windows

```bash
kwaainet logs -n 60 | grep -iE "peer id|control socket|relay reservation"
```

- `control socket listening addr=/ip4/127.0.0.1/tcp/5005`  ← **the untested path**
- same peer ID as before (`12D3KooWLMizEbVi…`)
- relay reservations confirmed
- `kwaainet p2p peers list` returns rows — this proves a *client* can talk to the
  TCP control socket, which is the actual parity question

---

## 3. Run the stress agent (both machines)

Copy `stress_agent.sh` and `targets.tsv` from this directory onto each machine, then:

```bash
# Linux
bash stress_agent.sh --targets targets.tsv --duration 3600 --workers 4 \
     --label metro-linux-native --node-pid "$(pgrep -f 'kwaainet run-node' | head -1)"

# Windows (Git Bash)
bash stress_agent.sh --targets targets.tsv --duration 3600 --workers 4 \
     --label metro-win-native
```

It writes `stress-<label>/progress.json` every 15s and `stress-<label>/results.jsonl`
per RPC. Send me `results.jsonl` when it finishes (or just paste `progress.json`
partway through and I can tell you how it's tracking).

---

## 4. Rollback — if anything looks wrong

```bash
kwaainet stop
kwaainet config set native_p2p false
kwaainet start --daemon
```

That returns the node to the Go `p2pd` path immediately. The binary can stay; only
the flag decides which stack runs. If the new binary itself is the problem, restore
the previous `kwaainet` from your release install (or `cargo install` the tag you
were on).

---

## What I'm looking for in the results

1. **Windows control socket over TCP works at all** — the explicit open item.
2. **Same peer ID, still announceable** — a native node must be indistinguishable
   to the rest of the fleet.
3. **Sustained RPC success rate** vs. what the same node did on the p2pd path.
4. **RSS drift over the hour.** On my macOS native node RSS went 19.5MB → 43.9MB in
   90 seconds; I need a long soak to tell warm-up from a leak, and a second and
   third OS to tell a platform quirk from a real one.
