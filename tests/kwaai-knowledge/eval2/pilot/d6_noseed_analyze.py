#!/usr/bin/env python3
"""Analysis and pre-registered verdict for the D6 no-seed trial (pilot/d6_noseed.py).

Verdict (projects/kwaai-knowledge/plans/D6-NoSeed-Dream-plan.md): PASS if arm B's paired
per-question change in coverage from cycle 0 to cycle 24 has a 95% bootstrap CI above 0 and a
mean larger than the cycle-0 retest spread. Secondary: arm B minus arm A at cycle 24, the Spearman
correlation of graph score with coverage across checkpoints, answer recall and keyword recall.

    .venv/bin/python pilot/d6_noseed_analyze.py        # prints the report, writes report.json / report.md
    .venv/bin/python pilot/d6_noseed_analyze.py --mode graph-only   # -> report_graph-only.{json,md}

`gold_in_prompt` is a retrieval measure without NLI: the share of a question's reachable nuggets
whose gold passage is among the prompt's chunks. It exists because coverage has a ceiling here:
many gold passages were accepted by Claude adjudication at NLI 0.3-0.9, so NLI scores them below
coverage's 0.5 threshold even when they are in the prompt.

Reads results/d6_noseed_s10/metrics.jsonl (NLI metrics). Keyword recall is read from the dumps, so
the script also runs, keyword recall only, before `metrics` has scored anything.
"""
from __future__ import annotations

import json
import random
import statistics as st
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from common import RESULTS, WORK, read_jsonl  # noqa: E402

OUT = RESULTS.parent / "d6_noseed_s10"
CYCLES = (0, 1, 2, 4, 8, 12, 16, 20, 24)
MODE = sys.argv[sys.argv.index("--mode") + 1] if "--mode" in sys.argv else "iterative"
# No chunk reaches a graph-only prompt, so gold_in_prompt is 0 by construction there.
MEASURES = (("coverage", "answer_recall", "keyword_recall") if MODE == "graph-only"
            else ("coverage", "gold_in_prompt", "answer_recall", "keyword_recall"))
BOOT = 5000


def parse_tag(tag: str) -> tuple[str, int, int, str]:
    _, arm, c, rep, mode = tag.split(":")
    return arm, int(c[1:]), int(rep[1:]), mode


def load_rows() -> dict[tuple, dict[str, dict]]:
    """(arm, cycle, rep, mode) -> qid -> measures, for complete runs only.

    A dump with fewer records than questions is an eval still running (or one that died): its
    mean would cover a different set of questions, so it is left out."""
    expected = len(json.loads((OUT / "D6_s10_questions.json").read_text()))
    gold = json.loads((WORK / "gold" / "D6_s10_gold.json").read_text())
    cutoff = gold["slice"]["cutoff"]
    reach = {q["id"]: [n for n in q["nuggets"] if n.get("status") in ("verified", "supported")]
             for q in gold["questions"]}
    rows: dict[tuple, dict[str, dict]] = {}
    for dump in sorted((OUT / "dumps").glob("*.jsonl")):
        recs = read_jsonl(dump)
        if len(recs) != expected:
            print(f"skipping incomplete {dump.name}: {len(recs)}/{expected} records", file=sys.stderr)
            continue
        for r in recs:
            key = parse_tag(r["run_tag"])
            kw = r["keyword_hits"] / r["total_keywords"] if r.get("total_keywords") else None
            # Retrieval without NLI: share of the question's reachable nuggets whose gold passage
            # (a chunk in the slice) is among the prompt's chunks.
            in_prompt = {c["chunk_index"] for c in r["retrieved"] if c["in_prompt"] and not c.get("synthetic")}
            ns = [n for n in reach.get(r["qid"], [])
                  if any(g["chunk_index"] < cutoff for g in n.get("gold_passages", []))]
            gip = (sum(any(g["chunk_index"] in in_prompt for g in n["gold_passages"]) for n in ns) / len(ns)
                   if ns else None)
            rows.setdefault(key, {})[r["qid"]] = {"keyword_recall": kw, "gold_in_prompt": gip}
    metrics = OUT / "metrics.jsonl"
    if metrics.exists():
        for r in read_jsonl(metrics):
            key = parse_tag(r["run_tag"])
            if key not in rows:
                continue  # an incomplete dump
            rows.setdefault(key, {}).setdefault(r["qid"], {}).update(
                {m: r.get(m) for m in ("coverage", "answer_recall")})
    return rows


def values(rows, key, measure) -> dict[str, float]:
    return {q: v[measure] for q, v in rows.get(key, {}).items() if v.get(measure) is not None}


def paired_delta(a: dict[str, float], b: dict[str, float]) -> dict | None:
    qs = sorted(set(a) & set(b))
    if len(qs) < 2:
        return None
    d = [b[q] - a[q] for q in qs]
    rng = random.Random(20260929)
    boots = sorted(st.mean(rng.choices(d, k=len(d))) for _ in range(BOOT))
    return {"n": len(qs), "mean": round(st.mean(d), 4),
            "ci95": [round(boots[int(0.025 * BOOT)], 4), round(boots[int(0.975 * BOOT)], 4)]}


