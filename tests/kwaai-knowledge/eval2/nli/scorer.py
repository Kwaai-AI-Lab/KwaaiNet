"""Deterministic NLI entailment scorer (no LLM judge).

Used for: nugget verification against source passages (gold), context nugget coverage
(passage ⊨ nugget), answer nugget recall (answer ⊨ nugget), and faithfulness
(context ⊨ answer sentence).

Long premises are split into overlapping token windows and the maximum entailment over
windows is taken, because the MNLI-class model was trained on sentence-length premises.
Every (premise, hypothesis) pair is cached by content hash in SQLite, so re-scoring the
same passages across snapshots is free.
"""
from __future__ import annotations

import re
import sqlite3
import sys
from dataclasses import dataclass
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from common import WORK, text_hash  # noqa: E402

MODEL_ID = "MoritzLaurer/DeBERTa-v3-large-mnli-fever-anli-ling-wanli"
WINDOW_TOKENS = 350
STRIDE_TOKENS = 128
MAX_LEN = 512


@dataclass(frozen=True)
class NLI:
    entail: float
    contradict: float


_SENT_RE = re.compile(r"(?<=[.!?])\s+(?=[A-Z0-9\"'(])")


def sentences(text: str) -> list[str]:
    """Split into claim-sized sentences; drops fragments too short to be a claim."""
    parts = [p.strip() for p in _SENT_RE.split(" ".join(text.split())) if p.strip()]
    return [p for p in parts if len(p.split()) >= 4]


class Scorer:
    def __init__(self, cache_path: Path | None = None, device: str | None = None, batch: int = 16):
        import torch
        from transformers import AutoModelForSequenceClassification, AutoTokenizer

        self.torch = torch
        self.device = device or ("mps" if torch.backends.mps.is_available() else "cpu")
        self.tok = AutoTokenizer.from_pretrained(MODEL_ID)
        self.model = AutoModelForSequenceClassification.from_pretrained(MODEL_ID).to(self.device).eval()
        labels = {v.lower(): int(k) for k, v in self.model.config.id2label.items()}
        self.i_ent, self.i_con = labels["entailment"], labels["contradiction"]
        self.batch = batch
        cache_path = cache_path or (WORK / "cache" / "nli.sqlite")
        cache_path.parent.mkdir(parents=True, exist_ok=True)
        self.db = sqlite3.connect(cache_path)
        self.db.execute("CREATE TABLE IF NOT EXISTS nli (p TEXT, h TEXT, model TEXT, ent REAL, con REAL,"
                        " PRIMARY KEY (p, h, model))")
        self.hits = self.misses = 0

    # -- windows --------------------------------------------------------------------------
    def windows(self, premise: str) -> list[str]:
        ids = self.tok(premise, add_special_tokens=False)["input_ids"]
        if len(ids) <= WINDOW_TOKENS:
            return [premise]
        out, start = [], 0
        while start < len(ids):
            out.append(self.tok.decode(ids[start:start + WINDOW_TOKENS]))
            if start + WINDOW_TOKENS >= len(ids):
                break
            start += WINDOW_TOKENS - STRIDE_TOKENS
        return out

    # -- raw scoring ----------------------------------------------------------------------
    def _run(self, pairs: list[tuple[str, str]]) -> list[NLI]:
        out: list[NLI] = []
        torch = self.torch
        for i in range(0, len(pairs), self.batch):
            chunk = pairs[i:i + self.batch]
            enc = self.tok([p for p, _ in chunk], [h for _, h in chunk], truncation="only_first",
                           max_length=MAX_LEN, padding=True, return_tensors="pt").to(self.device)
            with torch.no_grad():
                probs = torch.softmax(self.model(**enc).logits.float(), dim=-1).cpu().tolist()
            out.extend(NLI(p[self.i_ent], p[self.i_con]) for p in probs)
        return out

    def score_pairs(self, pairs: list[tuple[str, str]]) -> list[NLI]:
        """Entailment of each hypothesis by its premise, max over premise windows (cached)."""
        results: list[NLI | None] = [None] * len(pairs)
        todo: list[tuple[int, str, str]] = []
        for i, (p, h) in enumerate(pairs):
            row = self.db.execute("SELECT ent, con FROM nli WHERE p=? AND h=? AND model=?",
                                  (text_hash(p), text_hash(h), MODEL_ID)).fetchone()
            if row:
                results[i] = NLI(*row)
                self.hits += 1
            else:
                todo.append((i, p, h))
                self.misses += 1
        if todo:
            flat, owner = [], []
            for i, p, h in todo:
                for w in self.windows(p):
                    flat.append((w, h))
                    owner.append(i)
            scored = self._run(flat)
            best: dict[int, NLI] = {}
            for i, s in zip(owner, scored):
                b = best.get(i)
                best[i] = NLI(max(s.entail, b.entail) if b else s.entail,
                              max(s.contradict, b.contradict) if b else s.contradict)
            for i, p, h in todo:
                results[i] = best[i]
                self.db.execute("INSERT OR REPLACE INTO nli VALUES (?,?,?,?,?)",
                                (text_hash(p), text_hash(h), MODEL_ID, best[i].entail, best[i].contradict))
            self.db.commit()
        return results  # type: ignore[return-value]

    # -- derived measures -----------------------------------------------------------------
    def best_support(self, premises: list[str], hypothesis: str) -> tuple[float, int]:
        """Max entailment of `hypothesis` over `premises`, and the index of the best premise."""
        if not premises:
            return 0.0, -1
        s = self.score_pairs([(p, hypothesis) for p in premises])
        i = max(range(len(s)), key=lambda k: s[k].entail)
        return s[i].entail, i
