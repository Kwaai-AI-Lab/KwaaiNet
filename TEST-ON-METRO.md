# Testing v0.7.4 on metro-linux and metro-win

**Instructions for a Claude Code session running on metro-linux or metro-win.**
This branch, `test/v0.7.4-metro`, exists only to carry this file and your
results. It is never merged.

v0.7.4 is a patch release on top of v0.7.3. Read what changed:

```bash
git fetch origin --tags && git log --oneline v0.7.3..v0.7.4
```

These are the changes this test exercises:

| PR | Change | What to look for |
|---|---|---|
| #304 | The last node samples the next token instead of returning logits | Shards still serve; `shard chain` lists the node |
| #302, #298 | Long prompts are chunked through the chain; an oversize reply no longer unregisters the shard | Node stays on `shard chain` |
| #299 | The control stream reconnects after a daemon restart | `restart` brings the node back cleanly |
| #273 | One node per `KWAAINET_HOME`, held by `run/kwaainet.lock` | A second `start` is refused |
| #272, #274 | The Windows token file is created owner-only | Token ACL (metro-win) |
| #292, #289, #290 | `head_dim` from config.json; whole-model announce fix; Mac pinned-range warning | No direct check |

You are checking four things:

1. The updater moves this machine from 0.7.3 to 0.7.4, and the node comes back
   up as it was.
2. The node keeps its GPU.
3. The token file has the right permissions.
4. The new instance lock refuses a second node.

## Rules

- **Do not change config.yaml or the node's start flags**, except for the
  restore step in section 0. `kwaainet restart` reuses the recorded flags, so
  use it rather than `stop` + `start`.
- **Do not edit code, rebuild, or reinstall by any other route.** If the update
  fails, record the failure and stop. The failure is the result.
- **Do not ask for or use IP addresses or hostnames** to reach other machines.
  KwaaiNet reaches peers over p2p.
- **Do not force-push.** Push only to this branch.
- If something is unclear or looks dangerous, stop and ask the person at the
  keyboard (Reza, over AnyDesk).

## 0. Leave the Mistral demo and go back to the installed node

This machine has been serving Mistral Small 24B for the 3-machine demo. That
node is a test build, not the installed `kwaainet`, and runs on a config the
demo changed. **Do this section only after Reza says the demo is over.**

1. Stop the demo node with the binary it runs:
   - **metro-linux:** the Nix build from PR #304,
     `/nix/store/w0a93sxg9mzw4165y1psmcpvzal6asa6-kwaainet-0.7.3/bin/kwaainet stop`.
     Check it against
     `tests/kwaai-compute/mistral-gpu/results/metro-linux-sampling.md` on
     branch `test/mistral-gpu-3node`.
   - **metro-win:** the `KwaaiNet-headdim` build that
     `results/metro-win.md` on that branch names.
2. Restore the pre-demo config from the backups the demo took, and check their
   checksums before copying.
   - **metro-linux:** `~/config.70b-backup.yaml` (sha256 `614868f0…`) and
     `~/start-args.70b-backup.json` (`96fa4abd…`) go to
     `~/.kwaainet/config.yaml` and `~/.kwaainet/run/start-args.json`.
   - **metro-win:** `config.70b-backup.yaml` and `start-args.70b-backup.json`,
     listed in `results/metro-win.md`, go to the same two places under
     `%USERPROFILE%\.kwaainet`.
   - If a backup is missing or its checksum differs, stop and ask Reza.
3. Make sure auto-update is off for the moment. Section 2 runs the update by
   hand so it can be watched:
   `kwaainet config set contribute.auto_update false`
4. Start the **installed** binary: `kwaainet start --daemon`, adding `--shard`
   only if the restored `start-args.json` has `"shard": true`. Confirm that
   `kwaainet --version` reports `0.7.3`.

## 1. Wait until v0.7.4 is published, then record the "before" state

Pushing the tag starts a build of about an hour, so the tag can exist before
the release does.

```bash
curl -s https://api.github.com/repos/Kwaai-AI-Lab/KwaaiNet/releases/latest | grep '"tag_name"'
```

Go on only when this prints `"tag_name": "v0.7.4"`. Until then, check every
five minutes and change nothing.

**metro-linux (bash):**

```bash
kwaainet --version
kwaainet status
cat ~/.kwaainet/run/kwaainet.pid
which kwaainet && ldd "$(which kwaainet)" | grep -E 'cuda|cublas|curand'
nvidia-smi --query-gpu=name,memory.used,memory.total --format=csv
ls -l ~/.kwaainet/run/
```

**metro-win (PowerShell):**

