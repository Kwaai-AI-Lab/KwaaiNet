#!/usr/bin/env python3
"""Gold step 3: verify each nugget against the corpus itself.

For every nugget, BM25 retrieves candidate passages from the KB's own chunk store (the question
text is added to the query), and the NLI model scores passage ⊨ nugget. The best pair of top
passages is also tried, for facts split across two chunks.

  max entailment ≥ VERIFIED   → status "verified"
  BORDERLINE ≤ max < VERIFIED → status "borderline" (Claude adjudicates, gold/adjudicate.py)
  max < BORDERLINE            → status "unsupported" (also adjudicated: BM25 can miss)

Passages with entailment ≥ GOLD_PASSAGE become the nugget's gold passages, anchored by chunk id
and text hash. Thresholds are provisional until calibrated against author labels.

    .venv/bin/python gold/verify.py [KB ...]      # reads the ORIGINAL KB's chunk store

Reads work/gold/<KB>_nuggets.json, writes work/gold/<KB>_verified.json.
"""
from __future__ import annotations

import itertools
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from common import KBS, WORK  # noqa: E402
from corpus import BM25Index, clean_text  # noqa: E402
from nli.scorer import Scorer  # noqa: E402

VERIFIED = 0.90
GOLD_PASSAGE = 0.70
BORDERLINE = 0.30
N_CANDIDATES = 30
N_PAIR = 5


def verify_kb(kb: str, scorer: Scorer) -> None:
    src = json.loads((WORK / "gold" / f"{kb}_nuggets.json").read_text())
    index = BM25Index(kb)
    out_q = []
    for q in src["questions"]:
        nuggets = []
        for n in q["nuggets"]:
            cands = index.search(f"{n['text']} {q['question']}", N_CANDIDATES)
            scores = scorer.score_pairs([(clean_text(c.text), n["text"]) for c in cands])
            ranked = sorted(zip(cands, scores), key=lambda cs: cs[1].entail, reverse=True)
            best = ranked[0][1].entail if ranked else 0.0
            best_pair = None
            if ranked and best < VERIFIED:
                pairs = list(itertools.combinations([c for c, _ in ranked[:N_PAIR]], 2))
                ps = scorer.score_pairs([(clean_text(a.text) + " " + clean_text(b.text), n["text"]) for a, b in pairs])
                if ps:
                    j = max(range(len(ps)), key=lambda k: ps[k].entail)
                    if ps[j].entail > best:
                        best_pair = (pairs[j], ps[j].entail)
            best_all = max(best, best_pair[1] if best_pair else 0.0)
            status = ("verified" if best_all >= VERIFIED
                      else "borderline" if best_all >= BORDERLINE else "unsupported")
            gold = [{"chunk_id": c.chunk_id, "doc_name": c.doc_name, "chunk_index": c.chunk_index,
                     "text_hash": c.hash, "entail": round(s.entail, 4)}
                    for c, s in ranked if s.entail >= GOLD_PASSAGE]
            if best_pair and best_pair[1] >= GOLD_PASSAGE and not gold:
                gold = [{"chunk_id": c.chunk_id, "doc_name": c.doc_name, "chunk_index": c.chunk_index,
                         "text_hash": c.hash, "entail": round(best_pair[1], 4), "pair": True}
                        for c in best_pair[0]]
            nuggets.append({**n, "status": status, "max_entail": round(best_all, 4),
                            "max_contradict": round(max((s.contradict for _, s in ranked), default=0.0), 4),
                            "gold_passages": gold,
                            "top_candidates": [{"chunk_id": c.chunk_id, "entail": round(s.entail, 4),
                                                "text": clean_text(c.text)} for c, s in ranked[:3]]})
        out_q.append({**q, "nuggets": nuggets})
    dest = WORK / "gold" / f"{kb}_verified.json"
    dest.write_text(json.dumps({"kb": kb, "thresholds": {"verified": VERIFIED, "gold": GOLD_PASSAGE,
                                                          "borderline": BORDERLINE},
                                "questions": out_q}, indent=1, ensure_ascii=False))
    counts: dict[str, int] = {}
    for q in out_q:
        for n in q["nuggets"]:
            counts[n["status"]] = counts.get(n["status"], 0) + 1
    no_core = [q["id"] for q in out_q
               if not any(n["core"] and n["status"] == "verified" for n in q["nuggets"])]
    print(f"{kb:11s} {counts}  questions without a verified core nugget: {len(no_core)} {no_core}")


def main(argv: list[str]) -> None:
    scorer = Scorer()
    for kb in argv or [k for k, v in KBS.items() if v.tracker and v.public]:
        verify_kb(kb, scorer)
    print(f"NLI cache hits {scorer.hits}, misses {scorer.misses}")


if __name__ == "__main__":
    main(sys.argv[1:])
