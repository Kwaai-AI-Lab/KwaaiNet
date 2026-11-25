"""Figure 1 of the AIAS+ forgetting abstract: recall drop against memory drop, arms F, R and R2
pooled and binned by memory drop, one panel per corpus, from the 2026-11-23 temperature-0 re-score.

    python3 gen_binned_t0.py <dream_forget results dir>

Reads <run>/metrics_t0.jsonl (re-scored answers) and _recall_per_size/recall_per_size.csv (memory
per snapshot; unchanged by re-scoring). Baseline: the mean of the two re-scored cycle-0 evals,
both drawn at memory drop 0. Bins of memory drop: [0, 5%) holds cycle 0, then 10%-wide bins;
bins with fewer than 2 points are dropped. 95% intervals: percentile bootstrap within each bin.
Also writes binned_t0.csv (bin means, intervals and marginal cost between adjacent bins) beside
the figure. Only the four public runs above are read.

References (2026-11-25), from <run>/metrics_t0_extra.jsonl when scored, skipped when not:
  dotted line    conventional RAG: 20 hybrid chunks of the slice, no graph (E0 chunks-only);
  hollow marks   F, R, R2 at cycle 16 with every chunk kept (graph+all): dreaming, nothing forgotten.
Both are recall drops against the same cycle-0 baseline; the hollow marks sit just left of x = 0.
"""
import collections
import csv
import json
import random
import sys
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

RES = Path(sys.argv[1])
HERE = Path(__file__).resolve().parent
PANELS = [("Legal_p10_b40000", "Legal, 10% slice", "well structured docs"),
          ("Legal_p100_b40000", "Legal, 100%", "well structured docs"),
          ("NIST_p10_b40000", "NIST, 10% slice", "unstructured docs"),
          ("Manhattan_v3_p10_b40000", "Manhattan, 10% slice", "unstructured docs")]
ARMS = [("F", "#2a78d6", "o"), ("R", "#eb6834", "s"), ("R2", "#1baf7a", "^")]
EDGES = [0.0, 0.05, 0.15, 0.25, 0.35, 0.45, 0.55, 0.65, 0.75]
INK, INK2, GRID, BIN = "#0b0b0b", "#52514e", "#e4e3df", "#3d3a8c"
N_BOOT = 2000
R0 = {}  # cycle-0 recall per run, the baseline every drop is relative to
random.seed(0)


def mean(v):
    return sum(v) / len(v)


def pct(v, q):
    v = sorted(v)
    return v[min(len(v) - 1, max(0, int(q * len(v))))]


def points(run):
    mem = {}
    for r in csv.DictReader(open(RES / "_recall_per_size/recall_per_size.csv")):
        if r["run"] == run and r["rep"] == "r1" and (
                r["view"] == "graph+unforgotten" or (r["arm"] == "0" and r["view"] == "graph+all")):
            mem[(r["arm"], int(r["cycle"]))] = float(r["mem_chars"])
    by = collections.defaultdict(list)
    for line in open(RES / run / "metrics_t0.jsonl"):
        x = json.loads(line)
        if x.get("answer_recall") is not None:
            _, arm, c, rep, view = x["run_tag"].split(":")
            by[(arm, int(c[1:]), rep, view)].append(x["answer_recall"])
    rec = {k: mean(v) for k, v in by.items()}
    c0 = [v for (arm, c, rep, view), v in rec.items() if arm == "0"]
    r0, m0 = mean(c0), mem[("0", 0)]
    R0[run] = r0
    pts = [("c00", 0.0, 1 - v / r0) for v in c0]
    for arm, _, _ in ARMS:
        for c in (1, 2, 4, 8, 16):
            pts.append((arm, 1 - mem[(arm, c)] / m0, 1 - rec[(arm, c, "r1", "graph+unforgotten")] / r0))
    return pts


def references(run, r0):
    """Recall drops of the conventional-RAG baseline and of cycle 16 with nothing forgotten."""
    path = RES / run / "metrics_t0_extra.jsonl"
    if not path.exists():
        return None, {}
    by = collections.defaultdict(list)
    for line in open(path):
        x = json.loads(line)
        if x.get("answer_recall") is not None:
            _, arm, c, rep, view = x["run_tag"].split(":")
            by[(arm, c, view)].append(x["answer_recall"])
    rag = by.get(("E0", "c00", "chunks-only"))
    kept = {arm: 1 - mean(by[(arm, "c16", "graph+all")]) / r0
            for arm, _, _ in ARMS if by.get((arm, "c16", "graph+all"))}
    return (1 - mean(rag) / r0 if rag else None), kept


def binned(pts):
    bins = []
    for lo, hi in zip(EDGES, EDGES[1:]):
        inside = [(m, r) for _, m, r in pts if lo <= m < hi]
        if len(inside) >= 2 or (lo == 0.0 and inside):
            bins.append(((lo, hi), inside))
    return bins


plt.rcParams.update({"font.size": 7, "axes.edgecolor": GRID, "axes.labelcolor": INK2,
                     "xtick.color": INK2, "ytick.color": INK2, "axes.titlecolor": INK})
