# Llama 3.3 70B heterogeneous sharding test

This branch, `test/llama70b-heterogeneous`, carries the instructions and the results for running Llama 3.3 70B split across three machines. It is a test branch and is never merged. The plan behind it is [`projects/kwaai-compute/plans/Llama70B-Heterogeneous-plan.md`](../../../projects/kwaai-compute/plans/Llama70B-Heterogeneous-plan.md).

## Who runs what

| Machine | Instructions | Serves | Writes results to |
|---|---|---|---|
| metro-win | [metro-win.md](metro-win.md) | blocks 8–40, CPU | `results/metro-win.md` |
| metro-linux | [metro-linux.md](metro-linux.md) | node A: blocks 40–72, CPU; node B: blocks 72–80, GPU | `results/metro-linux.md` |
| Mac mini (Reza's coordinator) | [mac-mini.md](mac-mini.md) | blocks 0–8, MLX; runs the prompts | `results/mac-mini.md`, `results/runs.md` |

## Phases

Each machine works through these phases and reports at each gate. Reza says when to move on.

1. **Download.** The 8B node keeps running. Weights go into the shared Hugging Face cache, using a throwaway `KWAAINET_HOME`, so the live `config.yaml` is never touched.
2. **Switch.** Stop the 8B node and bring up the 70B node(s). Report **READY**.
3. **Run.** Reza runs the prompts from the Mac. Each machine records its peak memory meanwhile.
4. **Restore.** Only when Reza says so. Put the 8B node back exactly as it was.

## Rules for every session

- **Record as you go,** in your own results file only; another machine writes the other files. Each step gets a PASS or FAIL line with the command output that decides it.
- **Commit and push after each phase:**
  ```
  git add tests/kwaai-compute/llama70b/results/<file>
  git commit -m "test(70b): <machine> <phase>"
  git pull --rebase
  git push
  ```
  Never force-push.
- **Ask Reza for anything on screen,** and record his answers word for word.
- **Don't improvise around a failure.** If a step fails, record the exact error and stop. Don't change ports, block ranges or config beyond what the instructions say without asking Reza first.
- **Don't run Restore until Reza says so.** The other machines depend on your node staying up.
- **Change nothing else in the repo.** Only edit your results file. If an instruction is wrong, say so in your results file and to Reza; don't edit the instructions.
