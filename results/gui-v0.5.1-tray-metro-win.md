# GUI tray fix (KwaaiNetGUI#16) on metro-win: results

Run 2026-11-08 (local, UTC−8) by a Claude Code session at the keyboard of
metro-win. Reza was at the GUI over AnyDesk. His answers are quoted word for word.

- Windows 11 Pro for Workstations 10.0.22631, NVIDIA RTX A5000, driver 581.42
- Build under test: KwaaiNetGUI#16 (`fix/windows-tray-icon`) at
  `ae1d5b6302b2da5ba8bebd9219b7c215ceef761c`, CI run `42542729432`, artifact
  `kwaainet-gui-windows`. The PR head was checked before and after the CI run.
- Retest of the section 4 FAIL in `results/gui-v0.5.0-metro-win.md`, where no
  tray icon appeared.

## Summary

| Step | Result |
| --- | --- |
| 1. Baseline | **PASS**: node 0.7.3 running from `.cargo\bin`, PID 20744 |
| 2. Get the build | **PASS, with a deviation**: CI run concluded `failure`, but only `Build (linux)` failed. Windows build and tests passed |
| 3. Tray icon appears | **PASS**: icon appears (in the `^` overflow); `.ico` written and loadable |
| 4. Right-click menu | **PASS** (partly confirmed): menu opens and shows Quit KwaaiNet. The other entries were not confirmed |
| 5. Close with X, reopen from tray | **FAIL**: left-click on the tray icon **crashed the GUI** twice (access violation in `window_manager_plugin.dll`). Once it restored a minimized window correctly |
| 6. Quit KwaaiNet | **FAIL**: **Quit stopped the CLI-started node** (node, shard and storage), and the GUI then crashed on exit (`flutter_windows.dll`) instead of quitting cleanly |

