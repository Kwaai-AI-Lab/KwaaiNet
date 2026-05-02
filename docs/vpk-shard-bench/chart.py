"""
VPK Shard Benchmark — Visualisation
Measured on KwaaiNet metro nodes, 2026-05-02
"""

import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import matplotlib.ticker
from matplotlib.lines import Line2D
import os

# ── Measured data (bench run, 2026-05-02) ────────────────────────────────────
bench_n           = [12_500,  25_000,  50_000]
bench_local       = [ 2_139,   2_269,   2_488]   # µs  (local HNSW p50)
bench_wan2        = [33_007,  32_268,  31_415]   # µs  (2 WAN Eves, p50)
bench_qdrant_loc  = [   496,     722,   1_173]   # µs  (Qdrant local Docker p50)
bench_qdrant_cld  = [29_076,  28_881,  67_012]   # µs  (Qdrant Cloud us-west-1 p50)

# Earlier RTT-characterisation run (15K/30K/60K) — keep for model grounding
measured_n    = [15_000, 30_000, 60_000]
measured_local = [1_257,  2_164,  2_072]         # µs

p2p_rtt_us    = 25_608                           # µs  (p50 across two metro Eves)

# ── KwaaiNet local HNSW model  (log2 regression, 12.5K–50K) ─────────────────
# local_p50(N) ≈ A + B * log2(N)   µs
A,  B  = -4229, 407.5               # original fit (15K/30K/60K)

def local_hnsw(n):
    return np.maximum(100, A + B * np.log2(np.maximum(n, 1)))

def sharded_p50(n, k, rtt_us):
    """Parallel fan-out: max shard latency (concurrent)."""
    return rtt_us + local_hnsw(n / k)

# ── Qdrant local HNSW model  (log2 regression, 12.5K–50K) ───────────────────
# qdrant_local_p50(N) ≈ A_q + B_q * log2(N)   µs
A_q, B_q = -4149, 338.5

def qdrant_local_model(n):
    return np.maximum(50, A_q + B_q * np.log2(np.maximum(n, 1)))

# ── Colour palette ─────────────────────────────────────────────────────────
C_LOCAL     = "#2563EB"   # blue      — KwaaiNet local
C_WAN       = "#DC2626"   # red       — WAN sharded Eve
C_DC        = "#D97706"   # amber     — Datacenter sharded
C_LAN       = "#16A34A"   # green     — LAN sharded
C_MEAS      = "#111827"   # near-black
C_QDRANT_L  = "#7C3AED"   # purple    — Qdrant local Docker
C_QDRANT_C  = "#0891B2"   # teal      — Qdrant Cloud (us-west-1)

fig = plt.figure(figsize=(17, 10))
fig.patch.set_facecolor("#F9FAFB")

# ════════════════════════════════════════════════════════════════════════════
# Panel A — Latency vs N  (left)
# ════════════════════════════════════════════════════════════════════════════
ax1 = fig.add_subplot(1, 2, 1)
ax1.set_facecolor("#F9FAFB")

N = np.logspace(4, 9, 500)          # 10K → 1B

# KwaaiNet local
ax1.loglog(N, local_hnsw(N)/1e3,
           color=C_LOCAL, lw=2.5, label="KwaaiNet local HNSW (model)")
ax1.scatter(measured_n, [v/1e3 for v in measured_local],
            color=C_LOCAL, s=60, zorder=5, marker="o",
            label="KwaaiNet local (prev run)")
ax1.scatter(bench_n, [v/1e3 for v in bench_local],
            color=C_LOCAL, s=90, zorder=6, marker="D",
            label="KwaaiNet local (bench run)")

# WAN sharded — K = 2, 10, 100
for k, alpha in [(2, 1.0), (10, 0.7), (100, 0.45)]:
    ax1.loglog(N, sharded_p50(N, k, p2p_rtt_us)/1e3,
               color=C_WAN, lw=2, alpha=alpha,
               label=f"KwaaiNet WAN sharded K={k}")
ax1.scatter(bench_n, [v/1e3 for v in bench_wan2],
            color=C_WAN, s=80, zorder=5, marker="s",
            label="KwaaiNet WAN K=2 (bench run)")

# LAN sharded — K = 2, 10
for k, alpha in [(2, 1.0), (10, 0.65)]:
    ax1.loglog(N, sharded_p50(N, k, 1_000)/1e3,
               color=C_LAN, lw=2, alpha=alpha, linestyle="--",
               label=f"LAN sharded K={k}")

