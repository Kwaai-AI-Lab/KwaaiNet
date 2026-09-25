#!/usr/bin/env python3
"""Figures for the Dream RAG AIAS+ 2026 revision (see ../DreamRAG-AIAS2026-manuscript-plan.md).

Every number is parsed from, or copied with a citation to, a result file under
tests/kwaai-knowledge/results/. Output: fig{2..7}_*.{pdf,png} next to this script, named by
paper figure number (order of first mention). Function names fig2()..fig7() are historical:
fig2→Figure 5, fig3→3, fig4→2, fig5→6, fig6→4, fig7→7.
Labels are anonymized: no project, machine or memoir names.

    python3 projects/kwaai-knowledge/plans/figures/gen_paper_figures.py
"""
from __future__ import annotations

import json
import re
import statistics as st
from datetime import datetime
from pathlib import Path

import matplotlib.pyplot as plt
from matplotlib.lines import Line2D

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[3]
RES = REPO / "tests/kwaai-knowledge/results"

# Reference palette (dataviz skill), slots 1-3 validated all-pairs in light mode.
BLUE, ORANGE, AQUA = "#2a78d6", "#eb6834", "#1baf7a"
INK, INK2, MUTED, GRID = "#0b0b0b", "#52514e", "#8a8984", "#e6e5e1"

plt.rcParams.update({
    "font.family": "sans-serif",
    "font.sans-serif": ["Helvetica", "Arial", "DejaVu Sans"],
    "font.size": 8,
    "axes.titlesize": 8.5,
    "axes.labelsize": 8,
    "axes.edgecolor": MUTED,
    "axes.labelcolor": INK2,
    "axes.titlecolor": INK,
    "axes.titleweight": "bold",
    "axes.titlelocation": "left",
    "xtick.color": INK2,
    "ytick.color": INK2,
    "axes.spines.top": False,
    "axes.spines.right": False,
    "axes.grid": True,
    "grid.color": GRID,
    "grid.linewidth": 0.6,
    "axes.axisbelow": True,
    "legend.frameon": False,
    "pdf.fonttype": 42,  # embed TrueType, required by ACM
    "savefig.dpi": 300,
})

ANSI = re.compile(r"\x1b\[[0-9;]*m")

# Anonymized display names for the corpora.
NAMES = {
    "Manhattan": "Manhattan Project",
    "MobyDick": "Moby-Dick & companions",
    "Legal": "Legal opinions",
    "Meetings": "Meeting transcripts",
    "PythonDocs": "Python documentation",
    "NIST": "AI security standards",
    "Climate": "Climate science",
    "RFCs": "Internet RFCs",
    "DeepSea": "Deep-sea biology",
    "DreamMem": "Sleep & memory papers",
    "Astrophysics": "Astrophysics",
    "WarPeace": "War and Peace",
    "OSMDocs": "Map-data documentation",
    "CountryHistory": "Country history",
    "Poems": "Poetry",
    "D6": "Memoir",
}


def save(fig, stem: str) -> None:
    for ext in ("pdf", "png"):
        fig.savefig(HERE / f"{stem}.{ext}", bbox_inches="tight", facecolor="white")
    plt.close(fig)
    print("wrote", stem)


