# GUI tray fix (KwaaiNetGUI#16 at 27773e0) on metro-win: retest 2

Run 2026-11-11 (local, UTC−8) by a Claude Code session at the keyboard of
metro-win. Reza was at the GUI. His answers are quoted word for word.

- Windows 11 Pro for Workstations 10.0.22631, NVIDIA RTX A5000, driver 581.42
- Build: KwaaiNetGUI#16 (`fix/windows-tray-icon`) at
  `27773e061942b256ac81ab18a495a0a94733d1fe`, CI run `43063051093`, artifact
  `kwaainet-gui-windows`. The PR head was checked before download.
- Follows `results/gui-v0.5.1-tray-metro-win.md` (head `ae1d5b63`) and
  KwaaiNetGUI#17.

## Summary

| Step | Result |
| --- | --- |
| 1. Baseline | **PASS**: kwaainet 0.7.3, node PID 6516 from `.cargo\bin`, shard and storage running |
| 2. Build | **PASS**: CI run at `27773e0`, `completed` / `success` |
| 3. Icons | **PASS**: tray, taskbar and title-bar icons are the KwaaiNet logo (cloud + tree), no white square (from screenshots) |
| 4a. X, then tray left-click | **FAIL** (no crash): restored the hidden window **once in about four tries**. Other tries either never reached the GUI or arrived as a right-click and opened the menu |
| 4b. X, then right-click → Open KwaaiNet… | **PASS**: the window came back from hidden 2/2 (10:17:31, 10:19:47), and the process survived |
| 5. Quit KwaaiNet | **PASS**: the GUI exited cleanly, the tray icon is gone, and **no APPCRASH**. The node stopped, which is expected in Built-in mode |
| 6. Restore | **PASS**: `kwaainet start --daemon --shard` brought back node 11252, shard 17292 and storage 18132, all from `.cargo\bin` |

**What 27773e0 fixed:**
- **No crash on restore.** About 15 tray events (3 left-click restores, 8 right-clicks, 2 Open KwaaiNet…, Quit) produced no crash. At `ae1d5b63` every left-click and Open crashed the GUI.
- **No crash on Quit.** Quit exits cleanly. At `ae1d5b63` the GUI crashed in `flutter_windows.dll` on Quit.
- **Logo.** The icon is now the KwaaiNet logo, with no white square.

**What remains:** a left-click on the tray icon is not a reliable way to get the window back once it is hidden (4a), and the tray menu does not close when focus moves away.

The release exe's stderr now carries the GUI's `[tray]`, `[window-…]` and
`[shutdown]` lines, which is what made 4a diagnosable.

## Method

- The GUI was started from `$env:USERPROFILE\kwaainet-gui-tray-test2\app` with
  stderr redirected to a file.
- Reza asked: "take snapshots as I click on the icons". A background script
  took a full-screen screenshot every second and logged every change in the
  GUI window's state (`IsWindowVisible`, `IsIconic`, foreground) to
  `winstate.txt`. A second script timestamped new GUI stderr lines.
- The screenshots stay on metro-win under `kwaainet-gui-tray-test2\shots*`.
  They are not committed, because they show the desktop.

## 1. Baseline

```
PS> kwaainet --version
kwaainet 0.7.3

PS> kwaainet status
  🟢 Status:  Running
  🔢 PID:     6516
  ⏱️  Uptime:  2d 1h 18m 7s
  🟢 Shard:   Running (PID 9560)
  🟢 Storage: Running (PID 19328, port 7432)

   Id ProcessName Path                                         StartTime
 6516 kwaainet    C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe 11/9/2026 8:09:28 AM
 9560 kwaainet    C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe 11/9/2026 8:09:28 AM
19328 kwaainet    C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe 11/9/2026 8:09:28 AM
```

## 2. Build

```
$ gh pr view 16 -R Kwaai-AI-Lab/KwaaiNetGUI --json headRefOid
27773e061942b256ac81ab18a495a0a94733d1fe
$ gh run list -R Kwaai-AI-Lab/KwaaiNetGUI -b fix/windows-tray-icon -w CI -L 1 --json databaseId,headSha,conclusion
[{"conclusion":"success","databaseId":43063051093,"headSha":"27773e061942b256ac81ab18a495a0a94733d1fe","status":"completed"}]
```

Changes since `ae1d5b63`, the head from retest 1 (`git log ae1d5b63..27773e0`):

```
1485056 fix(windows): KwaaiNet tray icon, no crash on restore or quit
27773e0 test(window): answer window_manager's is* queries in the channel mock
```

- `window_restore.dart` → `openMainWindowAtChat`: no `setSkipTaskbar` on
  Windows. That was the #17 null-`taskbar_` crash.
- `shutdown.dart` → `performQuit`: on Windows it uses `windowManager.close()`
  instead of `destroy()`, and takes a `beforeClose` hook that the tray uses to
  remove its icon.
