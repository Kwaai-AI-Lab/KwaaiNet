#!/usr/bin/env python3
"""Generate a markdown comparison report from scores.json and result files."""

import json
import subprocess
from datetime import datetime, timezone
from pathlib import Path

RESULTS_DIR = Path(__file__).parent / "results"


def get_kwaainet_version() -> str:
    try:
        out = subprocess.check_output(["kwaainet", "--version"], stderr=subprocess.DEVNULL, text=True)
        return out.strip()
    except Exception:
        return "unknown"


def fmt(val, fmt_str=".3f", fallback="n/a") -> str:
    if val is None:
        return fallback
    if isinstance(val, float):
        return format(val, fmt_str)
    if isinstance(val, int):
        return f"{val:,}"
    return str(val)


def main():
    scores_file = RESULTS_DIR / "scores.json"
    if not scores_file.exists():
        print("No scores.json found. Run score.py first.")
        return

    scores = json.loads(scores_file.read_text())
    kw = scores.get("kwaainet", {})
    gm = scores.get("gemini", {})

    kw_results = []
    gm_results = []
    kw_file = RESULTS_DIR / "kwaainet_results.json"
    gm_file = RESULTS_DIR / "gemini_results.json"
    if kw_file.exists():
        kw_results = json.loads(kw_file.read_text())
    if gm_file.exists():
        gm_results = json.loads(gm_file.read_text())

    version = get_kwaainet_version()
    date = datetime.now(timezone.utc).strftime("%Y-%m-%d")

    lines = [
        f"# KwaaiNet RAG vs Gemini — {date}",
        "",
        f"**Corpus**: PreRAG (31 DOCX chapters)  ",
        f"**KwaaiNet version**: {version}  ",
        f"**Gemini model**: {scores.get('_gemini_model', 'gemini-1.5-pro')}  ",
        "",
        "## Summary",
        "",
        "| Metric | KwaaiNet | Gemini |",
        "|--------|----------|--------|",
        f"| LLM Judge score (0–3 avg) | {fmt(kw.get('judge_score'))} | {fmt(gm.get('judge_score'))} |",
        f"| Golden set F1 | {fmt(kw.get('golden_f1'))} | {fmt(gm.get('golden_f1'))} |",
        f"| Hit@5 (retrieval) | {fmt(kw.get('hit_at_5'))} | n/a |",
        f"| MRR (retrieval) | {fmt(kw.get('mrr'))} | n/a |",
        f"| Avg retrieval latency (ms) | {fmt(kw.get('avg_retrieval_ms'), '.0f')} | n/a |",
        f"| Avg generation latency (ms) | {fmt(kw.get('avg_generation_ms'), '.0f')} | {fmt(gm.get('avg_total_ms'), '.0f')} |",
        f"| Avg total latency (ms) | {fmt(kw.get('avg_total_ms'), '.0f')} | {fmt(gm.get('avg_total_ms'), '.0f')} |",
        "",
    ]

    # Per-category breakdown
    categories = ["factoid", "inferential", "out_of_scope"]
    for cat in categories:
        kw_cat = [r for r in kw_results if r.get("category") == cat]
        gm_cat = [r for r in gm_results if r.get("category") == cat]

        def cat_judge(results, key):
            vals = [r[key]["avg"] for r in results if r.get(key) and r[key]]
            return round(sum(vals) / len(vals), 3) if vals else None

        lines += [
            f"### {cat.replace('_', ' ').title()} questions ({len(kw_cat)} questions)",
            "",
            f"| | KwaaiNet | Gemini |",
            f"|--|----------|--------|",
            f"| Judge score | {fmt(cat_judge(kw_cat, 'judge_kwaainet'))} | {fmt(cat_judge(gm_cat, 'judge_gemini'))} |",
            "",
        ]

    # Per-question breakdown
    lines += [
        "## Per-question breakdown",
        "",
        "| ID | Category | Question | KwaaiNet answer | Gemini answer | KW judge | GM judge |",
        "|----|----------|----------|-----------------|---------------|----------|----------|",
    ]

    # Index gemini results by id
    gm_by_id = {r["id"]: r for r in gm_results}

    for r in kw_results:
        qid = r["id"]
        gm_r = gm_by_id.get(qid, {})
        kw_ans = r.get("answer", "")[:120].replace("|", "\\|").replace("\n", " ")
        gm_ans = gm_r.get("answer", "")[:120].replace("|", "\\|").replace("\n", " ")
        kw_j = r.get("judge_kwaainet")
        gm_j = gm_r.get("judge_gemini")
        kw_score = fmt(kw_j["avg"] if kw_j else None)
        gm_score = fmt(gm_j["avg"] if gm_j else None)
        q_short = r["question"][:60].replace("|", "\\|")
        lines.append(f"| {qid} | {r['category']} | {q_short} | {kw_ans} | {gm_ans} | {kw_score} | {gm_score} |")

    report = "\n".join(lines) + "\n"
    out_file = RESULTS_DIR / "report.md"
    out_file.write_text(report)
    print(f"Report written to {out_file}")

    # Print summary to terminal
    print("\n" + "=" * 60)
    print(f"  KwaaiNet judge score : {fmt(kw.get('judge_score'))}")
    print(f"  Gemini   judge score : {fmt(gm.get('judge_score'))}")
    print(f"  KwaaiNet Hit@5       : {fmt(kw.get('hit_at_5'))}")
    print(f"  KwaaiNet MRR         : {fmt(kw.get('mrr'))}")
    print("=" * 60)


if __name__ == "__main__":
    main()