```powershell
kwaainet --version
kwaainet status
Get-Content $env:USERPROFILE\.kwaainet\run\kwaainet.pid
(Get-Command kwaainet).Source
nvidia-smi --query-gpu=name,memory.used,memory.total --format=csv
Get-ChildItem $env:USERPROFILE\.kwaainet\run
```

## 2. Update

```bash
kwaainet update --force
```

Keep the **full output**, because it names the archive that was downloaded.
On metro-linux expect `kwaainet-x86_64-unknown-linux-gnu-cuda.tar.xz`, and on
metro-win a `-cuda` or `-cuda-full` zip.

0.7.3's updater includes the restart fix (#234), so **the node should restart
itself**. Before running anything else, record whether it did:

- `kwaainet --version`: expect `kwaainet 0.7.4`.
- The PID in `run/kwaainet.pid` should differ from section 1.
- `kwaainet status` should show the node running.

If the node did not restart itself, say so: that counts as a FAIL for #234 on
this platform. Then run `kwaainet restart` and carry on.

## 3. After the update

Give the node a minute to rejoin the network, then:

**metro-linux:**

```bash
kwaainet --version
kwaainet status
ldd "$(which kwaainet)" | grep -E 'cuda|cublas|curand|not found'
ls -l ~/.kwaainet/run/kwaainet.token ~/.kwaainet/run/kwaainet.lock
kwaainet logs | grep -i -E 'cuda|gpu|cpu fallback|error|panick' | tail -30
nvidia-smi --query-gpu=name,memory.used,memory.total --format=csv
kwaainet shard chain
```

**metro-win:**

```powershell
kwaainet --version
kwaainet status
(Get-Acl $env:USERPROFILE\.kwaainet\run\kwaainet.token).Access | Format-Table IdentityReference,FileSystemRights,AccessControlType
Get-ChildItem $env:USERPROFILE\.kwaainet\run
kwaainet logs | Select-String -Pattern 'cuda|gpu|cpu fallback|error|panick' | Select-Object -Last 30
nvidia-smi --query-gpu=name,memory.used,memory.total --format=csv
kwaainet shard chain
```

Then repeat the GPU benchmark from the v0.7.3 results
(`results/v0.7.3-<machine>.md` on branch `test/v0.7.3-metro`). On metro-win
that is `kwaainet benchmark --gpu --steps 50`, which gave 29.4 tok/s decode
on 0.7.3. On metro-linux, use the same `kwaainet generate` command, which gave
34.9 tok/s. Record the number next to the old one.

**What passes:**

- `kwaainet 0.7.4`, and `status` shows the node running with the same shard
  and storage state as section 1.
- **metro-linux:** the `libcudart`, `libcublas`, `libcublasLt` and
  `libcurand` lines resolve beside the binary, with no `not found`.
- **Both:** the logs show CUDA in use and no fallback to CPU. The benchmark is
  within about 10% of 0.7.3's figure.
- **Both:** `run/kwaainet.token` exists.
  - On Linux it is `-rw-------`.
  - On Windows its ACL has exactly one entry: the current user, with
    FullControl and Allow.
- **Both:** `run/kwaainet.lock` exists (new in this release). If the node
  served shards before, it is in `shard chain` again.

## 4. The instance lock (#273)

With the node running:

```bash
kwaainet start --daemon
```

**Pass:** it refuses with `A KwaaiNet node is already running on this
KWAAINET_HOME` and exits non-zero. `kwaainet status` still shows the original
PID.

**Do not delete `run/kwaainet.pid` to test the other case.** That case is a
known gap, filed as #305, and it leaves a node that only `kill` can stop.

Then check that `restart` brings the node back cleanly (#299):

```bash
kwaainet restart
kwaainet status          # running, new PID
kwaainet logs | grep -i -E 'reconnect|control stream|error' | tail -10
```

## 5. Put things back

Return to the configuration that was in place before this test:
`kwaainet config set contribute.auto_update true`, **unless** the restored
config from section 0 already had it set to `false`. In that case leave it as
`false`. Say which in the results.

## 6. Report

Write your results to `results/v0.7.4-<machine>.md`, where `<machine>` is
`metro-linux` or `metro-win`. Include every command and its output from
sections 0 to 5. End with one line per check: **PASS**, **FAIL** or
**SKIPPED**, plus the reason. Then commit and push:

```bash
git fetch origin test/v0.7.4-metro
git checkout -B test/v0.7.4-metro origin/test/v0.7.4-metro
git add results/v0.7.4-<machine>.md
git commit -m "test(v0.7.4): <machine> results"
git push origin test/v0.7.4-metro
```

If the other machine pushed first, run `git pull --rebase origin
test/v0.7.4-metro` and push again. Never force-push. If this machine cannot
push, print the results file and tell Reza.