- `close_handler.dart` → `onWindowClose`: during a quit, the close is let
  through instead of hiding to the tray.
- The `.ico` was regenerated (33,772 → 303,622 bytes, `tool/make_windows_ico.py`),
  and `setIcon` uses `isTemplate: !Platform.isWindows`.

Artifact expanded:

```
app\data\flutter_assets\assets\tray_kwaai_logo.ico     303622
app\kwaainet.exe                                     46624256
app\kwaainet_gui.exe                                   362496
```

No other `kwaainet_gui` was running. The GUI (PID 14256) was launched at
09:28:04. It attached to node 6516 and wrote `%TEMP%\tray_kwaai_logo.ico`
(303,622 bytes).

## 3. Icons

Reza did not answer this in words. He asked for screenshots instead. The
screenshot at 10:12:11 (`shots\shot-101211-950.png`), enlarged, shows:

- **Tray** (first in the `^` overflow): the cloud-and-tree logo, blue cloud
  and dark tree, on a transparent background. **No white square.**
- **Taskbar button and window title bar:** the same logo.
- The tray menu is open, reading: `Running (pid 6516)`, `Start service`
  (greyed), `Stop service`, `Open KwaaiNet…`, `Quit KwaaiNet`. Hovering the
  icon shows the tooltip `KwaaiNet — running (pid 6516)`.

**PASS.**

## 4. Close with X, then restore from the tray

Reza after the first round: "done". Then: "let's try again". After the window
had been hidden and he used the tray:

> yes - it comes back when I click the Open KwaaiNet menu item

After closing with X and left-clicking the tray icon:

> No, the KwaaiNet window does not appear simply by left clicking the overflow icon. I need to click the Open KwaaiNet... menu item.

He was then asked to left-click twice, then use Open KwaaiNet…. He said "done",
and then:

> the menu popped up and stayed up even when focus moved elsewhere

The GUI's own log, in order, matched with the window-state log:

```
before 10:15      [tray] icon mouseDown → opening window      (window visible)
                  [window-restore] window restored at chat
                  [tray] icon rightMouseDown → popping menu
                  [tray] icon mouseDown → opening window
                  [window-restore] window restored at chat
                  [tray] icon rightMouseDown → popping menu
10:15:00 hidden   [window-close] keepInTrayOnClose=true & tray enabled → hiding window + Dock icon
                  [tray] icon rightMouseDown → popping menu
                  [tray] menu click: key=show
10:17:31 visible  [window-restore] window restored at chat
10:18:21 hidden   [window-close] ... → hiding window + Dock icon
   (Reza's left-click: nothing logged)
                  [tray] icon rightMouseDown → popping menu
                  [tray] menu click: key=show
10:19:47 visible  [window-restore] window restored at chat
10:20:16 hidden   [window-close] ... → hiding window + Dock icon
                  [tray] icon rightMouseDown → popping menu
                  [tray] icon mouseDown → opening window
10:20:36 visible  [window-restore] window restored at chat
10:20:39 hidden   [window-close] ... → hiding window + Dock icon
10:21:09.418      [tray] icon rightMouseDown → popping menu     (Reza: left-click)
10:21:17.373      [tray] icon rightMouseDown → popping menu     (Reza: left-click)
```

The window-state log for the same period:

```
10:16:00.120 pid=14256 title='KwaaiNet' visible=False minimized=False foreground=False
10:17:31.190 pid=14256 title='KwaaiNet' visible=True  minimized=False foreground=True
10:18:21.713 pid=14256 title='KwaaiNet' visible=False minimized=False foreground=False
10:19:47.675 pid=14256 title='KwaaiNet' visible=True  minimized=False foreground=True
10:20:16.974 pid=14256 title='KwaaiNet' visible=False minimized=False foreground=False
10:20:36.163 pid=14256 title='KwaaiNet' visible=True  minimized=False foreground=True
10:20:39.183 pid=14256 title='KwaaiNet' visible=False minimized=False foreground=False
```

Screenshots at 10:21:09–10:21:24 show the tray menu open after each of the two
10:21 clicks. Windows logged no Application Error for `kwaainet_gui.exe` at any
point. PID 14256 ran until Quit.

### 4a. Left-click after X: FAIL (no crash)

- **Visible window:** a left-click restored the window, 2/2.
- **Hidden window:** a left-click restored it only once, at 10:20:36. The other
  tries went wrong in two ways:
  - The click was **not delivered** at all: after 10:18:21, no `mouseDown` was
    logged.
  - The click was **delivered as `rightMouseDown`**: at 10:21:09 and 10:21:17
    the menu popped instead of the window.

  Reza's account matches both: "does not appear simply by left clicking", and
  the menu "popped up".