# ── Figure 5: completeness vs retrieval accuracy over 31 dream cycles ────────
def fig2() -> None:
    rows = json.loads((RES / "dream_scores.json").read_text())
    comp = [(r["cycle"], r["graph_score"]) for r in rows if r.get("graph_score") is not None]
    # Cycle 12 recorded eval 0.0: a failed run, not a measurement. Excluded.
    ev = [(r["cycle"], r["eval_score"]) for r in rows
          if r.get("eval_score") and r["cycle"] != 12]
    vals = [e for _, e in ev]
    mu, sd = st.mean(vals), st.stdev(vals)

    fig, (a, b) = plt.subplots(2, 1, figsize=(3.4, 3.9), sharex=True,
                               gridspec_kw={"height_ratios": [1, 1], "hspace": 0.55})
    for ax in (a, b):
        # Cycles 1-9 used a 3B completion model, 10+ an 8B model (d6_accuracy_progress.md).
        ax.axvspan(0.5, 9.5, color="#f1f0ec", zorder=0, lw=0)

    a.plot(*zip(*comp), color=BLUE, lw=2, marker="o", ms=2.5, markeredgewidth=0)
    a.set_ylabel("Graph completeness (%)")
    a.set_title("(a) Structural completeness")
    a.text(5, 74, "3B completion\nmodel", ha="center", va="top", fontsize=7, color=INK2)
    a.text(20, 60, "8B completion model", ha="center", fontsize=7, color=INK2)
    a.set_ylim(48, 82)
    a.tick_params(labelbottom=True)
    a.set_xlabel("Dream cycle")

    b.axhspan(mu - sd, mu + sd, color=ORANGE, alpha=0.12, lw=0)
    b.axhline(mu, color=ORANGE, lw=1, ls="--")
    b.plot(*zip(*ev), ls="none", marker="o", ms=5, color=ORANGE,
           markeredgecolor="white", markeredgewidth=0.8)
    b.text(31.3, mu, f"mean {mu:.1f}%\n±1 SD", va="center", fontsize=7, color=INK2)
    b.set_ylabel("Keyword recall (%)")
    b.set_title("(b) Retrieval-augmented answer accuracy")
    b.set_ylim(40, 70)
    b.set_xlabel("Dream cycle")
    b.set_xlim(-0.5, 31.5)
    b.set_xticks(range(0, 32, 4))
    save(fig, "fig5_completeness_vs_accuracy")
    print(f"  fig2: n_eval={len(ev)} mean={mu:.2f} sd={sd:.2f}")


# ── Figure 3: fact density — entities down, completeness up ──────────────────
def parse_dream_log(path: Path) -> tuple[int, int, float, float]:
    t = ANSI.sub("", path.read_text(errors="ignore"))
    ents = [int(m) for m in re.findall(r"(\d+) entities, \d+ relations", t)]
    pcts = [float(x) for line in t.splitlines() if re.search(r"health|score", line, re.I)
            for x in re.findall(r"(\d+\.\d+)%", line)]
    # Health lines print "before% after% delta%"; first value = initial, second-to-last = final.
    return ents[0], ents[-1], pcts[0], pcts[-2]


def fig3() -> None:
    data = {}
    for p in sorted(RES.glob("rebuild_dream_*.log")):
        data[p.stem.replace("rebuild_dream_", "")] = parse_dream_log(p)
    data["WarPeace"] = parse_dream_log(RES / "warpeace_rebuild_dream_timeline.log")

    rows = sorted(data.items(), key=lambda kv: (kv[1][1] - kv[1][0]) / kv[1][0], reverse=True)
    labels = [NAMES[k] for k, _ in rows]
    y = range(len(rows))
    dent = [100 * (e1 - e0) / e0 for _, (e0, e1, _, _) in rows]
    h0 = [h for _, (_, _, h, _) in rows]
    h1 = [h for _, (_, _, _, h) in rows]

    fig, (a, b) = plt.subplots(1, 2, figsize=(7.0, 3.1), sharey=True,
                               gridspec_kw={"width_ratios": [1, 1.25], "wspace": 0.08})
    a.barh(y, dent, color=BLUE, height=0.62)
    for yi, d in zip(y, dent):
        a.text(d - 0.6, yi, f"{d:.0f}%", va="center", ha="right", fontsize=7, color=INK2)
    a.set_yticks(list(y), labels)
    a.set_xlim(-31, 0)
    a.axvline(0, color=MUTED, lw=0.8)
    a.set_xlabel("Change in entity count (%)")
    a.set_title("(a) Fewer entities")
    a.grid(axis="y", visible=False)

    for yi, s, e in zip(y, h0, h1):
        b.plot([s, e], [yi, yi], color=MUTED, lw=1.2, zorder=1)
    b.scatter(h0, list(y), s=26, facecolor="white", edgecolor=ORANGE, lw=1.4, zorder=2)
    b.scatter(h1, list(y), s=26, color=ORANGE, edgecolor="white", lw=0.6, zorder=3)
    b.set_xlabel("Mean entity completeness (%)")
    b.set_title("(b) More complete entities")
    b.grid(axis="y", visible=False)
    b.legend(handles=[
        Line2D([], [], ls="none", marker="o", mfc="white", mec=ORANGE, mew=1.4, ms=5.5,
               label="before cycle 1"),
        Line2D([], [], ls="none", marker="o", color=ORANGE, ms=5.5, label="after cycle 5"),
    ], loc="lower right", fontsize=7)
    save(fig, "fig3_fact_density")
    med = st.median(dent)
    up = sum(e > s for s, e in zip(h0, h1))
    print(f"  fig3: n={len(rows)} entity change {min(dent):.1f}..{max(dent):.1f}, "
          f"median {med:.1f}; completeness up on {up}/{len(rows)}")


