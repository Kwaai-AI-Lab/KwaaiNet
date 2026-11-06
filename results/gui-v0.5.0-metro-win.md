# KwaaiNet GUI (pre-v0.5.0) on metro-win — results

Run 2026-11-05, 13:44–14:06 local (UTC−8), by a Claude Code session at
metro-win, following `TEST-GUI-ON-METRO-WIN.md`.

- Windows 11 Pro for Workstations 10.0.22631
- NVIDIA RTX A5000, 24564 MiB, driver 581.42
- Build under test: KwaaiNetGUI CI run `41958484345` on `test/bundle-cuda-full`
  (KwaaiNetGUI#12), head `8e0b07285948b6427766e9738b4bb8e446599b72`,
  completed/success

## How the 👀 checks were done

Reza asked for testing to be completed without answering the 👀 questions one
by one. **No 👀 item has an answer from Reza in his words.** Where a check
could be seen, it comes from screenshots this session took of the desktop, and
is labelled *(screenshot)*. Reza did use the GUI during sections 2–3: the two
chat messages below are his.

Reza also told the session to go ahead, which it took as the go-ahead for
section 6, which the doc says to ask before.

The session cannot reach the tray menu. Where the doc says *quit from the
tray*, the session first sent the window a normal close, then ended the
`kwaainet_gui` process. **Ending the process does not run the GUI's quit
path**, so the tray-quit checks (sections 4, 5 and the end of 6) are recorded
as NOT TESTED, with what the process kill showed.

## Before testing: the machine was not in the doc's state

Not part of the doc, recorded so the baseline is clear:

- A PR #12 GUI from an earlier session was still open (from
  `Downloads\kwaainet-gui-pr12-8e0b072`, same CI run). Reza asked for it to be
  quit; the session ended its process. The node kept running.
- The node had been restarted by Reza at 2026-11-04 19:15 with **`--shard`**
  (`start-args.json` `shard: true`, 18.3 GB on the GPU). Reza confirmed this is
  the intended baseline.

## 0. Before

```
PS> kwaainet --version
kwaainet 0.7.3
PS> kwaainet status
  🟢 Status:  Running
  🔢 PID:     20336
  ⏱️  Uptime:  18h 31m 17s
  💻 CPU:     0.0%
  🧠 Memory:  220 MB

  🟢 Shard:   Running (PID 16988)

  🟢 Storage: Running (PID 11748, port 7432)
PS> (Get-Command kwaainet).Source
C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe
PS> "HOME=$env:HOME  USERPROFILE=$env:USERPROFILE  KWAAINET_HOME=$env:KWAAINET_HOME"
HOME=C:\Users\METRO_WIN11  USERPROFILE=C:\Users\METRO_WIN11  KWAAINET_HOME=
PS> Get-ChildItem $env:USERPROFILE\.kwaainet\run
LastWriteTime         Length Name
-------------         ------ ----
11/4/2026 7:15:07 PM       5 kwaainet.grpc
11/4/2026 7:15:07 PM       5 kwaainet.pid
11/4/2026 7:15:07 PM      64 kwaainet.token
8/10/2026 12:32:10 PM    227 relay_addr_cache.json
11/4/2026 7:15:30 PM       5 shard.pid
9/11/2026 7:18:08 PM       0 shard.ready
11/4/2026 7:15:16 PM       5 shard_local.port
11/4/2026 7:15:07 PM     250 start-args.json
11/4/2026 7:15:08 PM       5 storage_serve.pid
11/4/2026 6:07:26 PM    2862 update_check.json
PS> Get-Content $env:USERPROFILE\.kwaainet\run\start-args.json
{ "model": null, "blocks": null, "port": null, "grpc_port": null,
  "no_gpu": false, "public_name": null, "public_ip": null,
  "announce_addr": null, "identity_key": null, "no_relay": false,
  "shard": true, "no_contribute": false }
PS> nvidia-smi --query-gpu=name,memory.used,memory.total --format=csv
name, memory.used [MiB], memory.total [MiB]
NVIDIA RTX A5000, 18351 MiB, 24564 MiB
PS> Get-Process | Where-Object { $_.ProcessName -match 'kwaainet' } | Format-Table Id,ProcessName,Path
   Id ProcessName Path
11748 kwaainet    C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe
16988 kwaainet    C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe
20336 kwaainet    C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe
```

Process roles: 20336 `run-node --shard`, 16988 `shard serve --auto-rebalance`,
11748 `storage serve`.

`HOME` is set but equals the user profile, so the GUI looks in the right place.

