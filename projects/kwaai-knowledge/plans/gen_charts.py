import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
import numpy as np
from matplotlib.gridspec import GridSpec
from adjustText import adjust_text

KWAAI_BLUE   = "#1A73E8"
KWAAI_TEAL   = "#00BFA5"
KWAAI_AMBER  = "#FFB300"
KWAAI_RED    = "#E53935"
KWAAI_PURPLE = "#7B1FA2"   # D6 accent
KWAAI_GREY   = "#B0BEC5"
BG           = "#FAFAFA"
GRID         = "#E0E0E0"

plt.rcParams.update({
    "font.family": "sans-serif",
    "font.size": 11,
    "axes.spines.top": False,
    "axes.spines.right": False,
    "axes.facecolor": BG,
    "figure.facecolor": "white",
    "axes.grid": True,
    "grid.color": GRID,
    "grid.linewidth": 0.6,
})

# ── Data ──────────────────────────────────────────────────────────────────────

# Pipeline KBs (SQLite rebuild)
pipeline_kbs = ["OSMDocs", "CountryHistory", "Astrophysics", "DreamMem",
                "Poems", "WarPeace", "DeepSea"]
retrieval    = [96.3, 95.9, 89.5, 88.7, 86.3, 85.6, 87.2]
generation   = [81.3, 80.8, 74.0, 72.9, 71.9, 77.7, 33.3]

# D6 primary benchmark (separate section in report; token-overlap eval)
D6_RETRIEVAL = 89.5
D6_CHUNKS    = 1152
D6_ENTITIES  = 911
D6_BUILD_HRS = 2120 / 3600   # 35 min
D6_SPEED     = 0.54

# Pre-pipeline KBs
pre_kbs   = ["Legal", "ragbench", "MobyDick", "NIST", "Climate", "PythonDocs"]
pre_retr  = [94.4, 88.9, 87.0, 87.2, 80.2, 73.9]

# All KBs for ranked chart (pipeline + D6 + pre-pipeline)
all_kbs   = pipeline_kbs + ["D6"] + pre_kbs
all_retr  = retrieval    + [D6_RETRIEVAL] + pre_retr
# colour: blue=pipeline, purple=D6, grey=pre-pipeline
all_colors_map = (
    [KWAAI_BLUE] * len(pipeline_kbs)
    + [KWAAI_PURPLE]
    + [KWAAI_GREY] * len(pre_kbs)
)

# Build stats (pipeline + D6)
build_kbs  = pipeline_kbs + ["D6"]
chunks     = [672, 18754, 5638, 2254, 28742, 37486, 1886,  D6_CHUNKS]
entities   = [240, 8440,  5873, 2528, 7947,  17132, 145,   D6_ENTITIES]
build_hrs  = [1034/3600, 46650/3600, 20897/3600, 7908/3600,
              28878/3600, 103973/3600, 697/3600,  D6_BUILD_HRS]
speed      = [0.65, 0.40, 0.27, 0.29, 1.00, 0.36, 2.71, D6_SPEED]

# D6 phase progression
phases    = ["Phase 1\n(Person)", "Phase 2\n(+Place)", "Phase 2\nwindow=1",
             "Phase 3\n(+Org/Leg/Pub)"]
d6_recall = [82.0, 86.0, 88.0, 89.5]

# ── Figure 1: Retrieval & Generation Recall (pipeline KBs + D6) ──────────────
fig, ax = plt.subplots(figsize=(12, 5.5))

# Add D6 as the last group, separated by a small gap
all_plot_kbs = pipeline_kbs + ["D6"]
all_plot_ret = retrieval    + [D6_RETRIEVAL]
all_plot_gen = generation   + [None]         # no generation score for D6

x = np.arange(len(all_plot_kbs))
w = 0.36

bar_colors_r = [KWAAI_BLUE] * len(pipeline_kbs) + [KWAAI_PURPLE]
bars_r = ax.bar(x - w/2, all_plot_ret, w, label="Retrieval recall",
                color=bar_colors_r, zorder=3)

# Generation bars (None → 0 with hatch)
gen_vals = [g if g is not None else 0 for g in all_plot_gen]
bars_g = ax.bar(x + w/2, gen_vals, w, label="Generation recall",
                color=KWAAI_TEAL, zorder=3)
# Hatch the D6 generation bar to signal N/A
bars_g[-1].set_hatch("///")
bars_g[-1].set_edgecolor("#aaa")
bars_g[-1].set_facecolor("#eee")

ax.axhline(80, color=KWAAI_AMBER, linewidth=1.2, linestyle="--", label="80% target floor", zorder=2)
ax.axhline(90, color=KWAAI_AMBER, linewidth=1.2, linestyle=":",  label="90% target ceiling", zorder=2)

for bar in bars_r:
    ax.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 0.7,
            f"{bar.get_height():.1f}%", ha="center", va="bottom",
            fontsize=8.5, fontweight="bold")
for bar, val in zip(bars_g, all_plot_gen):
    if val is not None:
        ax.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 0.7,
                f"{val:.1f}%", ha="center", va="bottom", fontsize=8.5)
    else:
        ax.text(bar.get_x() + bar.get_width()/2, 2,
                "N/A", ha="center", va="bottom", fontsize=7.5, color="#888")