# ── Figure 2: cross-corpus retrieval / generation / judge ────────────────────
def fig4() -> None:
    pat = re.compile(r"✅ (\w+): ret=([\d.]+)% gen=([\d.]+)% judge=([\d.]+)/2")
    rows = [(m[1], float(m[2]), float(m[3]), float(m[4]))
            for m in pat.finditer((RES / "multi_corpus_eval_full_driver.log").read_text())]
    rows.sort(key=lambda r: r[1])
    labels = [NAMES[r[0]] for r in rows]
    y = list(range(len(rows)))

    fig, (a, b) = plt.subplots(1, 2, figsize=(7.0, 3.2), sharey=True,
                               gridspec_kw={"width_ratios": [2.2, 1], "wspace": 0.12})
    hgt = 0.36
    a.barh([i + hgt / 2 + 0.02 for i in y], [r[1] for r in rows], height=hgt, color=BLUE,
           label="Retrieval recall (retrieved passages)")
    a.barh([i - hgt / 2 - 0.02 for i in y], [r[2] for r in rows], height=hgt, color=ORANGE,
           label="Answer recall (generated answer)")
    for i, r in zip(y, rows):
        a.text(r[1] + 0.6, i + hgt / 2 + 0.02, f"{r[1]:.1f}", va="center", fontsize=6.5, color=INK2)
        a.text(r[2] + 0.6, i - hgt / 2 - 0.02, f"{r[2]:.1f}", va="center", fontsize=6.5, color=INK2)
    a.set_yticks(y, labels)
    a.set_xlim(50, 100)
    a.set_xlabel("Keyword recall (%)")
    a.set_title("(a) Recall")
    a.grid(axis="y", visible=False)
    a.legend(loc="upper center", bbox_to_anchor=(0.5, -0.16), ncol=2, fontsize=7)

    b.scatter([r[3] for r in rows], y, s=28, color=INK2, zorder=3)
    for i, r in zip(y, rows):
        b.plot([1.0, r[3]], [i, i], color=GRID, lw=1.2, zorder=1)
        b.text(r[3] + 0.03, i, f"{r[3]:.2f}", va="center", fontsize=6.5, color=INK2)
    b.set_xlim(1.0, 2.0)
    b.set_xticks([1.2, 1.4, 1.6, 1.8, 2.0])
    b.set_xlabel("LLM-judge score (0–2)")
    b.set_title("(b) Judge")
    b.grid(axis="y", visible=False)
    save(fig, "fig2_cross_corpus")
    print(f"  fig4: n={len(rows)} ret {min(r[1] for r in rows)}..{max(r[1] for r in rows)}")


