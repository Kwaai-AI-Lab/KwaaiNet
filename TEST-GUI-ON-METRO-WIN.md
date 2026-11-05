# Testing KwaaiNet GUI (pre-v0.5.0) on metro-win

**Instructions for a Claude Code session running on metro-win.** This branch,
`test/gui-v0.5.0-metro-win`, exists only to carry this file and your results.
It is never merged.

The GUI is about to be released as v0.5.0. It is the first GUI that bundles a
kwaainet node new enough for its RAG, DHT and inference tabs (v0.7.3), and the
first that sends the daemon's gRPC bearer token (KwaaiNetGUI#8). An older GUI
cannot talk to a v0.7.3 daemon at all, so a GUI that can is the last gate
before KwaaiNet 0.8.0.

You are testing the **CI build of KwaaiNetGUI#12**. That is GUI `main` (the
merge of #11) with one test-only change: the Windows build bundles kwaainet
v0.7.3's `-cuda-full` build, so the GUI's own node can use this machine's GPU.
The release itself will not bundle the CUDA build, because it cannot start
without an NVIDIA driver. Everything else is what v0.5.0 will ship.

You are checking four things:

1. The GUI finds the node that is already running on this machine and attaches
   to it, rather than starting a second one.
2. It authenticates. Every tab loads data, with no `unauthenticated`,
   `permission denied` or "old daemon" message.
3. Quitting the GUI leaves the CLI-started node running.
4. With no node running, the GUI starts its own from the bundled binary, and
   stops it cleanly (section 6).

## Rules

- **Do not change config.yaml** and do not press **Apply** in Settings. Only
  look at the Settings page.
- **Do not edit code, rebuild, or reinstall kwaainet.** If something fails,
  record it and carry on with the next section if it is independent, else stop.
- **Do not ask for or use IP addresses or hostnames.**
- **Do not force-push.** Push only to this branch.
- You cannot see the GUI window. The person at the keyboard (Reza, over
  AnyDesk) can. Each check marked **👀** is one to ask them about, and record
  their answer word for word.
- If something is unclear or looks dangerous, stop and ask.

## 0. Before

```powershell
kwaainet --version                     # expect: kwaainet 0.7.3
kwaainet status
(Get-Command kwaainet).Source
"HOME=$env:HOME  USERPROFILE=$env:USERPROFILE  KWAAINET_HOME=$env:KWAAINET_HOME"
Get-ChildItem $env:USERPROFILE\.kwaainet\run
Get-Content $env:USERPROFILE\.kwaainet\run\start-args.json
nvidia-smi --query-gpu=name,memory.used,memory.total --format=csv
Get-Process | Where-Object { $_.ProcessName -match 'kwaainet' } | Format-Table Id,ProcessName,Path
```

Back up the two files section 6 may disturb, so you can put them back:

```powershell
$bk = "$env:USERPROFILE\kwaainet-guitest-backup"
New-Item -ItemType Directory -Force $bk | Out-Null
Copy-Item $env:USERPROFILE\.kwaainet\config.yaml $bk\
Copy-Item $env:USERPROFILE\.kwaainet\run\start-args.json $bk\
```

The GUI looks for the node's state in `%HOME%\.kwaainet` when `HOME` is set,
and only otherwise in `%USERPROFILE%\.kwaainet`. If `HOME` is set and points
anywhere other than your user profile, stop and tell Reza: the GUI would look in
the wrong place.

The node must be running at this point. If it is not, start it with
`kwaainet restart` and record that you had to.

## 1. Get the build

```powershell
$gui = "$env:USERPROFILE\kwaainet-gui-test"
gh run list -R Kwaai-AI-Lab/KwaaiNetGUI -b test/bundle-cuda-full -w CI -L 1
$run = gh run list -R Kwaai-AI-Lab/KwaaiNetGUI -b test/bundle-cuda-full -w CI -L 1 --json databaseId --jq '.[0].databaseId'
gh run download $run -R Kwaai-AI-Lab/KwaaiNetGUI -n kwaainet-gui-windows -D $gui
Get-ChildItem $gui -Recurse -Depth 1
```

The artifact is a zip inside the download directory. Expand it, then find the
GUI exe and the bundled node next to it:

```powershell
Get-ChildItem $gui -Filter *.zip | ForEach-Object { Expand-Archive $_.FullName -DestinationPath $gui\app -Force }
Get-ChildItem $gui\app -Recurse -Include *.exe | Format-Table FullName,Length
& (Get-ChildItem $gui\app -Recurse -Filter kwaainet.exe | Select-Object -First 1).FullName --version
```

The run must be `completed` / `success`; if it is still running, wait (the
artifact is about 1 GB). Expect the bundled `kwaainet.exe` to report **0.7.3**,
and the CUDA DLLs to sit beside it:

```powershell
$app = Split-Path (Get-ChildItem $gui\app -Recurse -Filter kwaainet_gui.exe | Select-Object -First 1).FullName
Get-ChildItem $app -Filter *64_1*.dll | Format-Table Name,Length   # expect cublas64_12, cudart64_12, curand64_10, ...
```

## 2. Launch against the running node

Launch the GUI from the expanded folder (not from a shortcut), so it is the
build under test:

```powershell
Start-Process (Get-ChildItem $gui\app -Recurse -Filter kwaainet_gui.exe | Select-Object -First 1).FullName
```

Wait 30 seconds, then:

