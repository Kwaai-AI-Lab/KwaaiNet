# Building KwaaiNet from source on metro-win

For validating `main` on the A5000 box before a release tag exists. Run
directly on the machine — there is no SSH to metro (Tailscale trial ended
2026-07-25).

Roughly **60–90 minutes** for a clean release build. Windows differs from
metro-linux in three ways that matter, so this is not the Linux runbook with
different slashes.

## Prerequisites

```powershell
rustc --version          # 1.75+, MSVC toolchain (not GNU)
nvcc --version           # CUDA toolkit
go version               # kwaai-p2p-daemon's build.rs builds p2pd
```

Confirm the MSVC toolchain specifically — a `x86_64-pc-windows-gnu` default
will not link against the CUDA libraries:

```powershell
rustup show | Select-String "default host"     # expect x86_64-pc-windows-msvc
```

## The MSVC environment is required

`nvcc` shells out to `cl.exe`, which is not on `PATH` in an ordinary shell. CI
handles this with `ilammy/msvc-dev-cmd`; the local equivalent is to build from
a **Developer PowerShell for VS 2022**, or to import the environment into the
shell you are already in:

```powershell
# adjust the VS edition/year if different
& "C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\Tools\Launch-VsDevShell.ps1"
cl.exe                    # should print the compiler banner, not "not recognized"
```

Skipping this is the usual cause of a CUDA build failing several minutes in
with an `nvcc` error that does not mention `cl.exe`.

## Build

```powershell
cd $HOME\Source\KwaaiNet        # adjust if the checkout lives elsewhere
git fetch origin
git checkout main
git pull

git log --oneline -1            # expect 7acebeaf or later
```

Then, matching CI's recipe for this target (`.github/workflows/release.yml`,
*Build kwaainet with CUDA (Windows)*):

```powershell
cd core
$env:CUDA_COMPUTE_CAP = "80"
cargo build --release -p kwaainet --features cuda-windows -j 4
```

**Cap the parallelism — `-j 4`.** `cicc`, nvcc's device-code compiler, holds
around 3 GB per translation unit, and cargo defaults to one codegen job per
core. On 2026-08-25 an uncapped CUDA build exhausted metro-linux's RAM and
wedged the machine for roughly six hours (details in the metro-linux runbook).
Windows will page rather than invoke an OOM killer, so the failure mode here is
a build that crawls and a desktop that stops responding — less destructive, but
not worth risking. Check the headroom first and lower `-j` if it is tight:

```powershell
Get-CimInstance Win32_ComputerSystem | Select-Object TotalPhysicalMemory
(Get-CimInstance Win32_Processor).NumberOfLogicalProcessors
```

**`cuda-windows`, not `cuda`.** They are different features:

| feature | pulls in | used on |
|---|---|---|
| `cuda` | `kwaai-inference/flash-attn` | Linux |
| `cuda-windows` | `kwaai-inference/cuda` | Windows |

Flash-attention is not built on Windows. Using `--features cuda` here will
either fail to build or produce something CI never ships.

### Prefer the CPU build for this pass

None of what is being validated here — routed dials, relay circuits, config
handling — touches the GPU. A CPU build is faster, needs neither the CUDA
toolkit nor the MSVC environment above, and cannot run the machine out of
memory. Treat it as the default, not a fallback:

```powershell
cd core
cargo build --release -p kwaainet
```

Build CUDA only when the thing under test is inference itself.

## Install

Windows will not let you overwrite a running executable — stop the node first.
This is the opposite of the Unix behaviour, where a running process keeps its
inode and the file can be replaced underneath it.

```powershell
kwaainet stop
Start-Sleep -Seconds 5

Copy-Item "$HOME\.cargo\bin\kwaainet.exe" "$HOME\.cargo\bin\kwaainet-0.6.2.bak.exe" -Force
Copy-Item ".\target\release\kwaainet.exe" "$HOME\.cargo\bin\kwaainet.exe" -Force

kwaainet --version
```

If the copy fails with a sharing violation, something is still holding the
binary:

```powershell
Get-Process kwaainet -ErrorAction SilentlyContinue | Stop-Process -Force
```

## Restart and verify

```powershell
kwaainet start --daemon
Start-Sleep -Seconds 20

kwaainet p2p info | Select-Object -First 20
kwaainet shard chain | Select-Object -First 12
```

Expect the node back in `shard chain` with its block range, and `p2p info`
showing a confirmed external address or a relay reservation.

## What this build is being tested for

Nothing changes behaviour by default — `decentralized_dht` is `false`,
`announce_self` is `true`. The point is whether the fixes hold under real
traffic from a Windows peer.

**#137 — routed dials.** A cold bare-`PeerId` request, no connection held
first. Run from the Mac or another node against metro-win:

```bash
kwaainet p2p probe --peer <metro-win-peer-id> \
  --proto /kwaai/storage/1.0.0 --count 3
```

**#138 — relay circuit sizing.** Only meaningful if metro-win is reached
through a relay. If it holds a direct connection, this fix is exercised on
metro-linux instead.

**#139 — config reload before save.** Count, wait, count again:

```powershell
kwaainet rag list
Start-Sleep -Seconds 600
kwaainet rag list
```

**Windows auto-update restart.** Worth watching separately. `2ec22b99` fixed a
Windows-specific bug where auto-update installed a new binary and never
restarted the daemon — distinct from the Unix race, and **never tested on real
Windows**. It is unrelated to this manual build, but metro-win is the only
machine that can confirm it, and the next release is the occasion. Check after
any auto-update that the node actually came back:

```powershell
kwaainet status
Get-Content "$HOME\.kwaainet\logs\kwaainet.log" -Tail 40
```

## Rolling back

```powershell
kwaainet stop
Start-Sleep -Seconds 5
Copy-Item "$HOME\.cargo\bin\kwaainet-0.6.2.bak.exe" "$HOME\.cargo\bin\kwaainet.exe" -Force
kwaainet start --daemon
```

## Do not enable `decentralized_dht` yet

Same as metro-linux: it defaults to `false` and should stay there for this
pass. Enabling it on a single machine is worse than not enabling it at all,
because publishers and readers then disagree about where a record lives. It
needs a coordinated set of at least three nodes.