# ── Figure 6: cost — graph build vs corpus size; dream cycle vs graph size ───
def dream_cycle_minutes() -> dict[str, list[float]]:
    out: dict[str, list[float]] = {}
    for name in ("overnight_dream_timeline_driver.log", "overnight_dream_timeline_driver_tail.log"):
        kb, marks = None, []
        for line in (RES / name).read_text().splitlines():
            m = re.match(r"\[(.+?)\] (.*)", line)
            if not m:
                continue
            t, msg = datetime.strptime(m[1], "%Y-%m-%d %H:%M:%S"), m[2]
            if msg.startswith("KB: "):
                kb, marks = msg[4:].strip(), []
            elif "dream cycle" in msg or "timeline build" in msg:
                marks.append(t)
                if "timeline build" in msg and len(marks) == 6:
                    out[kb] = [(marks[i + 1] - marks[i]).seconds / 60 for i in range(5)]
    return out


def parse_build_log(path: Path) -> tuple[int, int]:
    t = ANSI.sub("", path.read_text(errors="replace"))
    n, _, secs = re.findall(r"\[\s*(\d+)/(\d+)\]\s+entities=\s*\d+\s+rels=\s*\d+\s+elapsed=(\d+)s", t)[-1]
    return int(n), int(secs)


def fig5() -> None:
    # Build cost: the logged graph builds behind Figures 3-4 (last progress line of each log).
    logs = {p.stem.replace("rebuild_dream_", ""): p for p in RES.glob("rebuild_dream_*.log")}
    logs["WarPeace"] = RES / "warpeace_rebuild_dream_timeline.log"
    build = {k: parse_build_log(p) for k, p in logs.items()}  # kb: (chunks, build_seconds)
    # Graph size at the start of the overnight cycles = entity count after the rebuild cycles.
    ents = {p.stem.replace("rebuild_dream_", ""): parse_dream_log(p)[1]
            for p in RES.glob("rebuild_dream_*.log")}
    mins = dream_cycle_minutes()

    fig, (a, b) = plt.subplots(1, 2, figsize=(7.0, 2.8), gridspec_kw={"wspace": 0.3})
    xs = [v[0] for v in build.values()]
    ys = [v[1] / 3600 for v in build.values()]
    a.scatter(xs, ys, s=30, color=BLUE, edgecolor="white", lw=0.8, zorder=3)
    aoffs = {"Manhattan": (4, 3), "Meetings": (4, 3), "PythonDocs": (4, 3), "RFCs": (4, -8),
             "Astrophysics": (-5, 1), "WarPeace": (4, 3), "MobyDick": (-5, -3)}
    for k, off in aoffs.items():
        c, s = build[k]
        a.annotate(NAMES[k], (c, s / 3600), xytext=off, textcoords="offset points",
                   ha="right" if off[0] < 0 else "left", fontsize=6.5, color=INK2)
    print("  fig5 build:", {k: (c, round(s / 3600, 1), round(c / s, 2)) for k, (c, s) in sorted(build.items())})
    a.set_xscale("log")
    a.set_yscale("log")
    a.set_xlabel("Corpus size (passages, log)")
    a.set_ylabel("Graph build time (h, log)")
    a.set_title("(a) Ingestion and graph construction")

    kbs = sorted(mins, key=lambda k: ents[k])
    for k in kbs:
        a_x = ents[k]
        b.scatter([a_x] * 5, mins[k], s=10, color=ORANGE, alpha=0.45, lw=0, zorder=2)
        b.scatter([a_x], [st.median(mins[k])], s=30, color=ORANGE, edgecolor="white", lw=0.8, zorder=3)
    offs = {"Manhattan": (4, 4), "MobyDick": (-4, 7), "Legal": (4, -9), "RFCs": (4, 4), "NIST": (6, 26)}
    for k, off in offs.items():
        b.annotate(NAMES[k], (ents[k], st.median(mins[k])), xytext=off,
                   ha="right" if off[0] < 0 else "left",
                   arrowprops=dict(arrowstyle="-", color=MUTED, lw=0.6) if k == "NIST" else None,
                   textcoords="offset points", fontsize=6.5, color=INK2)
    b.set_xscale("log")
    b.set_ylim(0, 24)
    b.set_xlabel("Graph size (entities, log)")
    b.set_ylabel("Minutes per dream cycle")
    b.set_title("(b) Dream cycle, 200-completion budget")
    b.legend(handles=[
        Line2D([], [], ls="none", marker="o", color=ORANGE, alpha=0.45, ms=3.5, label="each cycle"),
        Line2D([], [], ls="none", marker="o", color=ORANGE, ms=5.5, label="median of 5"),
    ], loc="lower left", fontsize=7, ncol=2)
    save(fig, "fig6_cost")
    for k in kbs:
        print(f"  fig5: {k:13s} entities={ents[k]:6d} median={st.median(mins[k]):.1f} min")