fig, axes = plt.subplots(1, 4, figsize=(7.0, 1.9), sharey=True)
out = []
for ax, (run, title, kind) in zip(axes, PANELS):
    pts = points(run)
    bins = binned(pts)
    ax.grid(True, color=GRID, lw=0.5); ax.set_axisbelow(True)
    for s in ("top", "right"):
        ax.spines[s].set_visible(False)
    for arm, colr, mk in ARMS:
        a = [(m, r) for name, m, r in pts if name == arm]
        ax.scatter([100 * m for m, _ in a], [100 * r for _, r in a], s=8, marker=mk, color=colr,
                   alpha=0.6, lw=0, label=arm)
    c0 = [r for name, _, r in pts if name == "c00"]
    ax.scatter([0] * len(c0), [100 * r for r in c0], s=10, marker="s", color=INK, zorder=4, label="cycle 0")
    xmax = max(m for _, m, _ in pts)
    ax.plot([0, 100 * xmax], [0, 100 * xmax], color=INK2, lw=0.8, ls=(0, (4, 3)))
    rag, kept = references(run, R0[run])
    if rag is not None:
        ax.axhline(100 * rag, color=INK2, lw=0.8, ls=(0, (1, 1.5)), zorder=1, label="RAG, 20 chunks")
    dx = -0.06 * 100 * xmax
    for i, (arm, colr, mk) in enumerate(ARMS):
        if arm in kept:
            ax.scatter([dx * (1 + 0.35 * i)], [100 * kept[arm]], s=12, marker=mk, facecolor="white",
                       edgecolor=colr, lw=0.9, zorder=4,
                       label="c16, nothing forgotten" if i == 0 else None)
    out.extend(dict(run=run, bin=f"ref {name}", recall_drop=round(v, 4))
               for name, v in ([("rag20", rag)] if rag is not None else []) + sorted(kept.items()))
    boots, means = [], []
    for (lo, hi), inside in bins:
        draws = []
        for _ in range(N_BOOT):
            s = [random.choice(inside) for _ in inside]
            draws.append((mean([m for m, _ in s]), mean([r for _, r in s])))
        boots.append(draws)
        mm, mr = mean([m for m, _ in inside]), mean([r for _, r in inside])
        means.append((mm, mr))
        lo_r, hi_r = pct([d[1] for d in draws], 0.025), pct([d[1] for d in draws], 0.975)
        ax.errorbar([100 * mm], [100 * mr], yerr=[[100 * (mr - lo_r)], [100 * (hi_r - mr)]], color=BIN,
                    marker="D", ms=3.5, mec="white", mew=0.5, capsize=2, lw=1.1, zorder=5)
        out.append(dict(run=run, bin=f"{lo:.2f}-{hi:.2f}", n=len(inside), memory_drop=round(mm, 4),
                        recall_drop=round(mr, 4), ci_lo=round(lo_r, 4), ci_hi=round(hi_r, 4)))
    ax.plot([100 * m for m, _ in means], [100 * r for _, r in means], color=BIN, lw=1.1, zorder=5,
            label="bin mean (95% CI)")
    for i in range(len(bins) - 1):
        dm, dr = means[i + 1][0] - means[i][0], means[i + 1][1] - means[i][1]
        ratios = [(boots[i + 1][k][1] - boots[i][k][1]) / (boots[i + 1][k][0] - boots[i][k][0])
                  for k in range(N_BOOT) if boots[i + 1][k][0] - boots[i][k][0] > 0.02]
        out.append(dict(run=run, bin=f"step {100 * means[i][0]:.0f}->{100 * means[i + 1][0]:.0f}%",
                        n=f"{len(bins[i][1])},{len(bins[i + 1][1])}", memory_drop=round(dm, 4),
                        recall_drop=round(dr, 4), marginal=round(dr / dm, 3),
                        ci_lo=round(pct(ratios, 0.025), 3), ci_hi=round(pct(ratios, 0.975), 3)))
    ax.set_title(title, loc="left", fontsize=7.5, fontweight="bold")
    ax.text(0.97, 0.96, kind, transform=ax.transAxes, ha="right", va="top", fontsize=6.5,
            style="italic", color=INK2, bbox=dict(boxstyle="round,pad=0.25", fc="white", ec=GRID, lw=0.5))
    ax.set_xlabel("memory drop (%)"); ax.set_ylim(-30, 90)
axes[0].set_ylabel("recall drop (%)")
handles, labels = axes[0].get_legend_handles_labels()
fig.legend(handles, labels, loc="lower center", ncol=len(labels), frameon=False, fontsize=6,
           labelcolor=INK, handletextpad=0.3, columnspacing=1.2, borderaxespad=0.1)
fig.tight_layout(pad=0.3, w_pad=0.6, rect=(0, 0.09, 1, 1))
for ext in ("png", "pdf"):
    fig.savefig(HERE / f"fig1_binned_t0.{ext}", dpi=300)
keys = ["run", "bin", "n", "memory_drop", "recall_drop", "marginal", "ci_lo", "ci_hi"]
with open(HERE / "binned_t0.csv", "w", newline="") as f:
    w = csv.DictWriter(f, fieldnames=keys); w.writeheader(); w.writerows(out)
for r in out:
    print(r)
