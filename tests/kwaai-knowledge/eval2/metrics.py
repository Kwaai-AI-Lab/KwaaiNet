#!/usr/bin/env python3
"""Per-question metrics for eval2 runs, computed offline from `--dump-jsonl` records.

    .venv/bin/python metrics.py results/eval2/dumps/*.jsonl      # -> results/eval2/metrics.jsonl

Endpoints (definitions fixed in prereg.md):
  coverage          reachable nuggets entailed by ≥1 in-prompt passage      (primary, NLI)
  answer_recall     reachable nuggets entailed by the generated answer      (NLI)
  contradiction     reachable nuggets the answer contradicts                 (NLI)
  faithfulness      answer claim sentences entailed by the passages they cite
                    (all in-prompt passages when a sentence cites none)     (NLI)
  p5/p20, p5_strict/p20_strict, dcg20, gold_passage_recall20                (Claude grades, gold passages)
nDCG needs the ideal ranking over all runs of a question, so metrics.py stores DCG and the grades;
analysis/analyze.py normalises.

Premises are what the generator saw (`context_text`, markup-cleaned); hypotheses are nuggets or
answer sentences. All NLI pairs are cached, so re-scoring is cheap.
"""
from __future__ import annotations

import json
import math
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from common import RESULTS, WORK, read_jsonl  # noqa: E402
from corpus import clean_text  # noqa: E402

THETA = 0.5
REACHABLE = {"verified", "supported"}  # nugget statuses that count (after adjudication/review)
_CITE = re.compile(r"\[(\d+(?:\s*[,–-]\s*\d+)*)\]")
_ABSTAIN = re.compile(r"(?i)(do(es)? not contain|not (mentioned|provided|available|found)|no information)")


def base_kb(kb: str) -> str:
    """The KB whose gold scores `kb`'s records: an Eval v2 clone scores against its source;
    a slice clone (D6_s10, D6_s10A/B/E; pilot/d6_noseed.py) against the slice gold, D6_s10."""
    kb = re.sub(r"(_s\d+)[ABE]$", r"\1", kb)
    return re.sub(r"_e2[ABE]?$", "", kb)


def load_gold(kb: str) -> dict[str, list[dict]]:
    """qid -> reachable nuggets (frozen gold if present, else the verified draft)."""
    for name in (f"{kb}_gold.json", f"{kb}_verified.json"):
        p = WORK / "gold" / name
        if p.exists():
            qs = json.loads(p.read_text())["questions"]
            return {q["id"]: [n for n in q["nuggets"] if n.get("status") in REACHABLE] for q in qs}
    raise FileNotFoundError(f"no verified gold for {kb}")


def cited_slots(sentence: str) -> set[int]:
    out: set[int] = set()
    for grp in _CITE.findall(sentence):
        for part in re.split(r"\s*,\s*", grp):
            if re.match(r"^\d+\s*[–-]\s*\d+$", part):
                a, b = map(int, re.split(r"\s*[–-]\s*", part))
                out.update(range(a, b + 1))
            elif part.strip().isdigit():
                out.add(int(part))
    return out


