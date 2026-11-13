# KwaaiNet PRs #272 and #273 on metro-win (Windows 11)

- **#272** `fix(rpc): create the Windows token temp file owner-only (#270)`, reviewed at `b71a3d19e8c5eaafa0e48d3a91e18fd8c4b86934`
- **#273** `fix(cli): one node per KWAAINET_HOME, held for the daemon's lifetime`, reviewed at `65b4bfd52404665e22a1b8008d718d788d87785d`

Both heads were re-checked with `gh pr view <n> --json headRefOid` after testing. They were unchanged.

- Date: 2026-11-12 (local), 2026-11-13 UTC.
- Machine: metro-win, Windows 11 Pro for Workstations 10.0.22631, RTX A5000.
- Reza was at the screen.

**Both builds are CPU-only.** They were plain `cargo build -p kwaainet --release` with no CUDA feature, so the shard ran on CPU.

## Summary

| Step | Result |
|---|---|
| A1 #272 DACL tests listed and pass | PASS |
| A2 mutation makes both DACL tests fail; revert passes | PASS |
| A3 clippy `-p kwaai-rpc` | PASS |
| A4 real token file owner-only; GUI shows node, no auth error | PASS (see note: 0.7.3's final file already looked the same) |
| B0 #273 tests + release build | PASS |
| B1 baseline restart | PASS |
| B2 restart race ×5 | PASS |
| B3 reconnect race ×3 | PASS |
| B4 second `run-node` refused, PID/token untouched | PASS |
| B5 `start --daemon` refused while running | PASS |
| B6 stop removes `kwaainet.pid` and `kwaainet.token` | **FAIL** (pid gone, token stays) — not caused by #273; same with 0.7.3 and #272 |
| B7 hard kill, then `start --daemon` | **FAIL** (refused "already running") — not caused by #273; 0.7.3 refuses the same way 2/2 |
| Restore installed 0.7.3 with original flags | PASS |

Neither FAIL comes from the new instance lock. Both come from the Windows stop/kill path, and both reproduce on the installed 0.7.3. Details are under B6 and B7.

## 0. Setup

```
cargo 1.93.1 (083ac5135 2025-12-15)
rustc 1.93.1 (01f6ddf75 2026-02-11)
/c/Users/METRO_WIN11/.cargo/bin/kwaainet
kwaainet 0.7.3
  🟢 Status:  Running
  🔢 PID:     19560
  🟢 Shard:   Running (PID 11312)
  🟢 Storage: Running (PID 22108, port 7432)
```

`start-args.json` was backed up to `%USERPROFILE%\start-args.backup.json`:

```json
{ "model": null, "blocks": null, "port": null, "grpc_port": null, "no_gpu": false,
  "public_name": null, "public_ip": null, "announce_addr": null, "identity_key": null,
  "no_relay": false, "shard": true, "no_contribute": false }
```

The installed 0.7.3's token, before any change: `icacls` → `DESKTOP-6LL31ML\METRO_WIN11:(F)`. That is one entry, not inherited.

Worktrees were created at the two heads. The patch files `core/patches/{cudarc,libp2p-kad,multistream-select}.patch` are byte-identical in this clone and in both worktrees (`cmp`). The fetched directories were then copied from this clone into each worktree.

For #273, `gh pr view 273 --json files` and `git diff 3d42c558..65b4bfd5 --stat` both count 5 files.

## Part A: #272

### A1: PASS

```
$ cargo test -p kwaai-rpc -- --list | grep -Ei 'dacl|owner_only'
auth::tests::temp_file_is_owner_only_from_creation: test
auth::tests::token_file_dacl_is_current_user_only: test
auth::tests::token_file_round_trips_owner_only: test

$ cargo test -p kwaai-rpc
test auth::tests::temp_file_is_owner_only_from_creation ... ok
test auth::tests::token_file_dacl_is_current_user_only ... ok
test result: ok. 7 passed; 0 failed   (lib)
test result: ok. 17 passed; 0 failed  (proto_roundtrip)
```

### A2: PASS

The mutation was applied at `auth.rs:57`:

```diff
-    return windows_acl::create_new_owner_only(path);
+    return std::fs::OpenOptions::new().write(true).create_new(true).open(path);
```

The tests were run with `cargo test -p kwaai-rpc --lib`. The two DACL tests are lib tests, so `--lib` covers them.

```
test auth::tests::temp_file_is_owner_only_from_creation ... FAILED
test auth::tests::token_file_dacl_is_current_user_only ... FAILED
---- auth::tests::temp_file_is_owner_only_from_creation stdout ----
DACL not protected: D:(A;ID;FA;;;SY)(A;ID;FA;;;BA)(A;ID;FA;;;S-1-5-21-1241282684-165535981-3015193593-1001)
---- auth::tests::token_file_dacl_is_current_user_only stdout ----
DACL not protected: D:(A;ID;FA;;;SY)(A;ID;FA;;;BA)(A;ID;FA;;;S-1-5-21-1241282684-165535981-3015193593-1001)
test result: FAILED. 5 passed; 2 failed
```

The mutated file's DACL has inherited ACEs (`ID`) for SYSTEM, Administrators and the user, and no `D:P`.

After `git checkout -- crates/kwaai-rpc/src/auth.rs` (HEAD = `b71a3d19`), the full run passes again: lib 7/7, proto 17/17, doc tests 2 ignored.

### A3: PASS

```
$ cargo clippy -p kwaai-rpc --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 16.45s     (exit 0)
```

### A4: PASS

The release build took 14 m 38 s.

1. `kwaainet stop` (installed binary) stopped PID 19560. It left `kwaainet.grpc` and `kwaainet.token` behind, which is the taskkill path; see B6.
2. `start-args.json` was restored from the backup.
3. `$K272 restart` printed `✅ KwaaiNet daemon restarted (PID 15848)`. Shard 19276 and storage 20060 came up.

```
$ $K272 status
  🟢 Status:  Running
  🔢 PID:     15848
$ icacls %USERPROFILE%\.kwaainet\run\kwaainet.token        (mtime 16:34, fresh)
C:\Users\METRO_WIN11\.kwaainet\run\kwaainet.token DESKTOP-6LL31ML\METRO_WIN11:(F)
$ (Get-Acl ...kwaainet.token).Sddl
O:S-1-5-21-…-1001G:S-1-5-21-…-513D:P(A;;FA;;;S-1-5-21-1241282684-165535981-3015193593-1001)
```

The result is one entry, for the current user, Full control, with no `(I)`, and the DACL is protected (`D:P`).

**Note:** the installed 0.7.3's final token file already showed the same single `(F)` entry before the test. So this real-file check does not tell #272 apart from 0.7.3. What #272 fixes is the window between the temp file's creation and its rename, and A1/A2 are the evidence for that.

**GUI.** There is no installed GUI on this machine, only test builds. Reza opened `C:\Users\METRO_WIN11\kwaainet-gui-fix-test\app5\kwaainet_gui.exe`, which is the KwaaiNetGUI #20 build at `ee5ed81`. He reported, word for word:

> "it shows node running, no errors"

`$K272 stop` stopped PID 15848, and `start-args.json` was restored.

## Part B: #273

`$K273` = `kn-273\core\target\release\kwaainet.exe`. The release build took 13 m 15 s.

The GUI from A4 (PID 9168) stayed open for all of Part B.

### B0: PASS

```
$ cargo test -p kwaainet --bin kwaainet instance_lock
test daemon::instance_lock_tests::instance_lock_admits_one_holder_at_a_time ... ok
test result: ok. 1 passed; 0 failed; 413 filtered out
$ cargo test -p kwaainet --bin kwaainet grpc_server::tests
test grpc_server::tests::failed_bind_leaves_the_live_token_alone ... ok
test grpc_server::tests::shutdown_leaves_a_token_it_did_not_write ... ok
test result: ok. 51 passed; 0 failed; 363 filtered out
build exit=0   Finished `release` profile [optimized] target(s) in 13m 15s
```

### B1: PASS

```
✅ KwaaiNet daemon restarted (PID 12172)
🟢 Status:  Running   🔢 PID: 12172   🟢 Shard: Running (PID 2648)   🟢 Storage: Running (PID 17552, port 7432)
run/: kwaainet.grpc kwaainet.lock kwaainet.pid kwaainet.token … start-args.json
```

### B2: PASS

**Is the refusal count meaningful?** `restart` (main.rs:118) and `reconnect` (main.rs:755) both spawn through `DaemonManager::spawn_daemon_child`. That function sends the child's stdout and stderr to `logs/kwaainet.log` (daemon.rs:309–310). So a refused daemon's `Error: another KwaaiNet node is running…` would land in that log.

The log was live during the loop: its mtime was 16:43:55 at 16:43:58. `grep -c` exits 1 when it counts 0 matches; an exit of 2 would mean an error.

```
restart run 1: exit=0 old_pid=12172 | Status: Running PID: 23160 | refusals before=0 after=0 new=0
restart run 2: exit=0 old_pid=23160 | Status: Running PID: 17412 | refusals before=0 after=0 new=0
restart run 3: exit=0 old_pid=17412 | Status: Running PID: 5252  | refusals before=0 after=0 new=0
restart run 4: exit=0 old_pid=5252  | Status: Running PID: 23884 | refusals before=0 after=0 new=0
restart run 5: exit=0 old_pid=23884 | Status: Running PID: 22580 | refusals before=0 after=0 new=0
```

Run 1 also printed `✅ Orphaned shard server stopped`.

### B3: PASS

```
reconnect run 1: exit=0 old_pid=22580 | Status: Running PID: 21732 | new refusals=0   ✅ Node restarted (PID 21732). Reconnecting to P2P network.
reconnect run 2: exit=0 old_pid=21732 | Status: Running PID: 9896  | new refusals=0
reconnect run 3: exit=0 old_pid=9896  | Status: Running PID: 7420  | new refusals=0
```

### B4: PASS

```
before: pid=7420 token_sha=92abf1d1e8c8aa89 grpc=8093   (pid, token mtime 16:44:13)
$ $K273 run-node; exit=1
Error: another KwaaiNet node is running on C:\Users\METRO_WIN11\.kwaainet — stop it, or give this one its own KWAAINET_HOME
after:  pid=7420 token_sha=92abf1d1e8c8aa89 grpc=8093   (pid, token mtime 16:44:13)
🟢 Status: Running  🔢 PID: 7420
```

A foreground `run-node` prints the refusal to stderr only; `kwaainet.log` stayed at 0. That's expected: only a daemon child's stderr goes to the log.

### B5: PASS

```
$ $K273 start --daemon; exit=1
  ✅ Confirmed model: unsloth/Llama-3.1-8B-Instruct
  ⚠️  A KwaaiNet node is already running on this KWAAINET_HOME.
  💡 Stop the existing node with: kwaainet stop
🟢 Status: Running  🔢 PID: 7420
```

The refusal comes only after the network-map lookup and model confirmation. That lookup runs before the `is_running()` check, and a successful lookup saves the model fields to `config.yaml`. That behaviour predates #273.

### B6: FAIL (not caused by #273)

```
$ $K273 stop
INFO kwaainet::daemon: Stopping daemon PID 7420
✅ KwaaiNet daemon stopped          (exit 0)
run/:
  kwaainet.grpc    16:44:13
  kwaainet.lock    16:42:22
  kwaainet.token   16:44:13     ← still there
  reannounce.flag  16:44:58
  relay_addr_cache.json, shard.pid 16:44:58, update_check.json
```

`kwaainet.pid` is gone, but `kwaainet.token` (and `kwaainet.grpc`) from the stopped node remain.

On Windows, `stop` kills the node with taskkill, so the node's own shutdown never runs and never removes them. The same leftover files appeared after `kwaainet stop` with the installed 0.7.3 and after `$K272 stop` (A4). #273 doesn't change this.

`shard.pid` and `reannounce.flag` changed at 16:44:58. That was the shard of node 7420 becoming ready, 45 s after start: `shard_cmd.rs` rewrites `shard.pid` and signals reannounce at that point. B5's refused start did not write them; `shard.pid` held 22952, the running shard's own PID.

### B7: FAIL (not caused by #273)

```
$ $K273 restart        → ✅ KwaaiNet daemon restarted (PID 15196)
$ taskkill /PID 15196 /F   → SUCCESS
tasklist (10 s later): kwaainet.exe 13968  (19 GB — the shard serve child, still alive)
$ $K273 start --daemon; exit=1
  ⚠️  A KwaaiNet node is already running on this KWAAINET_HOME.
$ $K273 status → 🔴 Status: Not running
```

**Cause.** This was reproduced with a per-second `tasklist` poll and a Win32 `OpenProcess`/`GetExitCodeProcess` probe:

```
live: 18440 parent=24572 kwaainet.exe run-node --shard
live: 13516 parent=18440 kwaainet.exe storage serve
live: 11252 parent=18440 kwaainet.exe shard serve --auto-rebalance
taskkill /PID 18440 /F
--- 3 s after kill
PID 18440 : OpenProcess OK, exit code 1 (259 = STILL_ACTIVE)    ← exited, but the process object lives on
live: 11252 parent=18440 … shard serve --auto-rebalance           ← orphaned shard still running
$ $K273 start --daemon → exit=1, "already running on this KWAAINET_HOME"
+7s / +13s / +19s  PID 18440 : OpenProcess OK, exit code 1
+25s               PID 18440 : OpenProcess failed, error 87       ← gone once the shard exits
```

The killed node disappears from `tasklist` within 1 s. But while the orphaned `shard serve` is alive (about 25 s), its process object can still be opened, with exit code 1. `DaemonManager::is_running()` asks only whether sysinfo can find the PID, so it reports the dead node as running. The start then stops at its early check (`main.rs`, the `start` arm, `if mgr.is_running()`).

The instance lock is not involved: it was released, as the next run shows. The same start once the zombie had cleared:

```
run/: kwaainet.grpc kwaainet.lock kwaainet.pid kwaainet.token …   (dead node's pid, lock file present)
$ $K273 start --daemon → exit=0  ✅ KwaaiNet daemon started (PID 20376); status Running
```

So a hard kill does not leave a stale lock. That part of the PR holds.

**A/B against the installed 0.7.3.** Each cycle was: restart, 20 s wait, `taskkill /F`, 3 s wait, then `start --daemon`.

```
273-a: node=22616 shard=7000  orphan_alive=1 | before: OpenProcess OK, exit code 1 | exit=1 'already running on this KWAAINET_HOME'
073-a: node=24232 shard=23756 orphan_alive=1 | before: OpenProcess OK, exit code 1 | exit=1 'already running'
273-b: node=1460  shard=8432  orphan_alive=1 | before: OpenProcess OK, exit code 1 | exit=1 'already running on this KWAAINET_HOME'
073-b: node=18956 shard=5076  orphan_alive=1 | before: OpenProcess OK, exit code 1 | exit=1 'already running'
```

0.7.3 refuses in the same state. That fits `is_running()` being unchanged since v0.7.3 (`git show v0.7.3:core/crates/kwaai-cli/src/daemon.rs`).

One earlier 0.7.3 run, done in the same way, did start (`✅ KwaaiNet daemon started (PID 11312)`) even though the zombie was present 3 s after the kill. I did not capture why.

In the runs without a shard (`start --daemon` without `--shard`), the killed PID could not be opened within 1 s, and the start succeeded (PID 16708, PID 11840).

**Possible fix, not in this PR.** On Windows, `is_running()` could treat a process whose `GetExitCodeProcess` is not `STILL_ACTIVE` as dead. Separately, the orphaned shard surviving its parent's kill for about 25 s is itself worth a look.

## Restore: PASS

```
$ kwaainet restart        (installed)  → ✅ KwaaiNet daemon restarted (PID 22564)
$ kwaainet --version → kwaainet 0.7.3   (/c/Users/METRO_WIN11/.cargo/bin/kwaainet)
$ kwaainet status
  🟢 Status:  Running   🔢 PID: 22564   🟢 Shard: Running (PID 22436)   🟢 Storage: Running (PID 14236, port 7432)
22564 parent=14908 "C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe" run-node --shard
start-args identical to backup
```

The original flags (`shard: true`) are back, on the installed binary. Both worktrees were removed with `git worktree remove --force`.

## Notes

- **`config.yaml` was not edited by hand.** Its mtime moved, last to 16:58:21 (the restore restart), because `start` and `restart` save the model fields after the network-map lookup. Afterwards it reads `model: unsloth/Llama-3.1-8B-Instruct`, `blocks: 32`, the same model the node announced throughout.
- **The KwaaiNetGUI #20 test build (app5, PID 9168) was left running** after A4.
- `C:\Users\METRO_WIN11\kwaainet-gui-fix-test\.kwaainet-update-0.5.1\`, dated 2026-11-11 20:02, looks like an update staged by one of the earlier GUI test builds. It was left untouched.
- **Not tested:**
  - a CUDA build;
  - two homes side by side (separate `KWAAINET_HOME`);
  - `stop` with the PID file missing;
  - a group-signal variant (Windows has no process-group signal to send here).
