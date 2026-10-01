#!/usr/bin/env python3
"""Gold step 2: split each NotebookLM reference answer into atomic nuggets (Claude).

A nugget is one self-contained, checkable fact that helps answer the question. Nuggets are
what the NLI scorer later tests for entailment — by source passages (verification), by the
retrieved context (coverage) and by the generated answer (recall). They must therefore be
standalone sentences: every entity named in full, no pronouns pointing outside the nugget.

    .venv/bin/python gold/nuggetize.py [KB ...]

Reads work/gold/<KB>_notebooklm.json, writes work/gold/<KB>_nuggets.json. Resumable: questions
already nuggetized are skipped.
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from claude import call_json  # noqa: E402
from common import KBS, WORK  # noqa: E402

PROMPT_VERSION = "nuggetize-v1"

SYSTEM = """You turn a reference answer into atomic "nuggets" for evaluating a question-answering system.

A nugget is ONE checkable fact from the reference answer that helps answer the question.

Rules:
- Each nugget is a single declarative sentence that stands alone: name every person, place,
  document or thing in full (no "he", "it", "the letter", "this study" that points outside the nugget).
- One fact per nugget. Split conjunctions of independent facts into separate nuggets.
- Use only facts stated in the reference answer. Do not add, correct or infer facts.
- Drop hedges, meta-commentary ("the sources say", "according to the document"), and restatements
  of the question.
- Mark a nugget core=true if a correct answer to the question must contain it; core=false if it is
  supporting detail. Every question should have at least one core nugget.
- At most 8 nuggets; keep the most important ones if the answer has more.
- If the reference answer says the information is not available, return an empty list and set
  answerable=false."""

SCHEMA = {
    "type": "object",
    "properties": {
        "answerable": {"type": "boolean"},
        "nuggets": {
            "type": "array",
            "items": {
                "type": "object",
                "properties": {"text": {"type": "string"}, "core": {"type": "boolean"}},
                "required": ["text", "core"],
                "additionalProperties": False,
            },
        },
    },
    "required": ["answerable", "nuggets"],
    "additionalProperties": False,
}


def nuggetize_kb(kb: str) -> None:
    src = json.loads((WORK / "gold" / f"{kb}_notebooklm.json").read_text())
    dest = WORK / "gold" / f"{kb}_nuggets.json"
    done = {q["id"]: q for q in json.loads(dest.read_text())["questions"]} if dest.exists() else {}
    out = []
    for q in src["questions"]:
        if q["id"] in done and done[q["id"]].get("prompt_version") == PROMPT_VERSION:
            out.append(done[q["id"]])
            continue
        user = f"Question:\n{q['question']}\n\nReference answer:\n{q['notebooklm_answer']}"
        r = call_json(kb=kb, kind="nuggetize", system=SYSTEM, user=user, schema=SCHEMA, effort="medium")
        nuggets = [{"id": f"{q['id']}.n{i + 1}", "text": n["text"].strip(), "core": n["core"]}
                   for i, n in enumerate(r["nuggets"][:8])]
        out.append({"id": q["id"], "question": q["question"], "gold_answer": q["notebooklm_answer"],
                    "answerable": r["answerable"], "nuggets": nuggets, "prompt_version": PROMPT_VERSION})
        dest.write_text(json.dumps({"kb": kb, "questions": out}, indent=1, ensure_ascii=False))
        print(f"  {kb} {q['id']}: {len(nuggets)} nuggets ({sum(n['core'] for n in nuggets)} core)")
    dest.write_text(json.dumps({"kb": kb, "questions": out}, indent=1, ensure_ascii=False))


def main(argv: list[str]) -> None:
    for kb in argv or [k for k, v in KBS.items() if v.tracker and v.public]:
        nuggetize_kb(kb)


if __name__ == "__main__":
    main(sys.argv[1:])