The fix works: there is a tray icon on Windows now, and with it the menu and
Quit. Being able to use the tray exposed two problems that the earlier test could
not reach. **Left-clicking the tray icon can crash the GUI** (step 5). **Quit runs
`kwaainet stop` on a node the GUI did not start** (step 6).
That is by design in the current code (see step 6), and contradicts what the
GUI test plan expects ("The GUI did not start this node, so it must not stop
it").

## 1. Baseline

```
PS> kwaainet --version
kwaainet 0.7.3

PS> kwaainet status
  🟢 Status:  Running
  🔢 PID:     20744
  ⏱️  Uptime:  3d 11m 49s
  🟢 Shard:   Running (PID 17048)
  🟢 Storage: Running (PID 7300, port 7432)

PS> Get-Process | ? ProcessName -match 'kwaainet' | ft Id,ProcessName,Path,StartTime
   Id ProcessName Path                                         StartTime
   -- ----------- ----                                         ---------
 7300 kwaainet    C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe 11/5/2026 4:46:10 PM
17048 kwaainet    C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe 11/5/2026 4:46:10 PM
20744 kwaainet    C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe 11/5/2026 4:46:09 PM
```

No `kwaainet_gui` was running.

## 2. Get the build

```
$ gh run view 42542729432 -R Kwaai-AI-Lab/KwaaiNetGUI --json status,conclusion,headSha
{"conclusion":"failure","headSha":"ae1d5b6302b2da5ba8bebd9219b7c215ceef761c","status":"completed"}

Analyze & test   success
Build (windows)  success
Build (linux)    failure
Build (macos)    success
```

The Linux failure is in `tray_manager`'s own Linux plugin, not in this PR:

```
.../.plugin_symlinks/tray_manager/linux/tray_manager_plugin.cc:118:17:
  error: 'app_indicator_new' is deprecated [-Werror,-Wdeprecated-declarations]
```

The PR touches only `assets/tray_kwaai_logo.ico`, `lib/src/tray/tray.dart`,
`pubspec.yaml` and `test/tray_icon_asset_test.dart`. `Build (linux)` passed on
`main`'s last CI run (`42266740117`, 2026-11-06), so a runner image change
(newer libayatana-appindicator deprecating `app_indicator_new`) is the likely
cause. **That is a guess; nobody has re-run main to confirm it.** Waiting would not change
the run's result, so the Windows artifact was tested anyway. **The PR cannot go
green until the Linux build is fixed or the warning is suppressed**, and the
same failure will likely show on `main`'s next run.

```
PS> gh run download 42542729432 -R Kwaai-AI-Lab/KwaaiNetGUI -n kwaainet-gui-windows -D $env:USERPROFILE\kwaainet-gui-tray-test
PS> Expand-Archive ... -DestinationPath $gui\app
app\data\flutter_assets\assets\tray_kwaai_logo.ico     33772
app\kwaainet.exe                                    46624256
app\kwaainet_gui.exe                                   92672
```

`%TEMP%\tray_kwaai_logo.ico` was deleted before launch, so its presence later
shows that this build wrote it. The GUI was launched from `$gui\app`.

## 3. Tray icon

15 s after launch:

```
   Id ProcessName  Path                                                             StartTime
 7300 kwaainet     C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe                     11/5/2026 4:46:10 PM
17048 kwaainet     C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe                     11/5/2026 4:46:10 PM
20744 kwaainet     C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe                     11/5/2026 4:46:09 PM
 5420 kwaainet_gui C:\Users\METRO_WIN11\kwaainet-gui-tray-test\app\kwaainet_gui.exe 11/8/2026 5:02:57 PM

FullName                                                    Length LastWriteTime
C:\Users\METRO_WIN11\AppData\Local\Temp\tray_kwaai_logo.ico  33772 11/8/2026 5:03:01 PM
```

The GUI attached to the running node; it did not start its bundled one.

The same Win32 call `tray_manager` makes, run against the written file:

```
LoadImage(NULL, "%TEMP%\tray_kwaai_logo.ico", IMAGE_ICON, SM_CXSMICON, SM_CYSMICON, LR_LOADFROMFILE)
HICON=56492961   (valid; the PNG in the 0.5.0 test returned 0)
```

Reza:

> yes - the icon appeared. I was able to launch a second instance. The X button in the top right of the tray con aloowed me to exit both.

Despite "exit both", both GUIs were still running afterwards (the X hides the
window to the tray, since "keep running in tray" defaults to on):

```
 5420 kwaainet_gui ...\kwaainet-gui-tray-test\app\kwaainet_gui.exe 11/8/2026 5:02:57 PM
17552 kwaainet_gui ...\kwaainet-gui-tray-test\app\kwaainet_gui.exe 11/8/2026 5:17:09 PM
```

Reza then:

> now there are no icons

and, asked to check the `^` overflow:

> the icons are in the overflow

> two icons, menu shows Quit KwaaiNet

So each instance shows its own icon. Windows 11 places a new app's icon in the
overflow by default.

**PASS.** The icon appears on Windows, so the fix works. The second instance is
expected, because the PR leaves the single-instance guard out ("Not fixed here").

## 4. Right-click menu

Reza: "two icons, menu shows Quit KwaaiNet".

**PASS** for the menu opening and holding Quit KwaaiNet. Reza was asked whether
the status line, Start service, Stop service and Open KwaaiNet… were also
there; that question was not answered before the next step.

## 5. Close with X, reopen from tray

The X hid the window, as designed; both processes survived (step 3). Then:

> I clicked one icon in the overflow and it disappeared. On the otherone I rightclicked and the click Quit KwaaiNet and it disappeared. No icons now.

A left click should open the window (`tray.dart` → `onTrayIconMouseDown` →
`openMainWindowAtChat`). The Windows Application log shows that the left-clicked
GUI (PID 17552, `0x4490`) **crashed** at that moment:

```
11/8/2026 5:39:52 PM  1000 Application Error
  Faulting application name: kwaainet_gui.exe, version: 0.5.0.1
  Faulting module name: window_manager_plugin.dll
  Exception code: 0xc0000005
  Fault offset: 0x000000000000b2af
  Faulting process id: 0x0x4490
  Faulting application path: C:\Users\METRO_WIN11\kwaainet-gui-tray-test\app\kwaainet_gui.exe
```

The node log's last line (`17:39:53 ... grpc_server: Session: inbound recv
error: ... h2 protocol error`) is this GUI's session dropping when it crashed.

### Retest (one GUI, window state recorded)

Reza asked for step 5 to be repeated:

> can we repeat that test. I think that the left click simply cause the icon to disappear and no KwaaiNet winow appeared. Let's redo.

One GUI was started (PID 11704, `0x2DB8`, 17:48:12) with stderr redirected to a
file. A script polled its `FLUTTER_RUNNER_WIN32_WINDOW` every 0.5 s through
`EnumWindows`, `IsWindowVisible`, `IsIconic` and `GetForegroundWindow`, and
logged each change. The window was minimized from launch.

Reza, after a first round:

> Left click of the tray icon killed the KwaaiNet GUI window

The process and node were both still up at that point, with the window still
minimized. Second round:

> left click bring the KwaaiNet window to the top.

Third round (asked to close with the title-bar X, then left-click the overflow
icon):

> left click in overflow made the GUI window to disappear

Recorded window states:

```
17:50:15.312 pid=11704 title='KwaaiNet' visible=True minimized=True  foreground=False
17:50:46.244 pid=11704 title='KwaaiNet' visible=True minimized=False foreground=True    <- left-click restored it
17:50:51.121 pid=11704 title='KwaaiNet' visible=True minimized=False foreground=False
17:51:01.966 pid=11704 title='KwaaiNet' visible=True minimized=False foreground=True
17:51:04.679 pid=11704 title='KwaaiNet' visible=True minimized=False foreground=False
17:51:41.736 pid=11704 title='KwaaiNet' visible=True minimized=False foreground=False
17:51:42.842 NO GUI WINDOW/PROCESS
```

Application log:

```
11/8/2026 5:51:42 PM  1000 Application Error
  Faulting application name: kwaainet_gui.exe, version: 0.5.0.1
  Faulting module name: window_manager_plugin.dll
  Exception code: 0xc0000005
  Fault offset: 0x000000000000b2af
  Faulting process id: 0x0x2DB8
```

That is the same module and offset as at 17:39:52. The node (17964, shard
13956, storage 17020) was unaffected. The window never shows as hidden
(`visible=False`) in the recording. Either the X had not hidden it when the
click came, or it hid and crashed within one 0.5 s poll, so this run does not
show whether the crash needs a hidden window. The captured stderr holds none of
the GUI's `[tray]` log lines, only `[status-watcher]`, `[rpc]` and `[session]`
startup lines, so it does not show which window call faulted.

**FAIL.** Left-clicking the tray icon restored a minimized window once
(17:50:46). It crashed the GUI twice with the same access violation in
`window_manager_plugin.dll`, both times when the window was not minimized.
`openMainWindowAtChat` is the left-click path. It was unreachable on Windows
until #16, so this crash is older than #16 rather than caused by it.

## 6. Quit KwaaiNet

After Quit:

```
PS> Get-Process | ? ProcessName -match 'kwaainet' | ft Id,ProcessName,Path,StartTime
(nothing: no kwaainet_gui and no kwaainet)

PS> kwaainet status
  🔴 Status:  Not running
  ⚫ Shard:   Not running
  🔴 Storage: Not running

PS> Get-ChildItem $env:USERPROFILE\.kwaainet\run
kwaainet.grpc              5 11/5/2026 4:46:09 PM
kwaainet.token            64 11/5/2026 4:46:09 PM
relay_addr_cache.json    227 8/10/2026 12:32:10 PM
shard.pid                  5 11/5/2026 4:46:33 PM
update_check.json         44 11/7/2026 6:20:03 PM
```

`kwaainet.pid`, `storage_serve.pid` and `start-args.json` are gone. That is the
footprint of `kwaainet stop`, not of a crash. The node log ends without a
shutdown line:

```
2026-11-09T01:39:36.141358Z  INFO kwaainet::node_native: ✅ Announced 32 blocks
2026-11-09T01:39:53.588375Z  WARN kwaainet::grpc_server: Session: inbound recv error: status: Unknown, message: "h2 protocol error: error reading a body from connection", ...
```

**FAIL:** the GUI exited, but it stopped node 20744, which it did not start,
together with its shard (17048) and storage (7300).

The GUI also did not exit cleanly. After `kwaainet stop`, it crashed while
tearing down the window:

```
11/8/2026 5:40:09 PM  1000 Application Error
  Faulting module name: flutter_windows.dll   Exception code: 0xc0000005   Fault offset: 0x1e220
  Faulting process id: 0x0x152C   (= 5420, the first GUI)
11/8/2026 5:40:10 PM  1000 Application Error
  Faulting module name: flutter_windows.dll   Exception code: 0xc000041d   Fault offset: 0x1e220
  Faulting process id: 0x0x152C
```

The crash came after the stop, so it did not cause it.

### Cause

This is the GUI's designed behaviour, not something introduced by #16. At
`ae1d5b63`, `lib/src/tray/tray.dart` → `onTrayMenuItemClick`, case `'quit'`,
calls `performQuit`. `lib/src/window/shutdown.dart` → `performQuit` documents:

```dart
/// The single, shared "clean quit" routine used by both the window-close
/// handler and the tray Quit item.
/// ...
/// 2. Drive the daemon down via the normal transition path — which runs
///    `kwaainet stop`, so the CLI reaps the node and its detached children
```

and does:

```dart
  _log('clean quit → stopping daemon via kwaainet stop');
  // ... stop() is a no-op in external mode.
  await container.read(daemonTransitionProvider.notifier).stop();
```

In Built-in daemon mode, the default, Quit stops whatever node is running, even
one that the CLI started and the GUI only attached to. The 0.5.0 test plan
(section 4) requires the opposite. The 0.5.0 run never reached this code: there
was no tray icon, so the GUI was ended from Task Manager. Quitting by closing
the window with "keep running in tray" turned off takes the same path, so it
also stops the node.

Neither GUI was left afterwards because the second GUI (17552) had already
crashed on the left-click (step 5) before Quit was used on the first. With two
instances, Quit in one would in any case stop the node both are attached to.

### Restore

`kwaainet stop` deletes `start-args.json`, so it was restored from the 0.5.0
test backup first, so that `restart` brings the node back with `--shard`:

```
PS> Copy-Item $env:USERPROFILE\kwaainet-guitest-backup\start-args.json $env:USERPROFILE\.kwaainet\run\start-args.json
PS> cmd /c "kwaainet restart > %TEMP%\kw-restart.txt 2>&1"     # exit 0
PS> kwaainet status
  🟢 Status:  Running
  🔢 PID:     17964
  🟢 Shard:   Running (PID 13956)
  🟢 Storage: Running (PID 17020, port 7432)

13956 C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe 11/8/2026 5:42:39 PM
17020 C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe 11/8/2026 5:42:39 PM
17964 C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe 11/8/2026 5:42:39 PM
```

The node runs from `.cargo\bin` with shard and storage, as before the test, but
under new PIDs. The backed-up `start-args.json` is the one from 2026-11-05
(`shard: true`, all else default), the same flags the node had.

## Follow-ups

1. **#16 fixes the tray icon on Windows.** It's ready to merge on that count,
   once the unrelated Linux CI failure is dealt with.
2. **Quit stops a node the GUI did not start.** New issue: the GUI should only
   stop a node it spawned itself, or ask first. When it does stop the node,
   it also deletes the CLI's `start-args.json`, so a later `kwaainet restart`
   loses the user's flags (here `--shard`).
3. **Single instance** is still open, as the PR states. With the node-stopping
   Quit, a second instance's Quit takes the first instance's node with it.
4. **Left-click on the tray icon crashes the GUI** in `window_manager_plugin.dll`
   (`0xc0000005` at `+0xb2af`, seen twice). Path: `tray.dart` →
   `onTrayIconMouseDown` → `openMainWindowAtChat`.
5. **The GUI crashes on exit** in `flutter_windows.dll` (`+0x1e220`) after a
   tray Quit.
6. A Windows debug build, or a release build that logs to a file (stderr from
   the release exe holds no `[tray]` lines), is needed to see which
   `windowManager` call faults.
