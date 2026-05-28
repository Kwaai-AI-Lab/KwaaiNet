#!/usr/bin/env python3
"""
Track graph completion score vs eval keyword hit rate across dream cycles.
Saves data to dream_scores.json; renders a matplotlib PNG after each cycle.

Usage:
    python3 tests/dream_plot.py [--cycles N] [--kb D6] [--peer PEER_ID]
"""
import argparse
import json
import re
import subprocess
import sys
from datetime import datetime
from pathlib import Path

DATA_FILE = Path(__file__).parent / "dream_scores.json"
PLOT_FILE = Path(__file__).parent / "dream_scores.png"

PEER = "p2p://12D3KooWDyPJBavUudh6dWitszGL2FSrEgy32SJY5qiSrATapGgd"
MODEL = "llama3.1:8b"
EVAL_Q = (Path(__file__).parent / "d6_eval_questions.json").resolve()


def run(cmd: list[str]) -> str:
    result = subprocess.run(cmd, capture_output=True, text=True)
    return result.stdout + result.stderr


def parse_dream_score(output: str) -> tuple[float | None, float | None]:
    """Return (score_before, score_after) from dream run output."""
    m = re.search(r"Score:\s+([\d.]+)%\s*→\s*([\d.]+)%", output)
    if m:
        return float(m.group(1)), float(m.group(2))
    return None, None


def parse_eval_score(output: str) -> float | None:
    """Return keyword hit rate % from eval output."""
    # looks for: "Hit rate: 65.0%" or "Overall: 65.0%"
    for pattern in [
        r"Hit rate[:\s]+([\d.]+)%",
        r"Overall[:\s]+([\d.]+)%",
        r"overall.*?([\d.]+)%",
        r"(\d+)/(\d+) keywords",
    ]:
        m = re.search(pattern, output, re.IGNORECASE)
        if m:
            if m.lastindex == 2:  # fraction form
                return 100 * int(m.group(1)) / int(m.group(2))
            return float(m.group(1))
    return None


def load_data() -> list[dict]:
    if DATA_FILE.exists():
        return json.loads(DATA_FILE.read_text())
    return []


def save_data(data: list[dict]) -> None:
    DATA_FILE.write_text(json.dumps(data, indent=2))


def plot(data: list[dict]) -> None:
    try:
        import matplotlib
        matplotlib.use("Agg")
        import matplotlib.pyplot as plt
        import matplotlib.ticker as ticker
    except ImportError:
        print("  (matplotlib not available — skipping plot)")
        return

    cycles   = [d["cycle"] for d in data]
    g_scores = [d.get("graph_score") for d in data]
    e_scores = [d.get("eval_score") for d in data]

    fig, ax1 = plt.subplots(figsize=(10, 5))
    ax2 = ax1.twinx()

    # graph score — solid blue
    g_valid = [(c, s) for c, s in zip(cycles, g_scores) if s is not None]
    if g_valid:
        gx, gy = zip(*g_valid)
        ax1.plot(gx, gy, "o-", color="#2563eb", linewidth=2, markersize=6,
                 label="Graph completion score")
        ax1.set_ylabel("Graph completion score (%)", color="#2563eb")
        ax1.tick_params(axis="y", labelcolor="#2563eb")
        ax1.yaxis.set_major_formatter(ticker.FormatStrFormatter("%.1f%%"))

    # eval score — dashed orange
    e_valid = [(c, s) for c, s in zip(cycles, e_scores) if s is not None]
    if e_valid:
        ex, ey = zip(*e_valid)
        ax2.plot(ex, ey, "s--", color="#ea580c", linewidth=2, markersize=6,
                 label="Eval keyword hit rate")
        ax2.set_ylabel("Eval keyword hit rate (%)", color="#ea580c")
        ax2.tick_params(axis="y", labelcolor="#ea580c")
        ax2.yaxis.set_major_formatter(ticker.FormatStrFormatter("%.1f%%"))

    ax1.set_xlabel("Dream cycle")
    ax1.xaxis.set_major_locator(ticker.MaxNLocator(integer=True))

    # combined legend
    lines1, labels1 = ax1.get_legend_handles_labels()
    lines2, labels2 = ax2.get_legend_handles_labels()
    ax1.legend(lines1 + lines2, labels1 + labels2, loc="lower right")

    ax1.set_title("D6 — Dream RAG progress: graph score vs eval accuracy")
    ax1.grid(True, alpha=0.3)
    fig.tight_layout()
    fig.savefig(str(PLOT_FILE), dpi=150)
    plt.close(fig)
    print(f"  Plot saved → {PLOT_FILE}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--cycles", type=int, default=10)
    parser.add_argument("--kb", default="D6")
    parser.add_argument("--peer", default=PEER)
    parser.add_argument("--model", default=MODEL)
    parser.add_argument("--eval-every", type=int, default=1,
                        help="Run eval every N dream cycles (default 1)")
    args = parser.parse_args()

    data = load_data()

    # Always render current data before running new cycles.
    if data:
        plot(data)

    start_cycle = data[-1]["cycle"] + 1 if data else 1

    for cycle in range(start_cycle, start_cycle + args.cycles):
        print(f"\n{'='*60}")
        print(f"  Cycle {cycle}  [{datetime.now().strftime('%H:%M:%S')}]")
        print(f"{'='*60}")

        # --- dream ---
        print("  Running dream cycle…")
        dream_out = run([
            "kwaainet", "rag", "dream", "run",
            "--kb", args.kb,
            "--inference-url", args.peer,
            "--model", args.model,
        ])
        before, after = parse_dream_score(dream_out)
        print(f"  Graph score: {before}% → {after}%  ({'+' if after and before and after>before else ''}{(after-before):.1f}%)" if (before and after) else "  (no score parsed)")

        entry: dict = {
            "cycle": cycle,
            "ts": datetime.utcnow().isoformat(),
            "graph_score": after,
        }

        # --- eval (every N cycles) ---
        if cycle % args.eval_every == 0 and EVAL_Q.exists():
            print("  Running eval…")
            eval_out = run([
                "kwaainet", "rag", "eval",
                "--questions", str(EVAL_Q),
                "--kb", args.kb,
                "--inference-url", args.peer,
                "--model", args.model,
                "--mode", "iterative",
            ])
            hit = parse_eval_score(eval_out)
            print(f"  Eval hit rate: {hit:.1f}%" if hit else "  (no eval score parsed)")
            entry["eval_score"] = hit
        else:
            entry["eval_score"] = None

        data.append(entry)
        save_data(data)
        plot(data)

    print("\nDone.")


if __name__ == "__main__":
    main()