# ── Figure 4: completeness per dream cycle, 12 corpora ───────────────────────
def completeness_series(kb: str) -> list[float]:
    """Cycle 0 (after build) through the last cycle: rebuild log, then the overnight log."""
    first = RES / ("warpeace_rebuild_dream_timeline.log" if kb == "WarPeace" else f"rebuild_dream_{kb}.log")
    pat = re.compile(r"Overall:\s+([\d.]+)%")
    vals = [float(x) for x in pat.findall(ANSI.sub("", first.read_text(errors="replace")))]
    night = RES / f"overnight_dream_timeline_{kb}.log"
    if night.exists():  # one line per completed cycle, no cycle-0 line
        vals += [float(x) for x in pat.findall(ANSI.sub("", night.read_text(errors="replace")))]
    return vals


def fig6() -> None:
    kbs = [p.stem.replace("rebuild_dream_", "") for p in RES.glob("rebuild_dream_*.log")] + ["WarPeace"]
    series = {k: completeness_series(k) for k in kbs}
    order = sorted(series, key=lambda k: series[k][-1] - series[k][0], reverse=True)
    fig, axes = plt.subplots(3, 4, figsize=(7.0, 4.6), sharex=True, sharey=True,
                             gridspec_kw={"hspace": 0.45, "wspace": 0.12})
    for ax, k in zip(axes.flat, order):
        v = series[k]
        first_run = 3 if k == "Manhattan" else 5  # cycles in the rebuild session
        if len(v) - 1 > first_run:  # later cycles ran in a separate overnight session
            ax.axvspan(first_run + 0.5, len(v) - 0.5, color="#f1f0ec", zorder=0, lw=0)
        ax.plot(range(len(v)), v, color=BLUE, lw=2, marker="o", ms=2.5, markeredgewidth=0)
        ax.set_title(NAMES[k], fontsize=7.5)
        ax.text(0.04, 0.9, f"{v[0]:.1f} → {v[-1]:.1f}", transform=ax.transAxes, fontfamily="DejaVu Sans",
                ha="left", va="top", fontsize=6.5, color=INK2)
    axes.flat[0].set_xlim(-0.5, 10.5)
    axes.flat[0].set_xticks([0, 5, 10])
    axes.flat[0].set_ylim(35, 50)
    for ax in axes[:, 0]:
        ax.set_ylabel("Completeness (%)")
    for ax in axes[-1, :]:
        ax.set_xlabel("Dream cycle")
    save(fig, "fig4_completeness_per_cycle")
    for k in order:
        print(f"  fig6: {k:13s} cycles={len(series[k]) - 1:2d} {series[k][0]:.1f} -> {series[k][-1]:.1f}")


# ── Figure 7: development history on the memoir, with its confounds marked ───
def milestones() -> list[tuple[str, float]]:
    """(label, recall %) from projects/kwaai-knowledge/d6_progress_chart.py, read without running it."""
    import ast
    src = (REPO / "projects/kwaai-knowledge/d6_progress_chart.py").read_text()
    for node in ast.parse(src).body:
        if isinstance(node, ast.Assign) and getattr(node.targets[0], "id", "") == "MILESTONES":
            return [(m[0], m[1]) for m in ast.literal_eval(node.value)]
    raise ValueError("MILESTONES not found")


