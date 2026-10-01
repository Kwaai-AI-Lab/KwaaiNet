"""Regression tests for eval2/metrics.py and the D6 analysis helpers.

Run with the eval2 venv: .venv/bin/python tests/kwaai-knowledge/eval2/test_metrics.py
"""
from __future__ import annotations

import sys
from pathlib import Path
from types import SimpleNamespace

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(HERE / "pilot"))
from d6_noseed_analyze import average_ranks, spearman  # noqa: E402
from metrics import question_metrics  # noqa: E402


class NoEntailment:
    def score_pairs(self, pairs):
        return [SimpleNamespace(entail=0.0, contradict=0.0) for _ in pairs]


def _rec(n_retrieved: int) -> dict:
    return {"run_tag": "t", "kb": "Legal", "qid": "q1", "answer": "", "graph_entities": 0,
            "graph_relations": 0, "keyword_hits": 0, "total_keywords": 0,
            "retrieved": [{"chunk_id": i, "text_hash": f"h{i}", "in_prompt": False,
                           "prompt_slot": None, "context_text": ""} for i in range(n_retrieved)]}


def _nugget(gold_chunk: int) -> dict:
    return {"text": "a fact", "gold_passages": [{"chunk_id": gold_chunk, "text_hash": f"h{gold_chunk}"}]}


def test_gold_passage_recall20_counts_only_the_top_20() -> None:
    # Regression (review of #239): the hit set was built from every retrieved chunk, so a
    # run with -k above 20 credited gold passages ranked 21st or lower.
    row = question_metrics(_rec(25), [_nugget(21)], NoEntailment(), None)
    assert row["gold_passage_recall20"] == 0.0, row["gold_passage_recall20"]
    row = question_metrics(_rec(25), [_nugget(2)], NoEntailment(), None)
    assert row["gold_passage_recall20"] == 1.0, row["gold_passage_recall20"]


def test_average_ranks_share_ties() -> None:
    assert average_ranks([50, 60, 60, 60, 70]) == [0.0, 2.0, 2.0, 2.0, 4.0]


def test_spearman_matches_scipy_with_ties() -> None:
    # Regression (review of #239): ties took the last rank of their run, not the mean.
    # Expected values are scipy.stats.spearmanr's.
    assert spearman([1, 2, 3, 4, 5], [50, 60, 60, 60, 70]) == 0.894
    assert spearman([1, 2, 3, 4, 5, 6], [3, 3, 1, 2, 2, 5]) == 0.088
    assert spearman([1, 2, 3, 4], [4, 3, 2, 1]) == -1.0


if __name__ == "__main__":
    for name, fn in list(globals().items()):
        if name.startswith("test_") and callable(fn):
            fn()
            print(f"ok  {name}")
