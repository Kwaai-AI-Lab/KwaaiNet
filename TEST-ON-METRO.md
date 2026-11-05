# Testing v0.7.3 on metro-linux and metro-win

**Instructions for a Claude Code session running on metro-linux or metro-win.**
This branch, `test/v0.7.3-metro`, exists only to carry this file and your
results. It is never merged.

v0.7.3 is an interim release: everything planned for 0.8.0, shipped as a patch
so it can be tested in production first. Read its release commit for what
changed:

```bash
git fetch origin && git show -s origin/main --format=%B   # or: git log -1 v0.7.3
```

You are checking three things:

1. The updater moves this machine from 0.7.2 to 0.7.3 and the node comes back
   up as it was.
2. The node keeps its GPU. metro-linux matters most here: v0.7.3 is the first
   release whose Linux CUDA archive ships its own CUDA runtime (#258, #240).
3. The new gRPC token gate (#254) is in place.

On metro-win there is a fourth: the PR #274 test (section 4).

## Rules

- **Do not change config.yaml** or the node's start flags. `kwaainet restart`
  reuses the recorded flags, so use it rather than `stop` + `start`.
- **Do not edit code, rebuild, or reinstall by any other route.** If the update
  fails, record the failure and stop. The failure is the result.
- **Do not ask for or use IP addresses or hostnames** to reach other machines.
  KwaaiNet reaches peers over p2p; Reza checks this node from his Mac that way.
- **Do not force-push.** Push only to this branch.
- If something is unclear or looks dangerous, stop and ask the person at the
  keyboard (Reza, over AnyDesk).

## 0. Wait until v0.7.3 is published

`kwaainet update` installs whatever GitHub marks as the latest release. Pushing
the tag starts a build of about an hour, so the tag can exist before the
release does.

```bash
curl -s https://api.github.com/repos/Kwaai-AI-Lab/KwaaiNet/releases/latest | grep '"tag_name"'
```

Go on only when this prints `"tag_name": "v0.7.3"`. Until then, check every
five minutes and change nothing.

## 1. Before the update

Record each output for the results file (section 5).

**metro-linux (bash):**

```bash
kwaainet --version
kwaainet status
which kwaainet && ldd "$(which kwaainet)" | grep -E 'cuda|cublas|curand'
nvidia-smi --query-gpu=name,memory.used,memory.total --format=csv
ls -l ~/.kwaainet/run/
```

**metro-win (PowerShell):**

```powershell
kwaainet --version
kwaainet status
(Get-Command kwaainet).Source
nvidia-smi --query-gpu=name,memory.used,memory.total --format=csv
Get-ChildItem $env:USERPROFILE\.kwaainet\run
```

Expect `kwaainet 0.7.2`. If it is already 0.7.3, the auto-updater got there
first: say so in the results, skip section 2, and carry on from section 3.

## 2. Update

```bash
kwaainet update --force
kwaainet --version          # expect: kwaainet 0.7.3
kwaainet restart
```

- Keep the **full output of `update --force`**. It names the archive it
  downloaded, which is part of the result.
  - On metro-linux, expect `kwaainet-x86_64-unknown-linux-gnu-cuda.tar.xz`,
    which is now the full archive with the CUDA libraries inside.
  - On metro-win, expect a `-cuda` or `-cuda-full` zip.
- Run `kwaainet restart` even if the update says it restarted the node. The
  update is performed by 0.7.2's updater, and the restart fix (#234) only
  arrives in 0.7.3.

## 3. After the update

Give the node a minute to rejoin the network, then:

**metro-linux:**

```bash
kwaainet --version
kwaainet status
ldd "$(which kwaainet)" | grep -E 'cuda|cublas|curand|not found'
ls -l "$(dirname "$(readlink -f "$(which kwaainet)")")"
ls -l ~/.kwaainet/run/kwaainet.token
kwaainet logs | grep -i -E 'cuda|gpu|cpu fallback|error' | tail -30
nvidia-smi --query-gpu=name,memory.used,memory.total --format=csv
kwaainet shard chain
```

**metro-win:**

```powershell
kwaainet --version
kwaainet status
Get-Item $env:USERPROFILE\.kwaainet\run\kwaainet.token | Format-List FullName,Length
(Get-Acl $env:USERPROFILE\.kwaainet\run\kwaainet.token).Access | Format-Table IdentityReference,FileSystemRights,AccessControlType
kwaainet logs | Select-String -Pattern 'cuda|gpu|cpu fallback|error' | Select-Object -Last 30
nvidia-smi --query-gpu=name,memory.used,memory.total --format=csv
kwaainet shard chain
```

**What passes:**

- `kwaainet 0.7.3`, and `status` shows the node running with the same shard
  and storage state as before the update.
- **metro-linux:** every `libcudart`, `libcublas`, `libcublasLt` and
  `libcurand` line resolves, with no `not found`, and they point beside the
  binary rather than into `/usr/local/cuda`. `libcuda.so.1` comes from the
  driver, which is expected.
- **Both:** the logs show CUDA in use, with no fallback to CPU. If the node
  serves shards, GPU memory in use should be well above its idle value.
- **Both:** `run/kwaainet.token` exists.
  - On Linux it must be `-rw-------`.
  - On Windows its ACL must hold exactly one entry: the current user, with
    FullControl and Allow.
- **Both:** if the node served shards before, it appears in `shard chain`
  again.

## 4. metro-win only: the PR #274 test

PR CI never runs the Windows tests, so this is the only check of #274 before
it merges. Run it only if this machine has a Rust toolchain (`cargo
--version`) and a KwaaiNet clone. Otherwise skip it and say so in the results.

```powershell
cd <your KwaaiNet clone>
git fetch origin fix/dacl-test-sid-alias
git worktree add ..\KwaaiNet-pr274 origin/fix/dacl-test-sid-alias
cd ..\KwaaiNet-pr274\core
cargo test -p kwaai-rpc --lib auth::
```

- **Pass:** all six `auth::tests` pass, including
  `token_file_dacl_is_current_user_only`.
- It also helps to say whether this session runs as the built-in Administrator
  (`whoami /user`, where the SID ends in `-500`). That is the case #269 was
  about. Under a normal account, the old test passed too.

If `gh` is logged in, also post the test output as a comment on PR #274:

```powershell
gh pr comment 274 --repo Kwaai-AI-Lab/KwaaiNet --body-file <file with the output>
```

## 5. Report

Write your results to `results/v0.7.3-<machine>.md`, where `<machine>` is
`metro-linux` or `metro-win`. Include every command and its output from
sections 1 to 4, and end with one line per check: **PASS**, **FAIL** or
**SKIPPED**, plus the reason. Then commit the file and push it to this branch:

```bash
git fetch origin test/v0.7.3-metro
git checkout -B test/v0.7.3-metro origin/test/v0.7.3-metro
git add results/v0.7.3-<machine>.md
git commit -m "test(v0.7.3): <machine> results"
git push origin test/v0.7.3-metro
```

If the push is rejected because the other machine pushed first, run `git pull
--rebase origin test/v0.7.3-metro` and push again. Never force-push.

If this machine cannot push, print the results file and tell Reza, who will
carry it across.