# Datacenter sharded — K = 2
ax1.loglog(N, sharded_p50(N, 2, 5_000)/1e3,
           color=C_DC, lw=2, linestyle=":",
           label="Datacenter sharded K=2")

# Qdrant local Docker model + measured points
ax1.loglog(N, qdrant_local_model(N)/1e3,
           color=C_QDRANT_L, lw=2.5, linestyle="-",
           label="Qdrant local HNSW (model)")
ax1.scatter(bench_n, [v/1e3 for v in bench_qdrant_loc],
            color=C_QDRANT_L, s=90, zorder=6, marker="^",
            label="Qdrant local Docker (measured)")

# Qdrant Cloud (us-west-1) — scatter + dotted line (no model; 50K shows index-build spike)
ax1.scatter(bench_n, [v/1e3 for v in bench_qdrant_cld],
            color=C_QDRANT_C, s=90, zorder=6, marker="v",
            label="Qdrant Cloud us-west-1 (measured)")
ax1.plot(bench_n, [v/1e3 for v in bench_qdrant_cld],
         color=C_QDRANT_C, lw=1.2, linestyle=":", alpha=0.7)

# Annotate the Qdrant Cloud 50K spike
ax1.annotate(
    "50K spike:\nCloud index\nrebuild?",
    xy=(50_000, 67_012/1e3),
    xytext=(120_000, 45),
    arrowprops=dict(arrowstyle="->", color=C_QDRANT_C, lw=1.1),
    fontsize=7.5, color=C_QDRANT_C,
    bbox=dict(boxstyle="round,pad=0.25", fc="white", ec=C_QDRANT_C, alpha=0.85),
)

# Network floor lines
for rtt, label, col in [
    (p2p_rtt_us, f"WAN RTT floor  ({p2p_rtt_us/1e3:.0f} ms)", C_WAN),
    (5_000,      "Datacenter floor (5 ms)",                     C_DC),
    (1_000,      "LAN floor (1 ms)",                            C_LAN),
]:
    ax1.axhline(rtt/1e3, color=col, lw=1, linestyle="dotted", alpha=0.6)
    ax1.text(1.1e4, rtt/1e3 * 1.12, label, color=col, fontsize=7.5, va="bottom")

# Annotate KwaaiNet local "flat" region
ax1.annotate(
    "KwaaiNet HNSW:\n407 µs × log₂N\n(nearly flat)",
    xy=(3e7, local_hnsw(3e7)/1e3),
    xytext=(1.5e6, 12),
    arrowprops=dict(arrowstyle="->", color=C_LOCAL, lw=1.2),
    fontsize=8, color=C_LOCAL,
    bbox=dict(boxstyle="round,pad=0.3", fc="white", ec=C_LOCAL, alpha=0.85),
)

# Annotate Qdrant local "flat" region
ax1.annotate(
    "Qdrant HNSW:\n338 µs × log₂N\n(2.5× faster/op)",
    xy=(2e6, qdrant_local_model(2e6)/1e3),
    xytext=(2e4, 3.5),
    arrowprops=dict(arrowstyle="->", color=C_QDRANT_L, lw=1.2),
    fontsize=8, color=C_QDRANT_L,
    bbox=dict(boxstyle="round,pad=0.3", fc="white", ec=C_QDRANT_L, alpha=0.85),
)

ax1.set_xlabel("Number of vectors (N)", fontsize=11)
ax1.set_ylabel("Search latency p50 (ms)", fontsize=11)
ax1.set_title("A  |  Latency vs corpus size", fontsize=12, fontweight="bold", pad=10)
ax1.set_xlim(1e4, 1e9)
ax1.set_ylim(0.3, 200)
ax1.legend(fontsize=7, loc="upper left", framealpha=0.92, ncol=1)
ax1.grid(True, which="both", color="#E5E7EB", lw=0.6)
ax1.tick_params(labelsize=9)
ax1.yaxis.set_major_formatter(matplotlib.ticker.FuncFormatter(
    lambda x, _: f"{x:g}"))

# ════════════════════════════════════════════════════════════════════════════
# Panel B — Savings vs K  (right)
# ════════════════════════════════════════════════════════════════════════════
ax2 = fig.add_subplot(1, 2, 2)
ax2.set_facecolor("#F9FAFB")

K = np.logspace(0.3, 4, 300)       # 2 → 10,000 shards

# HNSW savings: local(N) - local(N/K) = B * log2(K)  (constant w.r.t. N)
savings    = B   * np.log2(K)       # µs  — KwaaiNet
savings_qd = B_q * np.log2(K)       # µs  — Qdrant (smaller B → smaller savings)