The crash from #17 is gone. This is a different problem: which tray event
arrives for a left-click.

**Not established:** whether the remote session (AnyDesk) affects how the
clicks arrive. The "visible" left-clicks before 10:15 and the 10:20:36 one
came through the same session.

**Possible cause, not verified:** `tray_manager` 0.5.3 on Windows maps the
icon's mouse messages itself. The Windows 11 overflow flyout sends a click on
a hidden-window app's icon differently, for example as `NIN_SELECT` or
`WM_CONTEXTMENU` with `NOTIFYICON_VERSION_4`, or only on button-up. Reading
`tray_manager_plugin.cpp` → `HandleWindowProc` for which `lParam` values it
handles would settle this. An easy app-side mitigation: handle
`onTrayIconMouseUp` as well as `onTrayIconMouseDown`.

### Menu stays open when focus moves

Reza: "the menu popped up and stayed up even when focus moved elsewhere". This
is the classic Win32 tray-menu bug: `TrackPopupMenu` without the owner window
being made foreground first, or without the `WM_NULL` post afterwards.
`tray_manager`'s `PopUpContextMenu` owns this, so the fix may belong upstream.

### 4b. Open KwaaiNet… after X: PASS

2/2 from hidden (10:17:31, 10:19:47). No crash.

## 5. Quit KwaaiNet

Reza:

> kwaainet is not running. no icon in overflow or tray. did it crash?

It did not crash. The GUI's log:

```
[tray] menu click: key=quit
[tray] quit → performQuit
[shutdown] clean quit → stopping daemon via kwaainet stop
[daemon-state] stop() called
[daemon-controller] stop() invoked
[daemon-controller] running: C:\Users\METRO_WIN11\kwaainet-gui-tray-test2\app\kwaainet.exe stop
[session] Session inbound error: gRPC Error (code: 2, codeName: UNKNOWN, message: HTTP/2 error: Connection error: Connection is being forcefully terminated. (errorCode: 10), details: null, rawResponse: null, trailers: {})
[rpc] cancel of op 2 failed (ignored): SessionEndedError(localClose: session not open)
[window-close] close during quit → letting it through
```

The log file was last written at 10:22:14, and the window-state log shows the
process gone at 10:22:15.162.

```
PS> Get-Process | ? ProcessName -match 'kwaainet' | ft Id,ProcessName,Path
(nothing)

PS> kwaainet status
  🔴 Status:  Not running
  ⚫ Shard:   Not running
  🔴 Storage: Not running

PS> Get-WinEvent -LogName Application -MaxEvents 20 | ? { $_.Message -match 'kwaainet_gui' } | fl TimeCreated,Id,Message
(nothing)

PS> Get-WinEvent -FilterHashtable @{LogName='Application'; StartTime=(Get-Date).AddMinutes(-30)} | ? { $_.Message -match 'kwaainet' }
(nothing)
```

The `run\` directory after Quit holds `kwaainet.grpc`, `kwaainet.token`,
`relay_addr_cache.json`, `shard.pid` and `update_check.json`. `kwaainet.pid`,
`storage_serve.pid` and `start-args.json` are gone, which is `kwaainet stop`'s
footprint.

**PASS:** no `kwaainet_gui` is left, the tray icon is gone, there is no APPCRASH,
and the GUI's shutdown path ran to the end. The node, shard and storage
stopped, which is expected in Built-in mode (#17 item 2, unchanged by this PR).

## 6. Restore

`start-args.json` was gone, so the node was started with explicit flags. The
output was redirected to a fresh file through `cmd /c`, because a pipe or a
reused file is inherited and held by the daemon:

```
PS> cmd /c "kwaainet start --daemon --shard > %TEMP%\kw-start-<timestamp>.txt 2>&1"    # exit=0
PS> kwaainet status
  🟢 Status:  Running
  🔢 PID:     11252
  🟢 Shard:   Running (PID 17292)
  🟢 Storage: Running (PID 18132, port 7432)

11252 kwaainet C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe 11/11/2026 10:25:37 AM
17292 kwaainet C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe 11/11/2026 10:25:38 AM
18132 kwaainet C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe 11/11/2026 10:25:38 AM

start-args.json:  "shard": true,
```

**PASS.**

## Follow-ups

1. **#16 at 27773e0 fixes both crashes from #17 item 1 and item 3, and the
   icon.** From this run it is mergeable for those.
2. **Tray left-click on a hidden window is unreliable** (4a). It needs a look
   at `tray_manager`'s Windows event mapping. Handling the button-up event too
   may be enough.
3. **The tray menu does not close when focus moves.** This is
   `tray_manager`'s `PopUpContextMenu`, the foreground/`WM_NULL` pattern.
4. Unchanged from before: Quit stops a node the GUI did not start (#17 item 2).
   There is still no single-instance guard.
