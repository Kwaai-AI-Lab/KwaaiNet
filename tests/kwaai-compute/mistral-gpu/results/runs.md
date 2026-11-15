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

### The same prompts from KwaaiNet GUI on metro-win

Reza ran the same three prompts from the GUI on metro-win (screenshot, 2026-11-14):

- **Standard model:** a good answer (incompleteness, dark matter and dark energy, the hierarchy problem), with different wording from the CLI runs.
- **"How do civiliations collapse?":** the GUI repeated "What are the causes of the collapse of civilizations?" until the token limit.
- **"Is that your real hair?":** the GUI rambled in the first person ("I mean, it's not a wig, is it? … I mean, I don't even know who I am …").

**Cause: the prompt template, not the model or the network.** The GUI's chat goes through gRPC `ShardRun` (`grpc_server.rs` → `shard_cmd::run_streaming`). `run_streaming` chooses a template from the tokenizer: Llama 3 if it has `<|start_header_id|>`, ChatML if it has `<|im_start|>`, and **otherwise the raw prompt**. Mistral's tokenizer has neither token, so the model gets plain text without `[INST]…[/INST]`, treats it as a document to continue, and with greedy decoding falls into a loop. The CLI runs above passed because the prompts were wrapped in `[INST]` by hand.

Sampling is not the difference. `run_streaming` hard-codes temperature 1.0, top_k 0 and top_p 1.0, the same as the CLI's defaults, and `sample_token` treats that combination as greedy.

**Reproduced from the Mac CLI on the same three-node chain, 64 tokens:**

| Prompt as sent | Output |
|---|---|
| `How do civiliations collapse?` (raw) | "What are the causes of the collapse of civilizations?" ×6, the same as the GUI |
| `Is that your real hair?` (raw) | "I mean, it's not a wig, is it? I'm just curious. I mean, …", the same as the GUI, word for word |
| `[INST]How do civiliations collapse?[/INST]` | "Civilizations collapse due to a complex interplay of various factors … 1. **Environmental Degradation** …", a good answer |

**Fix needed, outside this test:** add a Mistral branch, such as `[INST]` present in the vocab → `[INST]{prompt}[/INST]`. It must go in all three copies of the template-selection code in `shard_cmd.rs` (`cmd_shard_run`, the second CLI path, and `run_streaming`). Note that `shard_api.rs`, `api.rs` and `grpc_server.rs → build_prompt` hard-code Llama 3 for every model. A single shared template function would remove the duplication.

## Phase 4: Mistral Small 24B — 2026-11-14 (22:59–23:02 PST)

**Gate: PASS.** `shard chain --total-blocks 40` (prefix `Mistral-Small-24B-Instruct-2501`) returned 3 servers, 40/40 blocks: professor_reza 0–8 (MLX), metro-win-rtx-a5000 8–24 (CUDA), metro-linux 24–40 (CUDA). All three had pushed READY (Small 24B): the Mac's shard footprint is 9.9 GB; the metros' figures are in their files.

The same prompts and flags as Phase 3: `[INST]…[/INST]` by hand, `--max-tokens 128 --stats`, prompt 1 twice.

### Summary

| Run | Prompt | Tokens | Prefill | Decode avg (min–max) | Mac hop | metro-win hop | metro-linux hop | Total |
|---|---|---|---|---|---|---|---|---|
| 1 (cold) | Summarize the critique of the standard model of physics | 14 in / 128 out | 2767 ms | 328 ms (263–688) | 40 ms | 116 ms | 169 ms | 44.4 s |
| 2 (warm) | same | 14 / 128 | 935 ms | 331 ms (239–537) | 40 ms | 114 ms | 174 ms | 42.9 s |
| 3 | is that your real hair | 8 / 64 (EOS) | 817 ms | 387 ms (268–641) | 39 ms | 117 ms | 229 ms | 25.2 s |
| 4 | How do civilizations collapse | 8 / 128 | 633 ms | 305 ms (272–464) | 39 ms | 121 ms | 143 ms | 39.3 s |

**Result: PASS.** All four runs completed over the three GPUs with no timeout or error. The answers are coherent and noticeably more precise than Nemo's: it names the forces, and adds fine-tuning to the critique and economic factors to the collapse list.

### Compared with Nemo 12B (Phase 3)

| | Nemo 12B | Small 24B |
|---|---|---|
| Decode avg | 242–330 ms/tok | 305–387 ms/tok |
| Mac hop, 8 blocks | ~21 ms | ~40 ms |
| metro-win hop, 16 blocks | ~90 ms | ~117 ms |
| metro-linux hop, 16 blocks + `lm_head` | 136–217 ms | 143–229 ms |
| Throughput | 3.0–3.9 tok/s | 2.5–3.3 tok/s |

- **Twice the weights cost only about 15–20% more time per token.** The Mac hop doubled (21 → 40 ms) in line with its weights, and metro-win rose about 30%, but metro-linux barely moved. That fits metro-linux's hop being mostly network/relay rather than compute, as #293's metro-win profile also suggested (metro-win's GPU is about 43 ms of each ~500 ms token on Nemo).
- **Cold start cost was again small:** about 1.8 s of extra prefill on run 1, for 10 GB of MLX weights.
- **Runs 1 and 2 are identical text again** (greedy decoding; see #293).

### Outputs (verbatim)

#### run 1: Summarize the critique of the standard model of physics  (22:59:21)

```
The Standard Model of particle physics is a highly successful theory that describes three of the four known fundamental forces (electromagnetic, weak, and strong interactions) and classifies all known elementary particles. However, it is not without its critiques and limitations. Here are some of the main critiques:

1. **Incomplete**: The Standard Model does not include gravity, one of the four fundamental forces. Incorporating gravity into a consistent quantum framework remains a major challenge.

2. **Fine-Tuning Problem**: The model requires precise tuning of parameters (such as the Higgs boson mass) to match experimental observations. This fine-tuning is seen
```

#### run 2: Summarize the critique of the standard model of physics  (23:00:07)

```
The Standard Model of particle physics is a highly successful theory that describes three of the four known fundamental forces (electromagnetic, weak, and strong interactions) and classifies all known elementary particles. However, it is not without its critiques and limitations. Here are some of the main critiques:

1. **Incomplete**: The Standard Model does not include gravity, one of the four fundamental forces. Incorporating gravity into a consistent quantum framework remains a major challenge.

2. **Fine-Tuning Problem**: The model requires precise tuning of parameters (such as the Higgs boson mass) to match experimental observations. This fine-tuning is seen
```

#### run 3: is that your real hair  (23:00:52)

```
I don't have a physical body or hair. I exist purely in the digital world as a text-based AI model. I don't have senses, a physical presence, or personal experiences. I'm here to process and generate text based on the data I've been trained on. How can I assist you today?
```

#### run 4: How do civilizations collapse  (23:01:18)

```
Civilizations collapse due to a combination of internal and external factors. While each collapse is unique, historians and researchers have identified several common themes and processes. Here are some of the key factors:

1. **Environmental Changes**:
 - **Climate Change**: Shifts in climate can lead to droughts, floods, or other environmental changes that make agriculture difficult.
 - **Resource Depletion**: Overuse of natural resources can lead to scarcity and conflict.

2. **Economic Factors**:
 - **Economic Inequality**: Extreme wealth disparities can lead to social unrest and political instability.
 - **Economic Collapse**:
```