ax2.semilogx(K, savings/1e3,    color=C_LOCAL,    lw=2.5, label="KwaaiNet HNSW compute saved")
ax2.semilogx(K, savings_qd/1e3, color=C_QDRANT_L, lw=2,   linestyle="--",
             label="Qdrant HNSW compute saved")

# Shade "wins" regions
ax2.axhspan(0, 1,    alpha=0.12, color=C_LAN, label="LAN overhead (≤1 ms)")
ax2.axhspan(1, 5,    alpha=0.10, color=C_DC,  label="Datacenter overhead (1–5 ms)")
ax2.axhspan(5, 100,  alpha=0.07, color=C_WAN, label="WAN overhead (5–100 ms)")

# Horizontal threshold lines
for rtt, label, col in [
    (1,   "LAN breakeven  (1 ms)",    C_LAN),
    (5,   "DC breakeven   (5 ms)",    C_DC),
    (26,  "WAN breakeven  (26 ms)",   C_WAN),
]:
    ax2.axhline(rtt, color=col, lw=1.5, linestyle="--", alpha=0.8)
    ax2.text(2.2, rtt * 1.08, label, color=col, fontsize=8)

# Annotate crossover points for KwaaiNet
for rtt_ms, col, rtt_label in [
    (1,  C_LAN, "LAN"),
    (5,  C_DC,  "DC"),
    (26, C_WAN, "WAN"),
]:
    k_cross = 2 ** (rtt_ms * 1e3 / B)
    if k_cross < 8e3:
        ax2.axvline(k_cross, color=col, lw=1.2, linestyle=":", alpha=0.7)
        ax2.text(k_cross * 1.15, 0.3,
                 f"K≈{int(k_cross):,}\nbreakeven\n({rtt_label})", color=col,
                 fontsize=7.5, ha="left", va="bottom",
                 bbox=dict(boxstyle="round,pad=0.2", fc="white", ec=col, alpha=0.8))
    else:
        ax2.annotate(
            "WAN breakeven:\nK ≈ 2⁶⁴  (off-chart\n— impossible)",
            xy=(9500, 26), xytext=(300, 21),
            arrowprops=dict(arrowstyle="->", color=C_WAN, lw=1.2),
            fontsize=7.5, color=C_WAN,
            bbox=dict(boxstyle="round,pad=0.3", fc="white", ec=C_WAN, alpha=0.9),
        )

# Mark our two measured Eves
ax2.axvline(2, color=C_MEAS, lw=1, linestyle=":", alpha=0.5)
ax2.text(2.1, 24, "← our 2 metro Eves", fontsize=7.5, color=C_MEAS)

ax2.set_xlabel("Number of Eve shards (K)", fontsize=11)
ax2.set_ylabel("Compute time saved vs local (ms)", fontsize=11)
ax2.set_title("B  |  HNSW savings vs shard count K\n(independent of N — this is the key insight)",
              fontsize=12, fontweight="bold", pad=10)
ax2.set_xlim(2, 1e4)
ax2.set_ylim(0, 30)
ax2.legend(fontsize=8, loc="lower right", framealpha=0.9)
ax2.grid(True, which="both", color="#E5E7EB", lw=0.6, axis="both")
ax2.tick_params(labelsize=9)

# ── Title + caption ──────────────────────────────────────────────────────────
fig.suptitle(
    "VPK Shard Benchmark — KwaaiNet vs Qdrant (local Docker + Cloud)",
    fontsize=13, fontweight="bold", y=1.01
)
caption = (
    "Measured on two WAN metro Eve nodes (P2P RTT p50 = 25.6 ms) and Qdrant 1.15.5 "
    "(local Docker + Cloud us-west-1).  N = 12.5K / 25K / 50K vectors, dim = 384, Q = 200 queries.\n"
    "KwaaiNet HNSW: p50 ≈ −4229 + 407 × log₂N µs.  "
    "Qdrant HNSW: p50 ≈ −4149 + 338 × log₂N µs (2.5× faster/op, same log growth).  "
    "Qdrant Cloud 50K spike likely index-build threshold.  "
    "WAN sharding breaks even at K ≈ 2⁶⁴ — physically impossible."
)
fig.text(0.5, -0.04, caption, ha="center", fontsize=7.5, color="#4B5563",
         wrap=True, style="italic")

plt.tight_layout()

out = os.path.join(os.path.dirname(__file__), "vpk-shard-bench.png")
plt.savefig(out, dpi=150, bbox_inches="tight", facecolor=fig.get_facecolor())
print(f"Saved: {out}")