Backup to `%USERPROFILE%\kwaainet-guitest-backup` (folder did not exist
before):

```
config.yaml       1969  sha256 1A7F2972482CA7170C5618B2DE492F21D86B3157AAC6E7F66B9F262FEE02C1BA
start-args.json    250  sha256 96FA4ABD885C06F42F7EBFADA653E1FE0832F1C2A6F2E81B65339E4D8DFE828A
```

## 1. Get the build

```
PS> gh run list -R Kwaai-AI-Lab/KwaaiNetGUI -b test/bundle-cuda-full -w CI -L 1
completed  success  test(ci): bundle CUDA-full kwaainet in the Windows build — DO NOT MERGE  CI  test/bundle-cuda-full  pull_request  41958484345  7m22s  2026-11-05T02:57:13Z
PS> gh run download 41958484345 -R Kwaai-AI-Lab/KwaaiNetGUI -n kwaainet-gui-windows -D $gui
C:\Users\METRO_WIN11\kwaainet-gui-test\kwaainet-gui-windows.zip   1032615867
PS> (expand) Get-ChildItem $gui\app -Recurse -Include *.exe
C:\Users\METRO_WIN11\kwaainet-gui-test\app\kwaainet.exe       58590208
C:\Users\METRO_WIN11\kwaainet-gui-test\app\kwaainet_gui.exe      92672
PS> & ...\app\kwaainet.exe --version
kwaainet 0.7.3
PS> Get-ChildItem $app -Filter *64_1*.dll
cublas64_12.dll          100796416
cublasLt64_12.dll        494290944
cudart64_12.dll             556544
cufft64_11.dll           275258368
curand64_10.dll           63279104
cusolver64_11.dll        116769792
cusparse64_12.dll        288743424
nvrtc-builtins64_126.dll   5345280
nvrtc64_120_0.alt.dll     45534208
nvrtc64_120_0.dll         45474304
```

The bundled `kwaainet.exe` is byte-identical (SHA-256) to the 0.7.3 that the
auto-updater installed in `.cargo\bin` (checked 2026-11-04).

## 2. Launch against the running node

GUI launched 13:48:34 from `C:\Users\METRO_WIN11\kwaainet-gui-test\app`.
After 30 s:

```
PS> Get-Process | Where-Object { $_.ProcessName -match 'kwaainet' } | Format-Table Id,ProcessName,Path,StartTime
   Id ProcessName  Path                                                        StartTime
11748 kwaainet     C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe                11/4/2026 7:15:08 PM
16988 kwaainet     C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe                11/4/2026 7:15:08 PM
20336 kwaainet     C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe                11/4/2026 7:15:07 PM
 8828 kwaainet_gui C:\Users\METRO_WIN11\kwaainet-gui-test\app\kwaainet_gui.exe 11/5/2026 1:48:34 PM
PS> Get-Content $env:USERPROFILE\.kwaainet\run\kwaainet.pid
20336
PS> kwaainet status
  🟢 Status:  Running
  🔢 PID:     20336
  ⏱️  Uptime:  18h 33m 57s
  🟢 Shard:   Running (PID 16988)
  🟢 Storage: Running (PID 11748, port 7432)
```

Exactly one node, same PID as section 0, from `.cargo\bin`; the bundled binary
was not started. The GUI attached.

👀 Status header: **not answered by Reza.** *(screenshot, 13:58, second
launch)* Header reads `Running • pid 20336 • v0.7.3`, with Start greyed and
Stop red; no red error bar visible.

👀 Settings → daemon mode: **not answered by Reza.** *(screenshot)* "KwaaiNet
binary location" has **Use built-in** selected. "Use system" shows
`Binary found at: C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe`. Nothing was
changed or applied.

## 3. Every tab talks to the node

👀 Tab-by-tab report: **not answered by Reza.** What the session could
establish:

