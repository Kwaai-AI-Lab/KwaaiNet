"""
D6 RAG Accuracy Progress Chart — research paper figure.

Usage:
    python3 projects/kwaai-knowledge/d6_progress_chart.py

Outputs:
    projects/kwaai-knowledge/d6_progress_chart.png  (300 dpi)
    projects/kwaai-knowledge/d6_progress_chart.pdf
"""

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
import matplotlib.ticker as mticker
import numpy as np

# ---------------------------------------------------------------------------
# Data: (milestone_label, x_position, recall_pct, phase_index)
# All percentages normalised to keyword recall % (0-100).
# x positions are sequential integers; labels are short annotation strings.
# ---------------------------------------------------------------------------

# Phase colours (colourblind-friendly palette)
PHASES = [
    ("Dense Retrieval",        "#4878D0"),  # 0 — blue
    ("Knowledge Graph",        "#EE854A"),  # 1 — orange
    ("Iterative / Dream",      "#6ACC65"),  # 2 — green
    ("Graph Enrichment",       "#D65F5F"),  # 3 — red
    ("Structured Pipeline",    "#956CB4"),  # 4 — purple
    ("Query Understanding",    "#8C613C"),  # 5 — brown
]

# (label, recall%, phase_idx, annotate)
# annotate: short string shown as a callout; None = dot only
MILESTONES = [
    # Phase 0 — Dense Retrieval (20q eval, normalised to kw%)
    ("M1",  24.6, 0, "Baseline\n(dense, 10q)"),
    ("M4",  37.9, 0, "Chunk\nfix"),
    ("M5",  43.1, 0, "all-minilm"),
    ("M9",  44.8, 0, "mxbai\nembed"),
    ("M11", 46.6, 0, "Auto\nrouter"),
    # Phase 1 — Knowledge Graph
    ("M12", 50.0, 1, "Family tree\nseed"),
    ("M14", 49.1, 1, None),
    ("M16", 51.7, 1, "k=20"),
    ("M17", 56.9, 1, "Iterative\nk=20"),
    # Phase 2 — Iterative / Dream
    ("M21", 54.3, 2, None),
    ("M22", 58.6, 2, "31 dream\ncycles"),
    ("M25", 55.2, 2, None),
    ("M27", 56.0, 2, None),
    ("M35", 59.5, 2, "Ghost\nrestore"),
    # Phase 3 — Graph Enrichment (40q eval starts here)
    ("M42", 54.4, 3, "40q eval\nbaseline"),
    ("M43", 63.1, 3, "+Org/Place\nseeds"),
    ("M45", 59.6, 3, None),
    # Phase 4 — Structured Pipeline
    ("M46", 64.9, 4, "Ordering A\npipeline"),
    ("M47", 63.6, 4, None),
    ("M48", 61.8, 4, None),
    ("M49", 63.6, 4, "name_overlap\nfix"),
    ("M50", 65.8, 4, "Relation\ninjection"),
    # Phase 5 — Query Understanding
    ("M51", 72.4, 5, "Smart\nrouting"),
    ("M52", 72.4, 5, None),
    ("M53", 68.9, 5, None),
    ("M54", 72.0, 5, None),
    ("M55", 70.2, 5, None),
    ("M56", 70.7, 5, None),
    ("M57", 71.6, 5, "HiRAG"),
    ("M58", 74.7, 5, "Bio\nexpansion"),
    ("M59", 70.2, 5, None),
    ("M60", 77.8, 5, "Post-dream\nre-eval"),
    ("M61", 76.0, 5, "Fresh\nrebuild"),
    ("M62", 76.9, 5, None),
    ("M63", 76.0, 5, None),
]

# Eval set change: from 20q/116kw to 40q/225kw between M35 and M42
EVAL_CHANGE_X = 14.5  # midpoint between M35 (x=13) and M42 (x=14)

# Noise floor: ±3.5pp (confirmed ±4pp spread, ~±3.5pp std)
NOISE = 3.5

# Representative points that get error bars (to avoid clutter)
ERROR_BAR_LABELS = {"M17", "M22", "M35", "M43", "M50", "M51", "M58", "M60"}

# ---------------------------------------------------------------------------
# Build arrays
# ---------------------------------------------------------------------------
xs = np.arange(len(MILESTONES), dtype=float)
ys = np.array([m[1] for m in MILESTONES])
phases = [m[2] for m in MILESTONES]

# Remap EVAL_CHANGE_X: index-based position
# M35 is index 13, M42 is index 14 → change at x = 13.5
EVAL_CHANGE_IDX = 13.5

# ---------------------------------------------------------------------------
# Figure
# ---------------------------------------------------------------------------
fig, ax = plt.subplots(figsize=(8.5, 4.2))
fig.subplots_adjust(left=0.09, right=0.97, top=0.90, bottom=0.22)

# Target band (80–90%)
ax.axhspan(80, 90, alpha=0.10, color="#228B22", zorder=0)
ax.text(len(MILESTONES) - 0.3, 85, "Target\n80–90%", va="center", ha="right",
        fontsize=7, color="#228B22", style="italic")

