#!/usr/bin/env python3
"""Gold step 1: import NotebookLM answers from the QA-Tracker XLSX files.

The per-KB `expected_answer` fields came from the OpenWebUI (Mistral-7B) column and contain
errors. The NotebookLM column is the better reference. Each tracker row is matched to its
question in eval_questions.json by fuzzy text match; weak matches are flagged, never guessed.

    .venv/bin/python gold/xlsx_import.py [KB ...]

Writes work/gold/<KB>_notebooklm.json.
"""
from __future__ import annotations

import difflib
import json
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import openpyxl  # noqa: E402

from common import KBS, QA_TRACKERS, WORK, load_questions  # noqa: E402

MATCH_OK = 0.90  # below this the pairing is flagged for review


def strip_citations(text: str) -> str:
    text = re.sub(r"^\s*Q\d+\s*[:.]\s*", "", text)  # leading "Q1:" row label
    text = re.sub(r"\s*\[\d+(?:\s*[,–-]\s*\d+)*\]", "", text)  # [1], [1, 2], [3-5]
    return re.sub(r"[ \t]+", " ", text).strip()


def norm(q: str) -> str:
    return re.sub(r"\W+", " ", q.lower()).strip()


def import_kb(kb: str) -> dict:
    spec = KBS[kb]
    if spec.tracker is None:
        raise SystemExit(f"{kb} has no QA-Tracker")
    ws = openpyxl.load_workbook(QA_TRACKERS / spec.tracker, read_only=True).active
    rows = [r for r in ws.iter_rows(values_only=True)][1:]
    rows = [(str(r[0]).strip(), str(r[1] or "").strip()) for r in rows if r and r[0]]
    questions = load_questions(kb)

    out, used = [], set()
    for q in questions:
        scored = sorted(
            ((difflib.SequenceMatcher(None, norm(q["question"]), norm(tq)).ratio(), i)
             for i, (tq, _) in enumerate(rows)),
            reverse=True,
        )
        ratio, i = scored[0]
        tq, ans = rows[i]
        flags = []
        if ratio < MATCH_OK:
            flags.append(f"weak question match {ratio:.2f}")
        if i in used:
            flags.append("tracker row matched twice")
        if not ans:
            flags.append("empty NotebookLM answer")
        used.add(i)
        out.append({
            "id": q["id"],
            "question": q["question"],
            "tracker_question": tq,
            "match_ratio": round(ratio, 3),
            "notebooklm_answer": strip_citations(ans),
            "notebooklm_answer_raw": ans,
            "old_expected_answer": q.get("expected_answer"),
            "flags": flags,
        })
    unmatched = [rows[i][0] for i in range(len(rows)) if i not in used]
    return {"kb": kb, "source": str(QA_TRACKERS / spec.tracker), "questions": out,
            "unmatched_tracker_rows": unmatched}


def main(argv: list[str]) -> None:
    kbs = argv or [k for k, v in KBS.items() if v.tracker]
    dest = WORK / "gold"
    dest.mkdir(parents=True, exist_ok=True)
    for kb in kbs:
        doc = import_kb(kb)
        (dest / f"{kb}_notebooklm.json").write_text(json.dumps(doc, indent=1, ensure_ascii=False))
        flagged = [q["id"] for q in doc["questions"] if q["flags"]]
        print(f"{kb:11s} {len(doc['questions'])} questions, flagged {len(flagged)} {flagged}, "
              f"unmatched tracker rows {len(doc['unmatched_tracker_rows'])}")


if __name__ == "__main__":
    main(sys.argv[1:])