# Vertical separator before D6
ax.axvline(len(pipeline_kbs) - 0.5, color="#ccc", linewidth=1.2, linestyle="--", zorder=1)
ax.text(len(pipeline_kbs) - 0.5, 104, "D6", ha="center", fontsize=8,
        color=KWAAI_PURPLE, style="italic")

ax.set_xticks(x)
ax.set_xticklabels(all_plot_kbs, fontsize=9.5)
ax.set_ylabel("Recall (%)")
ax.set_ylim(0, 110)
ax.set_title("Retrieval & Generation Recall — Pipeline KBs + D6 Primary Benchmark",
             fontsize=12.5, fontweight="bold", pad=12)

# Annotate DeepSea anomaly
ax.annotate("Scorer penalises\nabstractive answers",
            xy=(6 + w/2, 33.3), xytext=(4.8, 52),
            arrowprops=dict(arrowstyle="->", color=KWAAI_RED, lw=1.1),
            fontsize=8, color=KWAAI_RED)

patch_pipe  = mpatches.Patch(color=KWAAI_BLUE,   label="Pipeline KB retrieval")
patch_d6    = mpatches.Patch(color=KWAAI_PURPLE, label="D6 retrieval (primary benchmark)")
patch_gen   = mpatches.Patch(color=KWAAI_TEAL,   label="Generation recall")
patch_na    = mpatches.Patch(facecolor="#eee", hatch="///", edgecolor="#aaa",
                              label="Generation N/A (token-overlap eval only)")
ax.legend(handles=[patch_pipe, patch_d6, patch_gen, patch_na],
          loc="lower right", framealpha=0.9, fontsize=8.5)

fig.tight_layout()
fig.savefig("chart1_recall_pipeline.png", dpi=150)
plt.close(fig)
print("chart1 done")

# ── Figure 2: Retrieval recall — all KBs ranked ──────────────────────────────
fig, ax = plt.subplots(figsize=(13, 5.5))

order       = sorted(range(len(all_retr)), key=lambda i: all_retr[i], reverse=True)
sorted_kbs  = [all_kbs[i] for i in order]
sorted_retr = [all_retr[i] for i in order]
colors      = [all_colors_map[i] for i in order]

bars = ax.bar(range(len(sorted_kbs)), sorted_retr, color=colors, zorder=3)
ax.axhline(80, color=KWAAI_AMBER, linewidth=1.2, linestyle="--", zorder=2)
ax.axhline(90, color=KWAAI_AMBER, linewidth=1.2, linestyle=":",  zorder=2)

for bar, val in zip(bars, sorted_retr):
    ax.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 0.5,
            f"{val:.1f}%", ha="center", va="bottom", fontsize=8.5, fontweight="bold")

ax.set_xticks(range(len(sorted_kbs)))
ax.set_xticklabels(sorted_kbs, rotation=30, ha="right", fontsize=9.5)
ax.set_ylabel("Retrieval recall (%)")
ax.set_ylim(0, 108)
ax.set_title("Retrieval Recall — All 14 Knowledge Bases (Ranked)",
             fontsize=13, fontweight="bold", pad=12)

patch_pipe = mpatches.Patch(color=KWAAI_BLUE,   label="Pipeline KBs (SQLite rebuild)")
patch_d6   = mpatches.Patch(color=KWAAI_PURPLE, label="D6 — primary benchmark")
patch_pre  = mpatches.Patch(color=KWAAI_GREY,   label="Pre-pipeline KBs")
ax.legend(handles=[patch_pipe, patch_d6, patch_pre], loc="lower left", framealpha=0.9)

fig.tight_layout()
fig.savefig("chart2_recall_all.png", dpi=150)
plt.close(fig)
print("chart2 done")

# ── Figure 3: Build time vs corpus size, bubble = entities ───────────────────
fig, ax = plt.subplots(figsize=(10, 6.5))

bubble     = [e / 50 for e in entities]
dot_colors = [KWAAI_BLUE] * len(pipeline_kbs) + [KWAAI_PURPLE]

sc = ax.scatter(chunks, build_hrs, s=bubble, c=speed, cmap="RdYlGn",
                vmin=0.25, vmax=2.8, alpha=0.85,
                edgecolors=[KWAAI_PURPLE if k == "D6" else "white" for k in build_kbs],
                linewidths=[2.0 if k == "D6" else 1.2 for k in build_kbs],
                zorder=3)

texts = [
    ax.text(chunks[i], build_hrs[i], kb, fontsize=9.5,
            color=KWAAI_PURPLE if kb == "D6" else "black",
            fontweight="bold" if kb == "D6" else "normal")
    for i, kb in enumerate(build_kbs)
]
adjust_text(
    texts,
    x=chunks, y=build_hrs,
    ax=ax,
    arrowprops=dict(arrowstyle="-", color="#999", lw=0.7),
    expand=(1.6, 1.8),
    force_points=(0.6, 1.2),
    force_text=(0.5, 0.8),
    iter_lim=500,
)