- **Chat works end to end.** *(screenshots)* Reza sent two messages of his own
  (not the doc's `Say hello in five words.`):
  - "what is the capital of france?" → "The capital of France is Paris."
  - "is that your real hair?" → "I am a computer program and do not have a
    physical body, so I do not have hair. I exist solely as a digital entity,
    and I communicate with you through text. I'm here to help answer your
    questions and provide information, but I don't have a physical
    appearance. Is there anything else I can help with?"

  While the second was pending, the GUI showed a yellow bar, **"Waiting on the
  daemon — 30s so far."**, not an error. The inference panel / routing
  display was not seen in any screenshot.
- **Tabs exist** *(screenshot)*: Status, Sharding, VPK, Peers, DHT,
  Contribute, Network, Appearance, Experimental, Developer, About. Peers,
  DHT, Sharding/map, Settings values and RAG were not looked at tab by tab.
  The doc's table has no VPK, Contribute, Network or Developer row, and lists
  RAG, which was not visible in the sidebar (presumably behind
  Experimental → RAG, which was not turned on).
- On first launch the window opened on the chat view, with only a settings
  gear; the tab sidebar appeared under Settings.

```
PS> nvidia-smi --query-gpu=name,memory.used,utilization.gpu --format=csv
name, memory.used [MiB], utilization.gpu [%]
NVIDIA RTX A5000, 18446 MiB, 13 %
PS> kwaainet logs | Select-String -Pattern 'unauth|token|denied|error' | Select-Object -Last 20
2026-11-05T21:46:11.388966Z  WARN kwaainet::grpc_server: Session: inbound recv error: status: Unknown, message: "h2 protocol error: error reading a body from connection", details: [], metadata: MetadataMap { headers: {} }
2026-11-05T21:49:49.394305Z  INFO kwaai_inference::tokenizer: Loaded tokenizer.json: vocab=128256, bos=Some(128000), eos=Some(128009)
2026-11-05T21:55:25.546673Z  INFO kwaai_inference::tokenizer: Loaded tokenizer.json: vocab=128256, bos=Some(128000), eos=Some(128009)
```

- **No `unauth`, `denied` or token-rejection line.** The only `token` hits are
  tokenizer loads, one per chat request (13:49:49 and 13:55:25 local).
- The `h2 protocol error` WARN at 13:46:11 local is before this GUI launched
  (13:48:34). It matches the session ending the earlier GUI's process, which
  dropped its gRPC Session stream mid-read.

GPU, sampled every second (nvidia-smi) from launch to the end of section 3:

| Window (local) | What | util max | util mean | power max | power mean |
|---|---|---|---|---|---|
| 13:48:35–13:49:45 | idle, GUI up | 24 % | 6.8 % | 15.9 W | 14.6 W |
| 13:49:45–13:51:30 | chat 1 | 44 % | 6.9 % | 98.3 W | 17.9 W |
| 13:51:30–13:55:20 | between | 35 % | 2.9 % | 20.0 W | 13.6 W |
| 13:55:20–13:57:10 | chat 2 | 64 % | 15.2 % | 174.9 W | 24.9 W |

GPU utilisation did move during chat, in short bursts: about 1 s at 13:50:38
(44 %, 98 W) for answer 1, and about 5 s at 13:56:32–36 (up to 64 %, 175 W)
for answer 2. Each burst came about **50 s (chat 1) and 67 s (chat 2) after
the request** reached the node; the GPU was idle for that wait. The
node logged `Re-announcing to DHT (shard_ready=true, whole_model_ready=false)`
at 13:51:54 and 13:56:43, although it announces all 32 blocks.

## 4. Quit the GUI

**NOT TESTED as written** — the session cannot use the tray menu, and Reza
was not asked to. What was done instead:

```
GUI PID 8828, CloseMainWindow -> True
after window close:
   Id ProcessName  Path
11748 kwaainet     C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe
16988 kwaainet     C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe
20336 kwaainet     C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe
 8828 kwaainet_gui C:\Users\METRO_WIN11\kwaainet-gui-test\app\kwaainet_gui.exe
GUI still alive after window close (hid to tray). Ending process.
PS> Get-Process kwaainet
11748 kwaainet    C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe
16988 kwaainet    C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe
20336 kwaainet    C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe
PS> kwaainet status
  🟢 Status:  Running
  🔢 PID:     20336
  ⏱️  Uptime:  18h 42m 34s
  🟢 Shard:   Running (PID 16988)
  🟢 Storage: Running (PID 11748, port 7432)
```

- Closing the window keeps the GUI in the tray, as its setting "Keep running
  in tray when window is closed" (on) says.
- After the GUI process was killed, the node kept running with the same PID.
  That shows the node does not depend on the GUI process; it does **not** show
  that the GUI's Quit leaves it running.

### 4b. Tray-quit retry with Reza (16:29–16:40) — FAIL: no tray icon

Reza retried the tray quit himself. Node before: PID 5612 (`run-node
--shard`, from `.cargo\bin`, started 14:00:45 at restore), shard 20752,
storage 12012. GUI launched by the session at 16:29:03 (PID 20816).

Reza's answers, word for word:

1. "there is no quit option when I right click the tray icon"
2. "I clicked the X of the thumbnail that appears when I rightclicked the tray
   icon" — this was the taskbar button's window preview, not a tray menu.
   Both GUI processes were still alive afterwards (the window had hidden).
3. "I didnt find a KwaaiNet logo. Is the GUI running? Let's try again."
4. After a clean relaunch (one GUI, PID 13064) and with the notification-area
   overflow open: "no icon."
5. "I see only one in the task manager" (one `kwaainet_gui`, PID 13064).

What the session observed:

- **No KwaaiNet tray icon, in the taskbar or in the overflow.** Screenshot of
  the open overflow at 16:36 shows seven icons (AnyDesk, Windows Security,
  NVIDIA, volume, a USB device, two others), none of them KwaaiNet.
- So **Quit KwaaiNet is unreachable on Windows.** The menu exists in code
  (`lib/src/tray/tray.dart` → `_rebuildMenu`: status line, Start service,
  Stop service, Open KwaaiNet…, Quit KwaaiNet), but no icon is ever shown to
  open it.
- **Two GUIs can run at once.** At 16:29:30, 27 s after the session's launch,
  a second `kwaainet_gui` (PID 18720, parent `explorer.exe`) started from the
  same exe. Both ran side by side with no window; neither prevented the other.
  The session ended both before the clean retry.
- The GUI did write its tray image: `AppData\Local\Temp\tray_kwaai_logo.png`,
  1410 bytes, 16:29:33.
- Node PID 5612 and its shard and storage children ran throughout; no GUI
  action touched them.

**Likely cause, from reading the code (not verified on this machine):**
`tray.dart` → `init()` hands the tray a PNG on every platform:

```dart
_iconPath = await _materializeIcon('assets/tray_kwaai_logo@2x.png', 'tray_kwaai_logo.png');
await trayManager.setIcon(_iconPath!, isTemplate: true);
```

`tray_manager` on Windows expects an `.ico`, and a PNG fails silently. The
comment above the call speaks only of the macOS menu bar. The repo already
has `windows/runner/resources/app_icon.ico`.

**Consequence:** "Keep running in tray when window is closed" is on by default,
so on Windows the window's X leaves an invisible GUI with no way back and no
Quit, short of Task Manager; launching again starts a second copy.

## 5. Relaunch and attach again

Relaunched 13:57:51; after 30 s:

```
   Id ProcessName  Path                                                        StartTime
11748 kwaainet     C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe                11/4/2026 7:15:08 PM
16988 kwaainet     C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe                11/4/2026 7:15:08 PM
20336 kwaainet     C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe                11/4/2026 7:15:07 PM
12000 kwaainet_gui C:\Users\METRO_WIN11\kwaainet-gui-test\app\kwaainet_gui.exe 11/5/2026 1:57:51 PM
PS> Get-Content kwaainet.pid
20336
PS> kwaainet status
  🟢 Status:  Running
  🔢 PID:     20336
  ⏱️  Uptime:  18h 43m 15s
  🟢 Shard:   Running (PID 16988)
  🟢 Storage: Running (PID 11748, port 7432)
```

`run\` was identical before and after the relaunch (same files, sizes,
timestamps). It attached to the same node again; nothing different from the
first run except that it opened on Settings. 👀 **not answered by Reza**;
*(screenshot)* header `Running • pid 20336 • v0.7.3`. Then the GUI process
was ended, as in section 4; the node stayed up.

## 6. Negative variant: no node running

```
PS> kwaainet stop
INFO kwaainet::daemon: Stopping daemon PID 20336
  ✅ KwaaiNet daemon stopped
PS> kwaainet status
  🔴 Status:  Not running
  ⚫ Shard:   Not running
  🔴 Storage: Not running
PS> Get-Process kwaainet
(none)
PS> Get-ChildItem $env:USERPROFILE\.kwaainet\run
11/4/2026 7:15:07 PM       5 kwaainet.grpc
11/4/2026 7:15:07 PM      64 kwaainet.token
8/10/2026 12:32:10 PM    227 relay_addr_cache.json
11/4/2026 7:15:30 PM       5 shard.pid
11/4/2026 6:07:26 PM    2862 update_check.json
PS> nvidia-smi --query-gpu=memory.used,utilization.gpu --format=csv
665 MiB, 16 %
```

`kwaainet stop` removed `start-args.json`, `kwaainet.pid`,
`storage_serve.pid`, `shard_local.port` and `shard.ready`, and left
`kwaainet.grpc`, `kwaainet.token` and a stale `shard.pid`.

GUI launched 13:59:03; after 60 s:

```
PS> Get-Process | ... Format-Table Id,ProcessName,Path,StartTime
   Id ProcessName  Path                                                        StartTime
13724 kwaainet     C:\Users\METRO_WIN11\kwaainet-gui-test\app\kwaainet.exe     11/5/2026 1:59:08 PM
21588 kwaainet     C:\Users\METRO_WIN11\kwaainet-gui-test\app\kwaainet.exe     11/5/2026 1:59:09 PM
19248 kwaainet_gui C:\Users\METRO_WIN11\kwaainet-gui-test\app\kwaainet_gui.exe 11/5/2026 1:59:03 PM
command lines:
13724: "C:\Users\METRO_WIN11\kwaainet-gui-test\app\kwaainet.exe" run-node
21588: "C:\Users\METRO_WIN11\kwaainet-gui-test\app\kwaainet.exe" storage serve
PS> kwaainet status
  🟢 Status:  Running
  🔢 PID:     13724
  ⏱️  Uptime:  56s
  ⚫ Shard:   Not running
  🟢 Storage: Running (PID 21588, port 7432)
PS> Get-ChildItem $env:USERPROFILE\.kwaainet\run
11/5/2026 1:59:08 PM       6 kwaainet.grpc
11/5/2026 1:59:08 PM       5 kwaainet.pid
11/5/2026 1:59:08 PM      64 kwaainet.token
8/10/2026 12:32:10 PM    227 relay_addr_cache.json
11/4/2026 7:15:30 PM       5 shard.pid
11/5/2026 1:59:08 PM     251 start-args.json
11/5/2026 1:59:09 PM       5 storage_serve.pid
11/4/2026 6:07:26 PM    2862 update_check.json
PS> Get-Content start-args.json
{ "model": null, "blocks": null, "port": null, "grpc_port": null,
  "no_gpu": false, "public_name": null, "public_ip": null,
  "announce_addr": null, "identity_key": null, "no_relay": false,
  "shard": false, "no_contribute": false }
PS> (Get-Acl ...\kwaainet.token).Access
IdentityReference           FileSystemRights AccessControlType IsInherited
DESKTOP-6LL31ML\METRO_WIN11      FullControl             Allow       False
AreAccessRulesProtected: True
PS> nvidia-smi --query-gpu=memory.used,utilization.gpu --format=csv
833 MiB, 6 %
PS> kwaainet logs | Select-String -Pattern 'cuda|gpu|cpu fallback' | Select-Object -Last 20
(no lines)
```

Recorded, not judged:

- **Binary:** the bundled one, `kwaainet-gui-test\app\kwaainet.exe`
  (`run-node` + `storage serve`). Not `.cargo\bin`.
- **GUI shows it running / Chat answers:** 👀 **not answered by Reza**; not
  seen in a screenshot. Chat was not tried in this section.
- **GPU:** **not exercised.** The GUI started the node **without `--shard`**
  (`start-args.json` `shard: false`), so it loaded no model. GPU memory stayed
  at idle (max 851 MiB over the section, power max 66 W), and the logs have no
  CUDA, GPU or CPU-fallback line at all. This is neither a GPU pass nor a CPU
  fallback.
- **start-args.json:** rewritten by the GUI's start, `shard: true` → `shard:
  false`. Size 250 → 251 bytes. `kwaainet.grpc` also went 5 → 6 bytes (a
  different gRPC port).
- **Token ACL:** exactly one entry, current user, FullControl, Allow;
  inheritance off. As required.
- **Windows Firewall prompt** *(screenshot, 14:00)*: "Do you want to allow
  public and private networks to access this app? — kwaainet — Publisher:
  Unknown — Windows Firewall has blocked some features of kwaainet on all
  public and private networks." Firewall rules are per executable path, so
  the bundled node at a new path triggers it even though `.cargo\bin`'s
  binary is already allowed. **The session did not answer it**; that is Reza's
  call. A fresh v0.5.0 user would see this on first start, with "Publisher:
  Unknown".

Stopping (**NOT TESTED as written** — "stop the node from the GUI" and the
tray quit need Reza):

```
after ending the GUI process (not a GUI stop):
13724 kwaainet    C:\Users\METRO_WIN11\kwaainet-gui-test\app\kwaainet.exe
21588 kwaainet    C:\Users\METRO_WIN11\kwaainet-gui-test\app\kwaainet.exe
PS> kwaainet stop      (the .cargo\bin CLI)
INFO kwaainet::daemon: Stopping daemon PID 13724
  ✅ KwaaiNet daemon stopped
PS> Get-Process kwaainet
(none)
```

Killing the GUI leaves the node it started running (expected: the node is a
detached daemon). The `.cargo\bin` CLI stopped the bundled node and its
storage child cleanly, with no orphan.

### Restore

```
config.yaml unchanged during test: True    (hash equal to the backup)
(copied config.yaml and start-args.json back from the backup)
PS> kwaainet restart
  ✅ KwaaiNet daemon restarted (PID 5612)
  ✅ Shard serving: the daemon starts and supervises it
  ✅ Storage serving: the daemon starts and supervises it
PS> kwaainet status
  🟢 Status:  Running
  🔢 PID:     5612
  🟢 Shard:   Running (PID 20752)
  🟢 Storage: Running (PID 12012, port 7432)
PS> (Get-Process kwaainet).Path
C:\Users\METRO_WIN11\.cargo\bin\kwaainet.exe
start-args matches backup: True
```

Command lines: `run-node --shard`, `shard serve --auto-rebalance`,
`storage serve`. GPU back to 18420 MiB. The node is as in section 0, with a
new PID.

`kwaainet restart` started the daemon at once, but the session's command did
not return: run with its output piped (`| Tee-Object`), it hung until moved to
the background after 300 s. Most likely the detached daemon inherits the
console pipe. That only matters to scripts that capture `restart`'s output,
not to the node.

## Verdicts

- **0. Before: PASS.** CLI node running from `.cargo\bin` (with `--shard`, the
  baseline Reza confirmed), `HOME` = profile, files backed up.
- **1. Get the build: PASS.** Run `41958484345` completed/success; bundled
  `kwaainet 0.7.3` with the CUDA 12 DLLs beside it.
- **2. Launch against the running node: PASS on processes; 👀 not answered.**
  Same PID 20336, no second node. *(Screenshot)* `Running • pid 20336 •
  v0.7.3`, Use built-in, no red bar.
- **3. Every tab: PARTIAL — 👀 not answered.** Chat PASS: two answers over the
  authenticated channel, no auth error in logs, GPU used in bursts. Peers,
  DHT, Sharding/map, Settings and RAG were not checked tab by tab.
- **4. Quit the GUI: FAIL.** No tray icon on Windows, so there is no tray menu
  and no Quit (section 4b, Reza: "no icon."). Killing the GUI process left the
  node running with the same PID.
- **5. Relaunch: PASS on processes; tray quit impossible (see 4).** Attached to the
  same PID again; no state left behind.
- **6. No node running: PARTIAL.** GUI started the bundled node (PASS); token
  ACL correct (PASS); no orphan after stop (PASS, but stopped via CLI, not the
  GUI). GPU **not exercised**: the GUI starts the node without a shard.
  Chat and "stop from GUI" NOT TESTED; tray quit impossible (see 4).
- **Restore: PASS.**

## Surprising, even where it passed

0. **No tray icon on Windows** (section 4b). With the default "keep running in
   tray", closing the window strands an invisible GUI with no Quit; launching
   again runs a second copy, since nothing enforces a single instance. Likely
   a PNG handed to `tray_manager`, which wants an `.ico` on Windows.
1. **The GUI reports itself as 0.4.0.** Settings → Updates: "KwaaiNet 0.4.0 is
   up to date." This build is meant to become v0.5.0, so its version string
   has not been bumped. With "Download updates automatically" on, it is worth
   checking that a 0.4.0-labelled build cannot be "updated" to the real
   v0.4.0 release.
2. **Windows Firewall prompt on first start of the bundled node**, "Publisher:
   Unknown" (section 6). Every fresh Windows user will meet it.
3. **The GUI starts its own node without `--shard`**, so on a GPU machine a
   GUI-only user contributes no compute and never touches CUDA. That also
   makes section 6's GPU check impossible as written.
4. **Chat latency is mostly waiting, not generating:** about 50–67 s with the
   GPU idle before a 1–5 s burst of work, on a node that holds all 32 blocks
   itself yet reports `whole_model_ready=false`.
5. **`kwaainet stop` deletes `start-args.json`.** A later `kwaainet restart`
   would then have no recorded flags to reuse; the GUI's start wrote a new
   one with `shard: false`. Here the backup covered it.
6. The doc's tab table does not match the GUI: there are VPK, Contribute,
   Network and Developer tabs it does not list, and no RAG tab unless an
   experimental switch is on.