def fig7() -> None:
    ms = milestones()
    idx = {lab: i for i, (lab, _) in enumerate(ms)}
    # Milestones whose only change was running dream cycles (d6_accuracy_progress.md rows).
    # Excluded as mixed: M22 (+ sanitize, section-aware ingest), M68 and M70 (+ new seed
    # entities), M74 (YAML re-seed after the cycle).
    DREAM = {"M59", "M62", "M63", "M64", "M67"}
    # Milestones whose only change was to the hand-curated seed file (commit diffs touch only
    # tests/kwaai-knowledge/d6_family_tree.yaml, or the progress row says seed-only).
    SEED = {"M43", "M72", "M73", "M83", "M85", "M86", "M87", "M88"}
    # Changes to the instrument itself: (boundary after this milestone, label).
    MARKS = [("M35", "question set\n20 to 40 questions"),
             ("M50", "generation at\ntemperature 0"),
             ("M60", "scorer adds\nyear-proximity credit")]

    xs = list(range(len(ms)))
    ys = [y for _, y in ms]
    fig, ax = plt.subplots(figsize=(7.0, 3.0))
    for lab, text in MARKS:
        x = idx[lab] + 0.5
        ax.axvline(x, color=MUTED, lw=0.8, ls="--", zorder=1)
        left = lab == "M35"
        ax.text(x - 0.4 if left else x + 0.4, 21, text, fontsize=6.5, color=INK2, va="bottom",
                ha="right" if left else "left")
    ax.plot(xs, ys, color=GRID, lw=1.5, zorder=2)
    other = [i for i, (lab, _) in enumerate(ms) if lab not in DREAM | SEED]
    ax.scatter(other, [ys[i] for i in other], s=12, color=MUTED, zorder=3, lw=0)
    for group, color in ((DREAM, ORANGE), (SEED, AQUA)):
        pts = [idx[l] for l in group]
        ax.scatter(pts, [ys[i] for i in pts], s=30, color=color, edgecolor="white", lw=0.8, zorder=4)
    a, b = idx["M59"], idx["M60"]
    ax.annotate("same graph as the\nprevious point", xy=(b, ys[b]), xytext=(b - 3.5, 89),
                fontsize=6.5, color=INK2, ha="center",
                arrowprops=dict(arrowstyle="-", color=MUTED, lw=0.6))
    ax.set_xlim(-1, len(ms))
    ax.set_ylim(20, 100)
    ticks = [i for i, (lab, _) in enumerate(ms) if lab in {"M1", "M17", "M35", "M50", "M60", "M74", "M88"}]
    ax.set_xticks(ticks)
    ax.set_xticklabels([ms[i][0] for i in ticks])
    ax.set_xlabel("Development milestone (chronological)")
    ax.set_ylabel("Answer keyword recall (%)")
    ax.legend(handles=[
        Line2D([], [], ls="none", marker="o", color=MUTED, ms=3.5, label="code or configuration change"),
        Line2D([], [], ls="none", marker="o", color=ORANGE, ms=5.5, label="dream cycles only"),
        Line2D([], [], ls="none", marker="o", color=AQUA, ms=5.5, label="curated seed edits only"),
    ], loc="lower right", fontsize=7)
    save(fig, "fig7_development_history")
    for lab in sorted(DREAM | SEED, key=lambda l: idx[l]):
        i = idx[lab]
        print(f"  fig7: {lab:4s} {'dream' if lab in DREAM else 'seed '} {ys[i - 1]:5.1f} -> {ys[i]:5.1f}")


if __name__ == "__main__":
    fig2()
    fig3()
    fig4()
    fig5()
    fig6()
    fig7()