def question_metrics(rec: dict, nuggets: list[dict], scorer, grades) -> dict:
    from nli.scorer import sentences

    prompt = [c for c in rec["retrieved"] if c["in_prompt"]]
    premises = [clean_text(c["context_text"]) for c in prompt]
    by_slot = {c["prompt_slot"]: clean_text(c["context_text"]) for c in prompt}
    answer = rec["answer"]
    answer_clean = _CITE.sub("", answer)
    row: dict = {"run_tag": rec["run_tag"], "kb": base_kb(rec["kb"]), "qid": rec["qid"],
                 "n_nuggets": len(nuggets), "n_in_prompt": len(prompt),
                 "graph_entities": rec["graph_entities"], "graph_relations": rec["graph_relations"],
                 "keyword_recall": rec["keyword_hits"] / rec["total_keywords"] if rec["total_keywords"] else None,
                 "retrieved_hashes": [c["text_hash"] for c in rec["retrieved"]],
                 "retrieved_ids": [c["chunk_id"] for c in rec["retrieved"] if c["chunk_id"] is not None]}

    if nuggets:
        cov, rec_hits, contra = 0, 0, 0
        pairs = [(p, n["text"]) for n in nuggets for p in premises]
        scores = scorer.score_pairs(pairs) if pairs else []
        k = len(premises)
        for i, n in enumerate(nuggets):
            s = scores[i * k:(i + 1) * k]
            if s and max(x.entail for x in s) >= THETA:
                cov += 1
        ans = scorer.score_pairs([(answer_clean, n["text"]) for n in nuggets])
        rec_hits = sum(a.entail >= THETA for a in ans)
        contra = sum(a.contradict >= THETA for a in ans)
        row.update(coverage=cov / len(nuggets), answer_recall=rec_hits / len(nuggets),
                   contradiction=contra / len(nuggets))
        # Gold-passage recall@20: nuggets with at least one gold passage among the retrieved chunks.
        got_ids = set(row["retrieved_ids"]) | set(row["retrieved_hashes"])
        # Only NLI-verified gold passages (entailment ≥ 0.7 in gold/verify.py) anchor this metric:
        # Claude's adjudication cites passages as a set, which can include non-supporting chunks.
        def nli_gold(n: dict) -> list[dict]:
            return [g for g in n.get("gold_passages") or [] if not g.get("adjudicated") and not g.get("pair")]

        with_gold = [n for n in nuggets if nli_gold(n)]
        if with_gold:
            hit = sum(any(g["chunk_id"] in got_ids or g["text_hash"] in got_ids for g in nli_gold(n))
                      for n in with_gold)
            row["gold_passage_recall20"] = hit / len(with_gold)
    # Faithfulness over claim sentences (abstentions excluded and counted).
    sents = sentences(answer)
    claims = [s for s in sents if not _ABSTAIN.search(s)]
    row["abstained"] = bool(sents) and not claims
    if claims and premises:
        supported = 0
        for s in claims:
            slots = [x for x in cited_slots(s) if x in by_slot]
            prem = [by_slot[x] for x in slots] if slots else premises
            best = max(x.entail for x in scorer.score_pairs([(p, _CITE.sub("", s)) for p in prem]))
            supported += best >= THETA
        row["faithfulness"] = supported / len(claims)
        row["n_claims"] = len(claims)
    # Claude relevance grades, by retrieval rank.
    if grades is not None:
        g = [grades(row["kb"], rec["qid"], c["text_hash"]) for c in rec["retrieved"][:20]]
        vals = [x["grade"] if x else None for x in g]
        row["grades"] = vals
        if all(v is not None for v in vals) and vals:
            row["p5"] = sum(v >= 1 for v in vals[:5]) / min(5, len(vals))
            row["p20"] = sum(v >= 1 for v in vals) / len(vals)
            row["p5_strict"] = sum(v == 2 for v in vals[:5]) / min(5, len(vals))
            row["p20_strict"] = sum(v == 2 for v in vals) / len(vals)
            row["dcg20"] = sum((2 ** v - 1) / math.log2(i + 2) for i, v in enumerate(vals))
    return row


def main(argv: list[str]) -> None:
    from judge.relevance import db as judge_db, grades_for
    from nli.scorer import Scorer

    scorer = Scorer()
    jcon = judge_db()
    import os

    out = Path(os.environ.get("EVAL2_METRICS_OUT", RESULTS / "metrics.jsonl"))
    done = set()
    if out.exists():
        for r in read_jsonl(out):
            done.add((r["run_tag"], r["qid"]))
    gold_cache: dict[str, dict] = {}
    with out.open("a") as f:
        for path in argv:
            for rec in read_jsonl(Path(path)):
                if (rec["run_tag"], rec["qid"]) in done:
                    continue
                kb = base_kb(rec["kb"])
                gold = gold_cache.setdefault(kb, load_gold(kb))
                row = question_metrics(rec, gold.get(rec["qid"], []), scorer,
                                       lambda k, q, h: grades_for(k, q, h, jcon))
                f.write(json.dumps(row) + "\n")
                f.flush()
    print(f"NLI cache hits {scorer.hits}, misses {scorer.misses}")


if __name__ == "__main__":
    main(sys.argv[1:])
