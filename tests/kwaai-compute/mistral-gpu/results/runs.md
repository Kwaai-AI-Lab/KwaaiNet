# Runs (coordinator: Mac mini)

## Phase 3: Mistral Nemo 12B — 2026-11-14 (21:51–21:54 PST)

**Gate: PASS.** `shard chain --total-blocks 40` returned 3 servers, 40/40 blocks:

| Blocks | Node | Backend |
|---|---|---|
| 0–8 | professor_reza (Mac mini) | MLX, `target-nemo` binary |
| 8–24 | metro-win-rtx-a5000 | CUDA, A5000 |
| 24–40 | metro-linux | CUDA, A5000 (Nix `kwaainet-cuda` build) |

Both metros confirmed the shard was on the GPU before the runs (metro-linux: shard PID holds 11,800 MiB; see their results files).

Prompts were Reza's, chosen in place of the three in `mac-mini.md` (rivers, haiku, Australia), wrapped by hand in `[INST]…[/INST]`, with `--max-tokens 128 --stats`. Prompt 1 was run twice because the Mac's first forward pass pages in its MLX weights cold.

### Summary

| Run | Prompt | Tokens | Prefill | Decode avg (min–max) | Mac hop | metro-win hop | metro-linux hop | Total |
|---|---|---|---|---|---|---|---|---|
| 1 (cold) | Summarize the critique of the standard model of physics | 14 in / 128 out | 1780 ms | 283 ms (222–556) | 22 ms | 93 ms | 167 ms | 37.8 s |
| 2 (warm) | same | 14 / 128 | 732 ms | 330 ms (246–778) | 21 ms | 90 ms | 217 ms | 42.6 s |
| 3 | is that your real hair | 8 / 39 (EOS) | 698 ms | 242 ms (220–298) | 21 ms | 83 ms | 136 ms | 9.9 s |
| 4 | How do civilizations collapse | 8 / 128 | 607 ms | 317 ms (219–705) | 21 ms | 93 ms | 201 ms | 40.9 s |

**Result: PASS.** All four runs completed over three GPUs on three platforms, with no timeout or error, and the output is coherent, on-topic English. Run 3 stopped on EOS at 39 tokens; the others hit the 128-token cap mid-sentence.

### Observations

- **About 3–4 tok/s end to end** (decode 242–330 ms/tok). The per-hop averages add up to roughly 230–330 ms, so the time is in the hops themselves (compute plus the relayed round trip), not the client.
- **The Mac hop is the cheapest** at about 21 ms for 8 blocks, and steady (max 23 ms once warm). Cold paging only cost about 1 s of prefill on run 1 (1780 vs 732 ms), far from the 70B test's stalls.
- **metro-linux is the slowest and most variable hop** (136–217 ms average, maxima up to 538 ms) for the same 16 blocks that metro-win does in about 90 ms. It also holds `lm_head` (131k vocab × 5120), which adds some compute, but the spread suggests the relayed path to it is the larger part. Worth separating compute from transport in a later run.
- **Runs 1 and 2 produced identical text**, so sampling is greedy (or fixed-seed) with these flags. That makes the output reproducible across the three-machine chain, which is useful evidence that the head_dim fix gives the same numerics every time.
- **The headline throughput disagrees with the timing block** in runs 2 and 4 (e.g. "3.4 tok/s • 42.6s" vs "Throughput: 3.0 tok/s"). 128 / 42.6 s = 3.0, so the headline figure looks miscomputed. Cosmetic; not filed.

### Outputs (verbatim)

#### run 1: Summarize the critique of the standard model of physics  (21:51:36)

```
The Standard Model of Physics, which combines quantum mechanics and special relativity, has been incredibly successful in explaining three of the four known fundamental forces and classifying all known elementary particles. However, it faces several critiques and challenges:

1. **Incompleteness**: The Standard Model does not account for gravity, one of the four fundamental forces. This is often referred to as the "missing force." Efforts to incorporate gravity into the Standard Model have so far been unsuccessful, leading to the quest for a theory of quantum gravity.

2. **Dark Matter and Dark Energy**: Observations of the universe's large-scale structure and its accelerating expansion suggest the
```

#### run 2: Summarize the critique of the standard model of physics  (21:52:15)

```
The Standard Model of Physics, which combines quantum mechanics and special relativity, has been incredibly successful in explaining three of the four known fundamental forces and classifying all known elementary particles. However, it faces several critiques and challenges:

1. **Incompleteness**: The Standard Model does not account for gravity, one of the four fundamental forces. This is often referred to as the "missing force." Efforts to incorporate gravity into the Standard Model have so far been unsuccessful, leading to the quest for a theory of quantum gravity.

2. **Dark Matter and Dark Energy**: Observations of the universe's large-scale structure and its accelerating expansion suggest the
```

#### run 3: is that your real hair  (21:52:59)

```
No, it's not. I'm a text-based AI and don't have a physical body or hair. I'm here to provide information and answer questions to the best of my ability.
```

#### run 4: How do civilizations collapse  (21:53:11)

```
Civilizations collapse for a variety of reasons, often a combination of factors. Here are some of the most common causes:

1. **Environmental Degradation**: This is one of the most common causes of civilizational collapse. It can manifest in several ways:
 - **Climate Change**: Changes in climate can lead to droughts, floods, or other extreme weather events that make it difficult for a civilization to sustain itself.
 - **Soil Erosion and Depletion**: Intensive farming can lead to the loss of fertile soil, making it harder to grow food.
 - **Deforestation**: The loss of
```