# Eval set change marker
ax.axvline(EVAL_CHANGE_IDX, color="#555555", linewidth=1.0, linestyle="--", zorder=1)
ax.text(EVAL_CHANGE_IDX + 0.15, 27, "Eval set\n20q → 40q",
        fontsize=6.5, color="#555555", va="bottom")

# Draw phase-coloured line segments
phase_start = 0
for i in range(1, len(MILESTONES)):
    if phases[i] != phases[phase_start] or i == len(MILESTONES) - 1:
        end = i if phases[i] != phases[phase_start] else i + 1
        seg_x = xs[phase_start:end]
        seg_y = ys[phase_start:end]
        colour = PHASES[phases[phase_start]][1]
        ax.plot(seg_x, seg_y, color=colour, linewidth=1.6, zorder=2,
                solid_capstyle="round", solid_joinstyle="round")
        phase_start = i

# Dots
for i, (label, recall, phase_idx, ann) in enumerate(MILESTONES):
    colour = PHASES[phase_idx][1]
    ax.scatter(xs[i], recall, color=colour, s=22, zorder=4, linewidths=0)

# Error bars on representative points
for i, (label, recall, phase_idx, ann) in enumerate(MILESTONES):
    if label in ERROR_BAR_LABELS:
        colour = PHASES[phase_idx][1]
        ax.errorbar(xs[i], recall, yerr=NOISE, fmt="none",
                    ecolor=colour, elinewidth=0.8, capsize=2.5, zorder=3, alpha=0.7)

# Annotations (callouts above/below key milestones)
# (milestone_index, y_offset, ha)
ANNO_POS = {
    "M1":  (-8,  "center"),
    "M4":  (+5,  "center"),
    "M5":  (-8,  "center"),
    "M9":  (+5,  "left"),
    "M11": (-8,  "center"),
    "M12": (+5,  "center"),
    "M16": (-9,  "center"),
    "M17": (+5,  "center"),
    "M22": (+5,  "center"),
    "M35": (+5,  "center"),
    "M42": (-10, "center"),
    "M43": (+5,  "center"),
    "M46": (+5,  "center"),
    "M49": (-9,  "center"),
    "M50": (+5,  "center"),
    "M51": (+5,  "center"),
    "M57": (-9,  "center"),
    "M58": (+5,  "center"),
    "M60": (+5,  "center"),
}

for i, (label, recall, phase_idx, ann) in enumerate(MILESTONES):
    if ann and label in ANNO_POS:
        y_off, ha = ANNO_POS[label]
        colour = PHASES[phase_idx][1]
        ax.annotate(
            ann,
            xy=(xs[i], recall),
            xytext=(xs[i], recall + y_off),
            ha=ha, va="top" if y_off < 0 else "bottom",
            fontsize=5.8,
            color=colour,
            arrowprops=dict(arrowstyle="-", color=colour, lw=0.6),
        )

# Axes
ax.set_xlim(-0.5, len(MILESTONES) - 0.5)
ax.set_ylim(15, 95)
ax.set_ylabel("Keyword Recall (%)", fontsize=9)
ax.set_xlabel("Experiment Milestone", fontsize=9)

# X-tick labels: major milestones only
tick_labels = [m[0] if m[0] in {
    "M1", "M4", "M9", "M12", "M17", "M22", "M35",
    "M42", "M43", "M46", "M50", "M51", "M58", "M60"
} else "" for m in MILESTONES]
ax.set_xticks(xs)
ax.set_xticklabels(tick_labels, fontsize=7.5, rotation=0)

ax.yaxis.set_major_formatter(mticker.FormatStrFormatter("%d%%"))
ax.yaxis.set_major_locator(mticker.MultipleLocator(10))
ax.yaxis.set_minor_locator(mticker.MultipleLocator(5))
ax.tick_params(axis="y", labelsize=8)

# Spine cleanup
ax.spines["top"].set_visible(False)
ax.spines["right"].set_visible(False)
ax.grid(axis="y", color="#DDDDDD", linewidth=0.5, zorder=0)
ax.grid(axis="y", which="minor", color="#EEEEEE", linewidth=0.3, zorder=0)

# Legend
legend_patches = [
    mpatches.Patch(color=PHASES[i][1], label=PHASES[i][0])
    for i in range(len(PHASES))
]
ax.legend(
    handles=legend_patches,
    loc="upper left",
    fontsize=6.8,
    framealpha=0.85,
    edgecolor="#CCCCCC",
    handlelength=1.2,
    handleheight=0.8,
    borderpad=0.5,
    labelspacing=0.3,
)

# Title
ax.set_title(
    "Knowledge Graph RAG: Retrieval Accuracy on D6 Memoir (llama3.1:8b)",
    fontsize=9.5, pad=8
)

# ---------------------------------------------------------------------------
# Save
# ---------------------------------------------------------------------------
out_base = "projects/kwaai-knowledge/d6_progress_chart"
fig.savefig(out_base + ".png", dpi=300, bbox_inches="tight")
fig.savefig(out_base + ".pdf", bbox_inches="tight")
print(f"Saved {out_base}.png and {out_base}.pdf")
