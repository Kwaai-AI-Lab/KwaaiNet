# metro-win: KwaaiNet auto-update fix — onboarding

This machine (`metro-win`, Windows) is a KwaaiNet node running distributed inference. You're
running here to help recover it and validate a just-committed fix. Read this fully before doing
anything — the fix touches the exact update/restart mechanism you'll be using to deploy it, so
order of operations matters.

## Context — what happened

This session (on a separate Mac, working in the same GitHub repo) found and fixed two related
bugs in KwaaiNet's auto-update mechanism:

1. **Unix race condition** (already released in v0.5.3, commit `7c57102`): the new daemon process
   spawned after an auto-update could see a stale PID file from the still-cleaning-up old process
   and exit immediately, leaving nothing running. Confirmed happening on `metro-linux`; fixed by
   deferring the respawn until after the old process's cleanup (DHT unannounce, p2pd shutdown,
   PID-file removal) fully completes.

2. **Windows: auto-update never restarted the daemon at all** (fixed in commit `2ec22b9`, pushed
   to the branch **`fix/windows-auto-update-respawn`** — **not on `main`, not in any tagged
   release**). `maybe_auto_update()`'s Windows branch always
   returned `None`, based on a comment claiming a separate "installer batch" process would kill
   and restart the daemon — no such mechanism exists anywhere in the codebase.
   `install_update()` just renames the running `.exe` aside and copies in new files directly,
   in-process (Windows allows this — the OS loader opens EXEs with `FILE_SHARE_DELETE`, so the
   memory mapping stays valid after the rename). Nothing ever followed up to actually kill and
   restart the process. This is exactly what you're likely seeing right now: `kwaainet --version`
   probably already reports a newer version than `kwaainet status` implies (check the uptime —
   if it predates the last time the version file changed, that's the bug).

   The fix (commit `2ec22b9`) makes Windows return `Some(version)` unconditionally, routing it
   through the same respawn-after-cleanup path Unix already uses, and fixes binary-path
   resolution to look for `kwaainet.exe` (not `kwaainet`) on Windows.

**Important: this Windows fix has NOT been tested on real Windows hardware.** It compiled cleanly
and passed clippy on macOS, and mirrors an already-proven `cfg(windows)` pattern elsewhere in the
codebase, but you are the first real-world test of it.

## Step 1 — Check current state, don't assume

```powershell
kwaainet --version
kwaainet status
```

If `Status` shows `Not running` (main daemon dead) but `Shard`/`Storage` show running PIDs, that's
the pre-2ec22b9 Windows bug's known symptom — those are orphaned children from an old daemon that
never got cleanly restarted. If `Status` shows `Running` but with an uptime clearly older than
when the binary on disk last changed, that's the same symptom in its other observable form (old
process still executing, binary silently swapped out from under it).

## Step 2 — Recover: restart the daemon now

Regardless of which symptom you saw:

```powershell
kwaainet restart
```

(or `kwaainet stop` then `kwaainet start --daemon` if `restart` doesn't cleanly handle orphaned
shard/storage children — check `kwaainet status` after either way). This gets the node back on
the network on whatever binary is currently installed. Confirm with `kwaainet status` (fresh
uptime, all green) and ideally from the other side too — ask the user to confirm via
`kwaainet shard chain --total-blocks 32` from another machine that `metro-win` shows up with a
fresh, low uptime and current version.

## Step 3 — Build and deploy the actual fix (branch `fix/windows-auto-update-respawn`)

The fix is on a dedicated branch, not `main` and not in any tagged release, so the binary you just
restarted does **not** contain it. To get this machine onto the fixed code:

```powershell
git clone https://github.com/Kwaai-AI-Lab/KwaaiNet.git   # if not already cloned somewhere
cd KwaaiNet
git fetch origin
git checkout fix/windows-auto-update-respawn
git log --oneline -3   # confirm 2ec22b9's cherry-pick (same message, different hash) is present
cd core
cargo build -p kwaainet --release
```

If you already have a clone with local changes or a different branch checked out, `git fetch
origin` then `git checkout fix/windows-auto-update-respawn` is enough — no need to touch `main`.

Check first whether Rust/Go toolchains are already installed (`cargo --version`, `go version`) —
if this machine has been building KwaaiNet before, they likely are. If not, you'll need:
Rust (rustup.rs), Go (for the `kwaai-p2p-daemon` build script), and a C++ toolchain for CUDA
crates if this machine has an NVIDIA GPU (it likely does — `metro-win` was seen in the network map
with CUDA-relevant naming/context).

Once built:

```powershell
Copy-Item .\target\release\kwaainet.exe -Destination "<wherever the current kwaainet.exe lives>" -Force
kwaainet restart
kwaainet status   # confirm it's running, fresh uptime
kwaainet --version   # should now report the version from Cargo.toml at this commit — check core/Cargo.toml for the exact string, it may still read 0.5.3 since this fix hasn't been version-bumped yet
```

**Note:** this gets the *daemon* onto the fixed code, but does **not** by itself test the fix —
the bug only manifests during an actual auto-update-triggered respawn, not a manual
build-and-restart. The real test happens next time a *new tagged release* goes out and this
machine's background auto-update check picks it up on its own. Don't force that with `kwaainet
update` — that's a different, already-working manual-update code path (in `main.rs`, not
`node.rs`), unrelated to the bug that was just fixed.

## Step 4 — Report back

Once steps 1–3 are done, report back (to the user, who will relay to the other Claude session, or
directly if you have a way to reach it):
- What `kwaainet status`/`--version` showed initially (which symptom, if any)
- Whether the manual restart in step 2 worked cleanly
- Whether the build in step 3 succeeded, and on what toolchain versions
- Any errors encountered at any step — these are valuable regardless of whether they're related
  to the fix, since this is a fairly fresh Windows build/deploy path

## What NOT to do

- Don't force `kwaainet update` to try to test the auto-update fix — it won't exercise the actual
  code path that was fixed (see Step 3 note above).
- Don't push commits, create tags, or trigger CI from this machine — that's coordinated from the
  other session/user.
- Don't delete `kwaainet.exe.old` (the backup the installer keeps) unless asked — it's the
  rollback path if something goes wrong.