```powershell
Get-Process | Where-Object { $_.ProcessName -match 'kwaainet' } | Format-Table Id,ProcessName,Path,StartTime
Get-Content $env:USERPROFILE\.kwaainet\run\kwaainet.pid
kwaainet status
```

**What passes:**

- There is still exactly one `kwaainet` node process, with the **same PID** as
  in section 0, running from `.cargo\bin`. The GUI attached; it did not spawn
  the bundled binary.
- 👀 The status header shows the node running. No red error bar about the
  daemon, authentication, or an old daemon. (A red bootstrap-down bar in the
  first 30 seconds is expected and should clear.)
- 👀 Settings → daemon mode is **Built-in** (the default). Look; do not change it.

## 3. Every tab talks to the node

👀 Ask Reza to open each tab in turn and report what it shows. For each, the
pass is **real data and no error**: no `unauthenticated`, `permission denied`,
`unimplemented`, "requires a newer daemon" or similar.

| Tab | What to look for |
| --- | --- |
| Status / home | Peer id, version 0.7.3, uptime |
| Peers | A non-empty table; the Name column shows announced names; search filters |
| DHT | The Local band lists this node's records |
| Sharding / map | Peers and block coverage; "Show empty peers" toggles |
| Chat | Send `Say hello in five words.` An answer arrives, and the inference panel shows how it was routed |
| Settings | Loads current values. Turn on **Experimental → RAG** only if this changes a GUI preference and not config.yaml; if unsure, skip it |
| RAG (if enabled) | Lists the KBs on this node. Do not create or ingest anything |

While the chat runs:

```powershell
nvidia-smi --query-gpu=name,memory.used,utilization.gpu --format=csv
kwaainet logs | Select-String -Pattern 'unauth|token|denied|error' | Select-Object -Last 20
```

Record any log line mentioning the token or authentication, and whether GPU
utilisation moved during the chat.

## 4. Quit the GUI

👀 Ask Reza to quit the GUI from the tray menu (right click → Quit), not just
close the window. Then:

```powershell
Get-Process | Where-Object { $_.ProcessName -match 'kwaainet' } | Format-Table Id,ProcessName,Path
kwaainet status
```

**What passes:** the GUI process is gone and the node is still running with the
same PID. The GUI did not start this node, so it must not stop it.

If the node was stopped, record it as a failure, then run `kwaainet restart`.

## 5. Relaunch and attach again

Launch the GUI again as in section 2, confirm 👀 it shows the node running, and
quit it from the tray again. This catches state the first run left behind
(stale PID or port files). Record anything different from the first run.

## 6. Negative variant: no node running

This checks what a fresh GUI user sees: the GUI has to start its own node. It
displaces the GPU node for a few minutes, so **ask Reza before starting this
section**, and skip it if he says no.

```powershell
kwaainet stop
kwaainet status          # expect: not running
Get-ChildItem $env:USERPROFILE\.kwaainet\run
```

Launch the GUI as in section 2. Wait 60 seconds.

```powershell
Get-Process | Where-Object { $_.ProcessName -match 'kwaainet' } | Format-Table Id,ProcessName,Path,StartTime
kwaainet status
Get-ChildItem $env:USERPROFILE\.kwaainet\run
Get-Content $env:USERPROFILE\.kwaainet\run\start-args.json
(Get-Acl $env:USERPROFILE\.kwaainet\run\kwaainet.token).Access | Format-Table IdentityReference,FileSystemRights,AccessControlType
```

**Record, not judge:**

- Which binary is running: the bundled one under `$gui\app`, or `.cargo\bin`.
- 👀 Whether the GUI shows it running, and whether Chat answers.
- Whether the bundled node uses the GPU. It should, since it is the CUDA
  build. While Chat runs, check:
  ```powershell
  nvidia-smi --query-gpu=memory.used,utilization.gpu --format=csv
  kwaainet logs | Select-String -Pattern 'cuda|gpu|cpu fallback' | Select-Object -Last 20
  ```
  A fallback to CPU here is a FAIL.
- Whether `start-args.json` changed, and how.
- The token's ACL. It must be exactly one entry, the current user, with
  FullControl and Allow.

Then 👀 ask Reza to stop the node from the GUI, and quit the GUI from the tray.

```powershell
Get-Process | Where-Object { $_.ProcessName -match 'kwaainet' } | Format-Table Id,ProcessName,Path
```

**What passes:** no `kwaainet` process is left, including no orphaned shard or
storage child.

### Restore

```powershell
$bk = "$env:USERPROFILE\kwaainet-guitest-backup"
Copy-Item $bk\config.yaml $env:USERPROFILE\.kwaainet\config.yaml -Force
Copy-Item $bk\start-args.json $env:USERPROFILE\.kwaainet\run\start-args.json -Force
kwaainet restart
kwaainet status
(Get-Process kwaainet).Path   # expect: .cargo\bin
```

The node must end up as it was in section 0, running from `.cargo\bin`. If it
does not, stop and tell Reza.

## 7. Results

Write `results/gui-v0.5.0-metro-win.md` with:

- The date, Windows version, GPU and driver.
- Each section's commands and full output, and Reza's 👀 answers word for word.
- A PASS / FAIL line per section, with the reason for each FAIL.
- Anything surprising, even if it passed.

Then:

```powershell
git add results/gui-v0.5.0-metro-win.md
git commit -m "test(gui-v0.5.0): metro-win results"
git push origin test/gui-v0.5.0-metro-win
```