cbar = fig.colorbar(sc, ax=ax, pad=0.02)
cbar.set_label("Throughput (chunks/sec)", fontsize=10)
ax.set_xlabel("Corpus size (chunks)", fontsize=11)
ax.set_ylabel("Graph build time (hours)", fontsize=11)
ax.set_title("Build Time vs Corpus Size\n(bubble size = entity count, colour = throughput)",
             fontsize=13, fontweight="bold", pad=10)
ax.yaxis.set_major_formatter(plt.FuncFormatter(
    lambda v, _: f"{v:.0f}h" if v >= 1 else f"{v*60:.0f}m"))

fig.tight_layout()
fig.savefig("chart3_build_time.png", dpi=150)
plt.close(fig)
print("chart3 done")

# ── Figure 4: D6 phase progression ───────────────────────────────────────────
fig, ax = plt.subplots(figsize=(8, 5))

ax.plot(phases, d6_recall, marker="o", markersize=9, linewidth=2.2,
        color=KWAAI_PURPLE, zorder=3)
ax.fill_between(phases, d6_recall, 80, alpha=0.12, color=KWAAI_PURPLE)
ax.axhline(80, color=KWAAI_AMBER, linewidth=1.2, linestyle="--", label="80% target floor")
ax.axhline(90, color=KWAAI_AMBER, linewidth=1.2, linestyle=":",  label="90% target ceiling")

for i, (p, v) in enumerate(zip(phases, d6_recall)):
    ax.annotate(f"{v:.1f}%", (p, v), textcoords="offset points",
                xytext=(0, 10), ha="center", fontsize=10,
                fontweight="bold", color=KWAAI_PURPLE)

improvements = [None] + [d6_recall[i] - d6_recall[i-1] for i in range(1, len(d6_recall))]
for i, (p, v, delta) in enumerate(zip(phases, d6_recall, improvements)):
    if delta:
        ax.annotate(f"+{delta:.1f}pp", (p, v), textcoords="offset points",
                    xytext=(0, -18), ha="center", fontsize=8.5, color=KWAAI_TEAL)

ax.set_ylabel("D6 Retrieval Recall (%)")
ax.set_ylim(78, 95)
ax.set_title("D6 Entity Extraction — Phase Progression\n(District Six memoir, 209 questions)",
             fontsize=13, fontweight="bold", pad=10)
ax.legend(loc="lower right", framealpha=0.9)

fig.tight_layout()
fig.savefig("chart4_d6_phases.png", dpi=150)
plt.close(fig)
print("chart4 done")

# ── Figure 5: Entity density (pipeline + D6) ─────────────────────────────────
fig, ax = plt.subplots(figsize=(10, 5))

ent_density = [e / c * 100 for e, c in zip(entities, chunks)]
bar_cols    = [KWAAI_PURPLE if kb == "D6" else
               (KWAAI_TEAL if d > 100 else KWAAI_BLUE)
               for kb, d in zip(build_kbs, ent_density)]

x = np.arange(len(build_kbs))
bars = ax.bar(x, ent_density, color=bar_cols, zorder=3)

for bar, val in zip(bars, ent_density):
    ax.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 1,
            f"{val:.0f}", ha="center", va="bottom", fontsize=9, fontweight="bold")

ax.set_xticks(x)
ax.set_xticklabels(build_kbs, fontsize=10)
ax.set_ylabel("Entities per 100 chunks")
ax.set_title("Entity Extraction Density — Pipeline KBs + D6", fontsize=13,
             fontweight="bold", pad=10)

patch_d6 = mpatches.Patch(color=KWAAI_PURPLE, label="D6 — primary benchmark")
patch_hi = mpatches.Patch(color=KWAAI_TEAL,   label=">100 entities / 100 chunks")
patch_lo = mpatches.Patch(color=KWAAI_BLUE,   label="≤100 entities / 100 chunks")
ax.legend(handles=[patch_d6, patch_hi, patch_lo], framealpha=0.9)

fig.tight_layout()
fig.savefig("chart5_entity_density.png", dpi=150)
plt.close(fig)
print("chart5 done")

# ── Combined PDF ──────────────────────────────────────────────────────────────
from matplotlib.backends.backend_pdf import PdfPages

with PdfPages("RAGPerformanceReport-20260712-charts.pdf") as pdf:
    for fname, cap in [
        ("chart1_recall_pipeline.png", "Fig 1 — Pipeline KB + D6 Recall"),
        ("chart2_recall_all.png",      "Fig 2 — All 14 KBs Ranked"),
        ("chart3_build_time.png",      "Fig 3 — Build Time vs Size"),
        ("chart4_d6_phases.png",       "Fig 4 — D6 Phase Progression"),
        ("chart5_entity_density.png",  "Fig 5 — Entity Density"),
    ]:
        img = plt.imread(fname)
        fig, ax = plt.subplots(figsize=(11, 7))
        ax.imshow(img); ax.axis("off")
        pdf.savefig(fig, bbox_inches="tight")
        plt.close(fig)

print("PDF done")
