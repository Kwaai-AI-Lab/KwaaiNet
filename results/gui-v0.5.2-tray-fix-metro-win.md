# GUI Windows tray fix (local builds of `826ab44`, `891dc90`, `ee5ed81`) on metro-win

- Round 1 tested `826ab44`.
- A fresh-context review found problems; round 2 tested the fixes in
  `891dc90`.
- A second fresh-context review of the whole diff led to `ee5ed81`, tested in
  round 3 (at the end).

Run 2026-11-11 (local, UTC−8) by a Claude Code session at the keyboard of
metro-win. Reza was at the GUI. His answers are quoted word for word.

Darren has no Windows machine, so this fix was written and tested here.

- Windows 11 Pro for Workstations 10.0.22631
- Fix: KwaaiNetGUI branch `fix/windows-tray-menu`. It was tested as a working
  tree on `27773e0` (the KwaaiNetGUI#16 head, now merged). It was then
  committed and rebased onto `origin/main` (`69c47e6`, release v0.5.1) as
  `826ab44`. `main` gained no change to the touched files in between, so the
  rebase moved no code.
- Built locally with Flutter 3.47.9 stable (CI's version), using
  `flutter build windows --release`. Windows Developer Mode had to be on,
  because plugin builds need symlinks.
- Staged in `%USERPROFILE%\kwaainet-gui-fix-test\app` with the same bundled
  `kwaainet.exe` (0.7.3) as the CI artifact.
- Node: started from the CLI (`kwaainet start --daemon --shard`) before the
  GUI. PID 11252, with shard 17292 and storage 18132, all from `.cargo\bin`.
- Follows `results/gui-v0.5.1-tray-retest2-metro-win.md` and KwaaiNetGUI#17.

## What the fix changes

| Problem (retest 2 / #17) | Change |
| --- | --- |
| The tray menu stays up when focus moves away | `tray.dart`: on Windows, `popUpContextMenu(bringAppToFront: true)`. tray_manager 0.5.3 calls `SetForegroundWindow` before `TrackPopupMenu` only when asked, and Windows dismisses a notification-icon menu on click-away only if its owner is foreground |
| Left-click on the icon does not reliably restore a hidden window | No separate change. The working hypothesis was that the stuck menu's modal loop swallowed later clicks |
| A second GUI can be launched | `windows/runner/main.cpp`: a named mutex. A second launch shows, restores and foregrounds the existing window, then exits |
| Quit stops a node the GUI did not start | `DaemonController.startedNode` is set when the GUI spawns the node. Quit (`performQuit` → `stopOnQuit`) stops the node only in that case. "Stop service" still stops any node |

## Summary

| Step | Result |
| --- | --- |
| 1. Right-click menu closes on click-away | **PASS** |
| 2. X, then tray left-click restores the window | **PASS**: 4 of 4 left-clicks restored it, one of them on a dedicated retry |
| 3. Second launch while hidden | **PASS**: the existing window came back, no second process, one tray icon |
| 4. Quit with a CLI-started node | **PASS**: the GUI exited, the node kept running with the same PIDs, no APPCRASH |

## Results

### 1. Menu click-away: PASS

Reza: "Yes"

### 2. X, then left-click: PASS

On the first pass Reza reported: "No the KwaaiNet Window disappears after X as
does the tray icon. The overflow icon remains and the GUI window reappear after
clicking Open KwaaiNet..."

The GUI log and the window recorder disagreed. Every left-click logged
`[tray] icon mouseDown → opening window` followed by
`[window-restore] window restored at chat`, and the window became visible and
foreground:

| X (window hidden) | Left-click (window visible, foreground) |
| --- | --- |
| 13:00:58 | 13:01:22 |
| 13:01:28 | 13:01:41 |
| 13:02:17 | 13:03:24 |

The screenshot at 13:01:23 shows the window in front. The "icon" that
disappears on X is the taskbar button, which is the intended
hide-to-tray behaviour.

Reza was asked to repeat it once: X, then a single left-click on the overflow
icon. Reza: "the window came back instantly after left click"

### 3. Second launch: PASS

With the window hidden by X, Reza launched
`\Users\METRO_WIN11\kwaainet-gui-fix-test\app\kwaainet_gui.exe` from a
terminal. Reza: "Works launched
\Users\METRO_WIN11\kwaainet-gui-fix-test\app\kwaainet_gui.exe via terminal -
one a single icon appeared in the tray."

The recorder shows the hidden window becoming visible at 13:06:28. GUI 14368
stayed the only `kwaainet_gui` process.

### 4. Quit: PASS

Right-click → Quit KwaaiNet. Reza: "done - the icon is gone"

GUI log:

```
[tray] menu click: key=quit
[tray] quit → performQuit
[shutdown] clean quit → stopping the daemon if this app started it
[daemon-state] quit: node was not started by this app — leaving it running
[window-close] close during quit → letting it through
```

- The GUI process was gone at 13:15:38.
- Node 11252, shard 17292 and storage 18132 were still running.
- `~/.kwaainet/run` was untouched: `start-args.json`, `kwaainet.pid` and
  `storage_serve.pid` still had their 10:25 timestamps.
- The Application log had no Event 1000 or 1001 in the 40 minutes covering
  the test.

## Round 2: review fixes (`891dc90`)

### What the review found in `826ab44`

1. **One mutex for every build.** A build run from a KwaaiNetGUI checkout
   (its own sandboxed node) could no longer run beside the installed app.
   The second one exited and raised the other's window.
2. **In-place update could leave no app.** The updater relies on Quit
   stopping the node: on Windows a running bundled `kwaainet.exe` blocks the
   rename of the install directory, and the swap script then exits without
   relaunching. With Quit leaving attached nodes alone, an update while
   attached would have done that.
3. **Ownership as a flag goes stale.** If the GUI's node was stopped from the
   CLI and the user started another, Quit would stop the user's node.
4. **A second launch un-maximized the window** (`SW_RESTORE`).

### Changes

- `main.cpp`: the mutex name is keyed to the exe path, and the existing
  window is found by class plus owning exe. The window is brought back with
  `SetWindowPlacement`. Dropping `SW_RESTORE` alone did not fix item 4: a
  new process's first `ShowWindow` is replaced by the launcher's
  `STARTUPINFO` show state.
- The update's quit stops the node whoever started it.
- `startedNode` is keyed to the pid the spawned node writes.
- Before spawning, the GUI removes a dead node's `kwaainet.grpc` and
  `kwaainet.pid`. See the step 4 failure below for why.

### Runner checks (scripted with Win32 calls, no clicks)

| Case | Before relaunch | After relaunch of the same exe |
| --- | --- | --- |
| A | hidden + maximized | visible, **maximized**. With the first attempt (`ShowWindow` without `SW_RESTORE`, build `app2`) it came back un-maximized |
| B | visible + maximized | maximized |
| C | minimized | restored |
| D | hidden + normal | visible |

In every case the second process exited with code 0 and the GUI count did
not change. A copy of the same build in another folder (`app2-copy`) ran
alongside, as intended.

### With Reza

Build `app3` (mutex, placement and update fixes), against CLI node 11252:

| Step | Result | Reza |
| --- | --- | --- |
| 1. Menu click-away | **PASS** | "yes" |
| 2. X, then left-click | **PASS** | "yes" |
| 3. Quit, node 11252 survives | **PASS** | "Yes" |

The log shows `quit: node was not started by this app — leaving it running`,
11252, 17292 and 18132 still up, and no Event 1000/1001.

**Step 4 with `app3`, Quit a node the GUI spawned: FAIL.** I ran
`kwaainet stop` on 11252. The GUI launched, and with no node running it
started one: bundled `app3\kwaainet.exe`, `run-node` PID 7064 (pid file 7064).
Reza had to use the overflow icon: "Quit KwaaiNet is only available in the
overflow icon". Then: "done - the icon is gone". The log said
`quit: node was not started by this app` and 7064 kept running.

Cause: on Windows, `kwaainet stop` uses `taskkill`, so the node never
removes `run/kwaainet.grpc`; `stop` deletes only the pid file. The GUI's wait
for the new node's port file found the stale one at once. It then read the
pid before the new `run-node` had written it, so the GUI disowned its own
node. The same stale file already made that bind wait meaningless before
this fix.

Fix (in `891dc90`): remove `kwaainet.grpc` and `kwaainet.pid` before
spawning, which is only done once no node is alive. A claim that finds no
pid keeps the node pending (ours) instead of dropping it.

Build `app4`:

| Step | Result | Evidence |
| --- | --- | --- |
| 4. Quit stops a node the GUI spawned | **PASS** | `kwaainet stop` again left the stale `kwaainet.grpc`. The GUI spawned `run-node` 21684, and Reza Quit: "gone". The log shows `stop() called` → `running: …\app4\kwaainet.exe stop`, and no `kwaainet` or GUI process was left. No Event 1000/1001 |
| 5. Quit leaves a CLI node (rerun on `app4`) | **PASS** | Node restored with `kwaainet start --daemon --shard` (run-node 9748, shard 10916, storage 11960, `shard: true`). The GUI attached without spawning. Reza Quit: "done". The log shows `leaving it running`, 9748/10916/11960 still up, pid file 9748, `start-args.json` present. No Event 1000/1001 |

## Round 3: second review (`ee5ed81`)

### What the second review found in `891dc90`

1. **A failed spawn stayed "ours".** If both spawn attempts never bound,
   the pending flag stayed set. A node the user later started from the CLI
   would then be stopped by Quit.
2. **The update's stop applied everywhere.** On macOS, Linux, or with a
   `.cargo\bin` daemon on Windows, it stopped a CLI or service node that
   doesn't block the swap.
3. **Deleting the run files could delete a live node's pid file** whenever
   the GUI's liveness check is wrong (EPERM from `kill -0`, a `tasklist`
   failure, or a CLI start racing the check). That defeats the CLI's own
   "already running" guard.
4. Smaller: an elevated instance makes `CreateMutexW` fail with
   `ERROR_ACCESS_DENIED`; an empty exe path gave every failing instance one
   shared mutex.

### Changes

- The bind wait now waits for `kwaainet.grpc` to name **the port this GUI
  handed out**, instead of deleting anything. A dead node's file names
  another port.
- A spawn that never binds is disowned, and attaching to a running node
  disowns any pending spawn.
- `updateMustStopNode`: the update stops an attached node only on Windows,
  and only if the daemon binary is inside the install directory.
- The runner treats `ERROR_ACCESS_DENIED` as already running, and skips the
  check if the exe path can't be read.

### With Reza (build `app5`)

| Step | Result | Evidence |
| --- | --- | --- |
| 4. Quit stops a node the GUI spawned, with a stale port file | **PASS** | After `kwaainet stop`, `kwaainet.grpc` still said 8093 (dead node). The GUI handed the new node 49705, waited for it, and spawned run-node 11908. Reza: "node is not running". The log shows `stop() called` → `running: …\app5\kwaainet.exe stop`. No `kwaainet` or GUI process left, no Event 1000/1001 |
| 5. Quit leaves a CLI node | **PASS** | `kwaainet start --daemon --shard` gave run-node 19560, storage 22108 and shard 11312. The GUI attached without spawning. Reza: "Quit clicked". The log shows `leaving it running`; 19560/22108/11312 still up, pid file 19560, `start-args.json` present, no Event 1000/1001 |

Steps 1–3 (menu, left-click, single instance) did not change in code since
round 2 and were not repeated.

### Local checks (`ee5ed81`)

- `flutter analyze`: no issues.
- Full suite on Windows: 543 pass, with the same 32 pre-existing Windows
  failures.

## State at the end

Node 19560 (with shard 11312 and storage 22108) from `.cargo\bin`, started
with `--shard`. No GUI running.

## Not tested here

- macOS and Linux. The Quit and stale-file changes apply on every platform;
  the tray and runner changes are Windows-only.
- The in-place update path. It is unit-tested only
  (`stopAttached stops the node whoever started it`, `updateMustStopNode`).
  A local build doesn't check for updates.
- Known gap: in the default built-in mode, the update still stops a
  `.cargo\bin` CLI node. The check uses the daemon path the GUI is set to,
  not the running node's image.

## Local checks (`891dc90`)

- `flutter analyze`: no issues.
- Full suite on Windows: 537 pass. The 32 failures are the same ones that fail
  on the unmodified `27773e0` on Windows. CI runs the suite on Linux.

## Evidence (local, not committed)

`%USERPROFILE%\kwaainet-gui-fix-test`:
- `gui-stderr.txt`, `gui2-stderr.txt`, `gui3-run2-stderr.txt`,
  `gui3-run3-stderr.txt`, `gui4-stderr.txt`, `gui4b-stderr.txt`,
  `gui5-stderr.txt` and `gui5b-stderr.txt`;
- `shots\` and `shots-r2\` (one screenshot per second), each with a
  `winstate.txt` of window-state changes.
