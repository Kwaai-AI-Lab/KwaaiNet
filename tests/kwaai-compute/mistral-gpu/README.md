# Mistral on three GPUs: Mac (MLX), metro-win (CUDA), metro-linux (CUDA)

This branch, `test/mistral-gpu-3node`, carries the instructions and results. It is never merged. The plan is [`projects/kwaai-compute/plans/Mistral-GPU-3Node-plan.md`](../../../projects/kwaai-compute/plans/Mistral-GPU-3Node-plan.md).

The code under test is on a different branch, **`fix/config-head-dim`**. Every machine builds its binary from that branch.

## Who runs what

| Machine | Instructions | Blocks | Writes results to |
|---|---|---|---|
| Mac mini (Reza's coordinator) | [mac-mini.md](mac-mini.md) | 0–8, MLX; runs the prompts | `results/mac-mini.md`, `results/runs.md` |
| metro-win | [metro-win.md](metro-win.md) | 8–24, CUDA | `results/metro-win.md` |
| metro-linux | [metro-linux.md](metro-linux.md) | 24–40, CUDA | `results/metro-linux.md` |

## Phases

Reza says when to move from one phase to the next.

0. **Leave the 70B.** Stop the 70B node(s) from the last test. Don't restore the 8B yet.
1. **Build and download Nemo.** Build from `fix/config-head-dim`. Download Mistral Nemo 12B's blocks under a throwaway `KWAAINET_HOME`. Report **BUILT+DOWNLOADED**.
2. **Serve Nemo.** Start the node on the GPU. Report **READY**.
3. **Nemo runs.** Reza runs the prompts. Record GPU memory meanwhile.
4. **Switch to Mistral Small 24B.** Download, restart with the new model, report **READY**, and record during the runs.
5. **Restore.** Only when Reza says so. Put the 8B node back with the backups from the 70B test, using the installed `kwaainet`.

## Rules for every session

- **Record as you go,** in your own results file only. Each step gets a PASS or FAIL line with the command output that decides it.
- **Commit and push after each phase:**
  ```
  git add tests/kwaai-compute/mistral-gpu/results/<file>
  git commit -m "test(mistral): <machine> <phase>"
  git pull --rebase
  git push
  ```
  Never force-push.
- **Don't improvise around a failure.** Record the exact error and stop. Change no ports, block ranges or config beyond the instructions without asking Reza.
- **Don't run Restore until Reza says so.**
- **Change nothing else in the repo.** If an instruction is wrong, say so in your results file and to Reza.
- **Keep the 70B backups.** `config.70b-backup.yaml` and `start-args.70b-backup.json` in your home directory are what Phase 5 restores from. Don't overwrite them.
- **Don't replace the installed `kwaainet`.** Run the test binary by its path (`$K` below). A daemon started from `$K` starts its shard child from `$K` too.
