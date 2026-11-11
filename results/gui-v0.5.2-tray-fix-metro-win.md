# GUI Windows tray fix (local build of `826ab44`) on metro-win

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

## Not tested here

- **Quit stopping a node that the GUI itself spawned.** Unit-tested in
  `test/quit_leaves_attached_node_test.dart`. Testing it for real means
  stopping Reza's node first.
- macOS and Linux. The Quit change applies on every platform; the tray and
  runner changes are Windows-only.

## Local checks

- `flutter analyze`: no issues.
- The new tests and the existing quit and tray tests pass.
- Full suite on Windows: 531 pass. The 32 failures also fail on the
  unmodified `27773e0` on Windows. CI runs the suite on Linux.

## Evidence (local, not committed)

`%USERPROFILE%\kwaainet-gui-fix-test`: `gui-stderr.txt`, `shots\` (one
screenshot per second) and `shots\winstate.txt` (window-state changes).
