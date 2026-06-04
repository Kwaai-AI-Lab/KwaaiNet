# NSF ACCESS Delta — KwaaiNet Inference Integration

**Applies to:** Any KwaaiNet inference workload (RAG graph build, eval, dream cycle)  
**System:** [Delta](https://docs.ncsa.illinois.edu/systems/delta/) at NCSA, University of Illinois  
**Credits:** NSF ACCESS research allocation

---

## Delta H100 Hardware

| Partition | GPUs per node | GPU model | VRAM | Notes |
|-----------|--------------|-----------|------|-------|
| `gpuH100x8` | 8 | H100 SXM5 | 80 GB HBM3 each | Primary target |
| `gpuH100x4` | 4 | H100 SXM5 | 80 GB HBM3 each | Smaller jobs |
| `gpuA100x8` | 8 | A100 SXM4 | 80 GB HBM2e | Fallback |
| `gpuA40x4` | 4 | A40 | 48 GB GDDR6 | Cheaper |

**Useful facts:**
- Login: `login.delta.ncsa.illinois.edu` (SSH with ACCESS credentials)
- Scheduler: SLURM
- Storage: `$HOME` (25 GB), `$SCRATCH` (`/scratch/your_project/`) — large models go on scratch
- Internet access: outbound from compute nodes ✅ — enables p2p relay connection
- Inbound connections to compute nodes: ❌ — blocked by firewall
- Apptainer (Singularity) containers: ✅ — use for reproducible Ollama/vLLM environments
- Max interactive walltime: 30 min (use batch for builds)
- Max batch walltime: 48 h

---

## Integration Options

### Option A — Outbound p2p relay from SLURM job (Recommended first step)

**How it works:** The SLURM job runs `kwaainet start --daemon` inside the allocation. Because
compute nodes make *outbound* connections, the node joins the p2p relay fabric normally. The
local machine sees a new peer via `kwaainet node list` and can route inference to it immediately.

**Pros:** Zero protocol changes. Works today once `kwaainet` is installed on Delta.  
**Cons:** Allocation must outlast the build. Idle time between builds burns credits.  
**Best for:** Full graph builds, dream cycles — any long single-shot workload.

**Setup:**

```bash
# On Delta — one-time setup
# 1. Install kwaainet (download release binary)
mkdir -p ~/.local/bin
curl -fsSL https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/latest/download/kwaainet-installer.sh | sh
# or: scp the binary from your local machine

# 2. Install Ollama
curl -fsSL https://ollama.ai/install.sh | sh
ollama pull llama3.1:8b   # or whichever model

# 3. Copy your node identity (or generate a new one)
scp ~/.kwaainet/identity.yaml delta:~/.kwaainet/identity.yaml
# OR: kwaainet identity init   (generates a fresh peer ID)
```

**SLURM job script (`run-inference-node.sh`):**

```bash
#!/bin/bash
#SBATCH --job-name=kwaai-inference
#SBATCH --partition=gpuH100x8
#SBATCH --gpus-per-node=1
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=8
#SBATCH --mem=32G
#SBATCH --time=02:00:00
#SBATCH --account=YOUR_ACCESS_ALLOCATION

# Start Ollama in background
ollama serve &
sleep 5

# Start KwaaiNet daemon — connects outbound to p2p relay
kwaainet start --daemon --public-name "delta-h100"

# Keep allocation alive until job walltime
wait
```

```bash
# Submit and get the peer ID
sbatch run-inference-node.sh
# Watch for the peer ID in the job log:
squeue --me
tail -f slurm-<JOBID>.out
# Once you see: "Peer ID: 12D3KooW..."
# Add it to your local build command as --inference-urls mux://12D3KooW...
```

**Robustness note:** If the p2p connection drops, `kwaainet start` should reconnect automatically.
The inference-mux retry logic will handle transient drops. See [known robustness gaps](#known-robustness-gaps).

---

### Option B — vLLM + SSH reverse tunnel + direct HTTP URL (Production path)

**How it works:** The SLURM job runs vLLM (OpenAI-compatible API) on the H100. An SSH reverse
tunnel forwards the vLLM port to a known public host (e.g., your Mac or a VPS). KwaaiNet
connects to the tunnel endpoint as a plain `http://` inference URL.

**Pros:** Low latency (no p2p relay overhead). Survives relay disconnects. Reusable across
multiple local sessions without restarting the job.  
**Cons:** Requires a persistent SSH tunnel process and a host with a public IP or known address.
Needs a new `http://` inference URL type in KwaaiNet (currently only `mux://` and localhost).  
**Best for:** When you have a long allocation (8–48h) and want to run multiple build cycles
or interactive dev against the H100.

**SLURM job script (`run-vllm-tunnel.sh`):**

```bash
#!/bin/bash
#SBATCH --job-name=kwaai-vllm
#SBATCH --partition=gpuH100x8
#SBATCH --gpus-per-node=1
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=16
#SBATCH --mem=64G
#SBATCH --time=08:00:00
#SBATCH --account=YOUR_ACCESS_ALLOCATION

# Install vLLM if not already in your env
# module load python/3.11  (check Delta module system)
# pip install vllm --quiet

# Start vLLM with OpenAI-compatible API on port 8000
python -m vllm.entrypoints.openai.api_server \
  --model meta-llama/Llama-3.1-8B-Instruct \
  --port 8000 \
  --gpu-memory-utilization 0.85 &

sleep 30  # wait for model load

# Open reverse tunnel back to relay host (YOUR_RELAY_HOST must be reachable via SSH)
# This forwards remote port 18000 → compute node port 8000
ssh -N -R 18000:localhost:8000 \
    -o ServerAliveInterval=30 \
    -o ExitOnForwardFailure=yes \
    YOUR_USER@YOUR_RELAY_HOST &

echo "vLLM tunnel open — use http://YOUR_RELAY_HOST:18000 as inference URL"
wait
```

**KwaaiNet change required:** Add `http://` URL support to `inference_mux.rs` / `ollama_proxy.rs`
so `--inference-urls http://YOUR_RELAY_HOST:18000` routes directly via HTTP instead of p2p relay.
Estimated effort: ~2 days (the Ollama proxy HTTP client already exists; just needs a URL scheme
branch in the mux dispatcher).

---

### Option C — Async SLURM job submission per inference call (Research path)

**How it works:** Each chunk inference request submits a micro SLURM job, waits for output,
and returns the result. No persistent allocation needed — credits burn only during actual work.

**Pros:** No wasted idle time. Works even with very short allocation windows.  
**Cons:** SLURM job startup overhead (~30–60s per job) makes it impractical for 1136-chunk
builds. Better suited to one-shot large batch tasks than interactive inference.  
**Best for:** Overnight batch runs where throughput matters more than latency, or if interactive
allocations are unavailable.

**Implementation note:** Requires a `SlurmInferenceBackend` in `graph.rs` — a new impl of the
inference trait that calls `sbatch`, polls `squeue`, reads output. Estimated effort: ~5 days.
Not recommended until Options A and B are validated.

---

## Recommended Sequence

```
Phase 1 (now):   Option A — get kwaainet + Ollama running on Delta via SLURM
                  Validate: one 30-min interactive job, one chunk extraction test
Phase 2:         Fix inference-mux robustness (retry on skip, circuit breaker)
                  so Option A survives flaky relay connections during long builds
Phase 3:         Option B — add http:// URL type, use vLLM for higher throughput
                  (vLLM batching on H100 >> Ollama on A6000 for the same model)
Phase 4:         If credits allow, keep a standing 8h allocation during active
                  D6 iteration cycles; tear down when not building
```

---

## Known Robustness Gaps

The following issues affect all remote inference options and should be fixed before using Delta
for production builds:

| Gap | File | Fix |
|-----|------|-----|
| Timeout = silent chunk skip, no retry | `graph.rs:3033–3055` | Retry up to 3× with backoff (2s, 8s, 30s); log skips to `skipped_chunks.log` |
| No retry in `ollama_proxy` | `ollama_proxy.rs:308` | Pass mux retry result through instead of immediate 502 |
| No circuit breaker | `inference_mux.rs` | Park peer for 60s after 3 consecutive failures |
| Instant reconnect, no backoff | `inference_mux.rs:510–542` | Exponential backoff (100ms → 1s → 10s) before reconnect |

Fix tracking: `projects/kwaai-compute/inference-robustness.md` (to be created)

---

## Model Notes for H100

| Model | VRAM required | H100 fit | Expected tokens/s |
|-------|--------------|----------|-------------------|
| llama3.1:8b (Q4) | ~5 GB | ✅ trivial | ~800–1200 t/s |
| llama3.1:70b (Q4) | ~40 GB | ✅ comfortable | ~200–400 t/s |
| llama3.3:70b (Q4) | ~40 GB | ✅ comfortable | ~200–400 t/s |
| llama3.1:70b (FP16) | ~140 GB | ❌ (80 GB limit) | — |
| llama3.1:405b (Q4) | ~230 GB | ❌ single node | Multi-node only |

**Recommendation for D6 builds:** llama3.1:70b Q4 on a single H100 — same accuracy tier as
the 8b but with significantly better entity extraction quality. The 70b at ~300 t/s on an H100
would reduce per-chunk latency vs the 8b on an A6000, while producing fewer extraction errors.
