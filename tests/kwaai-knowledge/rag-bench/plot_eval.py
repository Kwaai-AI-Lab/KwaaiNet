#!/usr/bin/env python3
"""
Parse kwaainet rag eval output and render a results chart.
Usage: python3 plot_eval.py <eval_log.md> [--out chart.png]
"""
import sys, re, json, argparse
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
import numpy as np

def parse_eval_log(path):
    """Extract per-question results from kwaainet rag eval output."""
    text = Path(path).read_text()

    questions = []
    # Each question block starts with the question number line, e.g.:
    # Q01  What does TLSA stand for?
    # followed by keyword hit lines and a score line
    # Pattern varies by version; parse liberally.
    #
    # Look for lines like:
    #   [Q01] score=2/2  or  Q01: 2/2  or  Score: 1/2
    # and per-question summaries in JSON / text form.

    # Try to find JSON summary block first
    json_match = re.search(r'\{[^{}]*"results"\s*:', text, re.DOTALL)
    if json_match:
        try:
            data = json.loads(text[json_match.start():])
            return data.get("results", []), data.get("total_score", 0), data.get("total_possible", 0)
        except Exception:
            pass

    # Fallback: parse text output
    # Lines like: "  Q01  factual   2/2  ✓  What does TLSA stand for?"
    #         or: "  q01  2/2"
    results = []
    for line in text.splitlines():
        m = re.match(
            r'\s*(?P<id>[Qq]\d+)\s+'
            r'(?P<type>\w+)?\s*'
            r'(?P<hits>\d+)/(?P<total>\d+)'
            r'.*?(?P<q>.+)?',
            line.strip()
        )
        if m:
            hits  = int(m.group("hits"))
            total = int(m.group("total"))
            results.append({
                "id":    m.group("id").upper(),
                "type":  m.group("type") or "unknown",
                "hits":  hits,
                "total": total,
                "pct":   hits / total if total > 0 else None,
            })

    # Extract overall score from last "Score: X/Y (Z%)" line
    overall_match = re.findall(r'(?:Score|score)[:\s]+(\d+)/(\d+)', text)
    if overall_match:
        hits, total = map(int, overall_match[-1])
    else:
        hits  = sum(r["hits"]  for r in results)
        total = sum(r["total"] for r in results if r["total"] > 0)

    return results, hits, total


def plot(results, hits, total, out_path):
    scored   = [r for r in results if r["total"] > 0]
    unscored = [r for r in results if r["total"] == 0]

    # Category colour map
    cat_colours = {
        "factoid":    "#4c78a8",
        "factual":    "#4c78a8",
        "inferential":"#f58518",
        "out_of_scope":"#e45756",
        "unknown":    "#aaaaaa",
    }

    fig, axes = plt.subplots(
        1, 2,
        figsize=(14, 6),
        gridspec_kw={"width_ratios": [3, 1]},
    )
    fig.suptitle("ragbench eval — kwaainet RAG", fontsize=14, fontweight="bold")

    # ── Left: per-question bar chart ──────────────────────────────────────
    ax = axes[0]
    if scored:
        ids   = [r["id"]  for r in scored]
        pcts  = [r["pct"] * 100 for r in scored]
        cats  = [r["type"] for r in scored]
        colours = [cat_colours.get(c, "#aaaaaa") for c in cats]

        x = np.arange(len(ids))
        bars = ax.bar(x, pcts, color=colours, edgecolor="white", linewidth=0.5, zorder=3)
        ax.set_xticks(x)
        ax.set_xticklabels(ids, rotation=45, ha="right", fontsize=8)
        ax.set_ylabel("Keyword recall (%)")
        ax.set_ylim(0, 110)
        ax.axhline(100, color="#555", linewidth=0.7, linestyle="--", label="Perfect")
        ax.set_title(f"Per-question scores  ({len(scored)} scored, {len(unscored)} no keywords)")
        ax.grid(axis="y", alpha=0.3, zorder=0)

        # Value labels on bars
        for bar, pct in zip(bars, pcts):
            ax.text(
                bar.get_x() + bar.get_width() / 2,
                bar.get_height() + 2,
                f"{pct:.0f}%",
                ha="center", va="bottom", fontsize=7,
            )

        legend_patches = [
            mpatches.Patch(color=v, label=k.capitalize())
            for k, v in cat_colours.items()
            if k in cats
        ]
        ax.legend(handles=legend_patches, fontsize=8, loc="upper right")
    else:
        ax.text(0.5, 0.5, "No scored questions\n(all have empty expected_keywords)",
                ha="center", va="center", transform=ax.transAxes, fontsize=11)
        ax.set_title("Per-question scores")

    # ── Right: overall gauge vs D6 baseline ──────────────────────────────
    ax2 = axes[1]
    overall_pct = hits / total * 100 if total > 0 else 0
    d6_pct      = 88.9

    kbs    = ["D6\n(baseline)", "ragbench"]
    scores = [d6_pct, overall_pct]
    bar_colours = ["#54a24b", "#4c78a8"]

    b2 = ax2.bar(kbs, scores, color=bar_colours, edgecolor="white", linewidth=0.5, zorder=3)
    ax2.set_ylim(0, 110)
    ax2.set_ylabel("Keyword recall (%)")
    ax2.set_title("Overall score vs D6")
    ax2.grid(axis="y", alpha=0.3, zorder=0)
    ax2.axhline(d6_pct, color="#54a24b", linewidth=1, linestyle="--", alpha=0.6)

    for bar, pct in zip(b2, scores):
        ax2.text(
            bar.get_x() + bar.get_width() / 2,
            bar.get_height() + 2,
            f"{pct:.1f}%",
            ha="center", va="bottom", fontsize=12, fontweight="bold",
        )

    if total > 0:
        ax2.text(
            0.5, 0.05,
            f"{hits}/{total} keywords matched\n({len(unscored)} questions unscored)",
            ha="center", va="bottom", transform=ax2.transAxes,
            fontsize=8, color="#555",
        )

    plt.tight_layout()
    plt.savefig(out_path, dpi=150, bbox_inches="tight")
    print(f"Chart saved: {out_path}")


if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("log", help="Path to kwaainet rag eval output file")
    ap.add_argument("--out", default="/tmp/ragbench_eval_chart.png")
    args = ap.parse_args()

    results, hits, total = parse_eval_log(args.log)
    if not results and total == 0:
        print("No scored results found in log — check file format")
        sys.exit(1)

    overall = hits / total * 100 if total > 0 else 0
    print(f"Overall: {hits}/{total} ({overall:.1f}%)")
    print(f"Scored questions: {sum(1 for r in results if r['total'] > 0)}")
    plot(results, hits, total, args.out)