def average_ranks(v: list[float]) -> list[float]:
    """Rank each value, tied values sharing the mean of the ranks they span (as scipy does).

    Ties are common here: a saturated arm leaves the graph score unchanged across cycles.
    """
    order = sorted(range(len(v)), key=lambda i: v[i])
    ranks = [0.0] * len(v)
    i = 0
    while i < len(order):
        j = i
        while j + 1 < len(order) and v[order[j + 1]] == v[order[i]]:
            j += 1
        for k in range(i, j + 1):
            ranks[order[k]] = (i + j) / 2
        i = j + 1
    return ranks


def spearman(xs: list[float], ys: list[float]) -> float | None:
    if len(xs) < 3:
        return None
    a, b = average_ranks(xs), average_ranks(ys)
    ma, mb = st.mean(a), st.mean(b)
    num = sum((p - ma) * (q - mb) for p, q in zip(a, b))
    den = (sum((p - ma) ** 2 for p in a) * sum((q - mb) ** 2 for q in b)) ** 0.5
    return round(num / den, 3) if den else None


def graph_scores() -> dict[tuple[str, int], dict]:
    state = json.loads((OUT / "state.json").read_text())["steps"]
    out = {("0", 0): state["build"]["result"]["score"]}
    for k, v in state.items():
        if k.startswith("dream:") and v.get("status") == "done":
            _, arm, c = k.split(":")
            out[(arm, int(c[1:]))] = v["result"]["score"]
    return out


def main() -> None:
    rows = load_rows()
    scores = graph_scores()
    rep: dict = {"measures": {}, "graph": {f"{a}:c{c:02d}": s for (a, c), s in sorted(scores.items())}}
    for m in MEASURES:
        base = values(rows, ("0", 0, 1, MODE), m)
        if not base:
            continue
        retest = [st.mean(v.values()) for r in (1, 2, 3) if (v := values(rows, ("0", 0, r, MODE), m))]
        mr: dict = {"cycle0_mean": round(st.mean(base.values()), 4),
                    "cycle0_retest_means": [round(x, 4) for x in retest],
                    "cycle0_retest_spread": round(max(retest) - min(retest), 4) if len(retest) > 1 else None,
                    "vector_c0": round(st.mean(v.values()), 4)
                    if (v := values(rows, ("0", 0, 1, "vector"), m)) else None,
                    "arms": {}}
        for arm in "AB":
            traj = []
            for c in CYCLES:
                v = base if c == 0 else values(rows, (arm, c, 1, MODE), m)
                if v:
                    traj.append({"cycle": c, "mean": round(st.mean(v.values()), 4),
                                 "delta_vs_c0": paired_delta(base, v) if c else None,
                                 "graph_score": scores.get((arm if c else "0", c), {}).get("overall")})
            end = [st.mean(v.values()) for r in (1, 2, 3) if (v := values(rows, (arm, 24, r, MODE), m))]
            pts = [(t["graph_score"], t["mean"]) for t in traj if t["graph_score"] is not None]
            mr["arms"][arm] = {"trajectory": traj,
                               "c24_repeat_means": [round(x, 4) for x in end],
                               "spearman_graph_score_vs_measure": spearman(*zip(*pts)) if len(pts) >= 3 else None}
        a24, b24 = values(rows, ("A", 24, 1, MODE), m), values(rows, ("B", 24, 1, MODE), m)
        mr["B_minus_A_c24"] = paired_delta(a24, b24) if a24 and b24 else None
        rep["measures"][m] = mr

    cov = rep["measures"].get("coverage")
    if cov and cov["arms"].get("B", {}).get("trajectory") and cov["arms"]["B"]["trajectory"][-1]["cycle"] == 24:
        d = cov["arms"]["B"]["trajectory"][-1]["delta_vs_c0"]
        spread = cov["cycle0_retest_spread"] or 0
        ok = d["ci95"][0] > 0 and d["mean"] > spread
        rep["verdict"] = {"pass": ok, "delta": d, "retest_spread": spread,
                          "rule": "arm B Δcoverage c0→c24: 95% CI above 0 and mean > cycle-0 retest spread"}
    else:
        rep["verdict"] = {"pass": None, "note": "coverage for arm B at cycle 24 not scored yet"}

    stem = "report" if MODE == "iterative" else f"report_{MODE}"
    rep["mode"] = MODE
    (OUT / f"{stem}.json").write_text(json.dumps(rep, indent=1))
    lines = [f"# D6 no-seed trial: recall vs dream cycles (first 10%, retrieval: {MODE})", ""]
    for m, mr in rep["measures"].items():
        lines += [f"## {m}", "", f"cycle 0: {mr['cycle0_mean']} (retest {mr['cycle0_retest_means']}, "
                  f"spread {mr['cycle0_retest_spread']}); vector-only {mr['vector_c0']}", "",
                  "| arm | cycle | mean | Δ vs c0 (95% CI) | graph score |", "|---|---|---|---|---|"]
        for arm, ar in mr["arms"].items():
            for t in ar["trajectory"]:
                d = t["delta_vs_c0"]
                ds = f"{d['mean']:+.3f} ({d['ci95'][0]:+.3f}, {d['ci95'][1]:+.3f})" if d else "—"
                lines.append(f"| {arm} | {t['cycle']} | {t['mean']:.3f} | {ds} | {t['graph_score']} |")
        lines += ["", f"B − A at c24: {mr['B_minus_A_c24']}", ""]
    lines += ["## Verdict", "", json.dumps(rep["verdict"]), ""]
    (OUT / f"{stem}.md").write_text("\n".join(lines))
    print("\n".join(lines))


if __name__ == "__main__":
    main()
