#!/usr/bin/env python3
"""Claude-graded passage relevance for retrieval evaluation.

Every (question, retrieved passage) pair in a set of `rag eval --dump-jsonl` files is graded
0/1/2 against the question's gold nuggets:

  2  the passage by itself states at least one nugget
  1  the passage is relevant or partial: it helps, but needs other passages to state a nugget
  0  irrelevant

Grades are cached in SQLite by (kb, qid, passage text hash, model, prompt version), so a passage
retrieved by many snapshots is judged once. Uncached pairs go through the Message Batches API
(half price, asynchronous); a request Claude declines is re-run synchronously with server-side
fallbacks (which Batches does not accept).

    .venv/bin/python judge/relevance.py results/eval2/dumps/<run>.jsonl [...]

Only KBs on the public allowlist are sent.
"""
from __future__ import annotations

import hashlib
import json
import sqlite3
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import claude  # noqa: E402
from common import WORK, read_jsonl, require_public  # noqa: E402
from corpus import clean_text  # noqa: E402

PROMPT_VERSION = "relevance-v1"
CACHE = WORK / "cache" / "relevance.sqlite"
MAX_PASSAGE_CHARS = 4000

SYSTEM = """You grade how relevant one retrieved passage is to a question, for evaluating a retrieval system.

You are given the question, the gold facts ("nuggets") a correct answer contains, and one passage.

Grade:
2 = the passage by itself states at least one of the nuggets (paraphrase counts; the fact must
    actually be stated, not merely implied or topically related).
1 = the passage is relevant but partial: it is about the right subject and contains information
    that would help establish a nugget, but does not by itself state any nugget.
0 = the passage is not relevant to the question.

Judge only what the passage says. Ignore markup, navigation text and formatting noise. List the
ids of the nuggets the passage states (only for grade 2; otherwise an empty list). Keep the
rationale to one short sentence."""

SCHEMA = {
    "type": "object",
    "properties": {
        "grade": {"type": "integer", "enum": [0, 1, 2]},
        "nugget_ids": {"type": "array", "items": {"type": "string"}},
        "rationale": {"type": "string"},
    },
    "required": ["grade", "nugget_ids", "rationale"],
    "additionalProperties": False,
}


def db() -> sqlite3.Connection:
    CACHE.parent.mkdir(parents=True, exist_ok=True)
    con = sqlite3.connect(CACHE)
    con.execute("CREATE TABLE IF NOT EXISTS grades (kb TEXT, qid TEXT, text_hash TEXT, model TEXT, prompt TEXT,"
                " grade INTEGER, nugget_ids TEXT, rationale TEXT, source TEXT, ts REAL,"
                " PRIMARY KEY (kb, qid, text_hash, model, prompt))")
    return con


def load_gold(kb: str) -> dict[str, dict]:
    """qid -> {question, nuggets[]} from the frozen gold file (verified nuggets preferred)."""
    for name in (f"{kb}_gold.json", f"{kb}_verified.json", f"{kb}_nuggets.json"):
        p = WORK / "gold" / name
        if p.exists():
            return {q["id"]: q for q in json.loads(p.read_text())["questions"]}
    raise FileNotFoundError(f"no gold for {kb}")


def user_prompt(question: str, nuggets: list[dict], passage: str) -> str:
    ns = "\n".join(f"- [{n['id']}] {n['text']}" for n in nuggets)
    return f"Question:\n{question}\n\nNuggets:\n{ns}\n\nPassage:\n{passage[:MAX_PASSAGE_CHARS]}"


def custom_id(kb: str, qid: str, h: str) -> str:
    return hashlib.sha1(f"{kb}|{qid}|{h}|{PROMPT_VERSION}".encode()).hexdigest()


def pending_pairs(dumps: list[Path], con: sqlite3.Connection) -> dict[str, dict]:
    """Uncached pairs, keyed by batch custom_id."""
    todo: dict[str, dict] = {}
    gold_cache: dict[str, dict] = {}
    for path in dumps:
        for rec in read_jsonl(path):
            kb = rec["kb"].removesuffix("_e2E").removesuffix("_e2A").removesuffix("_e2B").removesuffix("_e2")
            require_public(kb)
            gold = gold_cache.setdefault(kb, load_gold(kb))
            g = gold.get(rec["qid"])
            if not g or not g.get("nuggets"):
                continue
            for ch in rec["retrieved"]:
                h = ch["text_hash"]
                hit = con.execute("SELECT 1 FROM grades WHERE kb=? AND qid=? AND text_hash=? AND model=? AND prompt=?",
                                  (kb, rec["qid"], h, claude.MODEL, PROMPT_VERSION)).fetchone()
                if hit:
                    continue
                cid = custom_id(kb, rec["qid"], h)
                todo[cid] = {"kb": kb, "qid": rec["qid"], "text_hash": h,
                             "user": user_prompt(g["question"], g["nuggets"], clean_text(ch["text"]))}
    return todo


def store(con: sqlite3.Connection, item: dict, result: dict, source: str) -> None:
    con.execute("INSERT OR REPLACE INTO grades VALUES (?,?,?,?,?,?,?,?,?,?)",
                (item["kb"], item["qid"], item["text_hash"], claude.MODEL, PROMPT_VERSION, int(result["grade"]),
                 json.dumps(result["nugget_ids"]), result["rationale"], source, time.time()))


def run_batch(todo: dict[str, dict], con: sqlite3.Connection, poll_s: int = 30) -> None:
    from anthropic.types.message_create_params import MessageCreateParamsNonStreaming
    from anthropic.types.messages.batch_create_params import Request

    c = claude.client()
    requests = [Request(custom_id=cid, params=MessageCreateParamsNonStreaming(
        model=claude.MODEL, max_tokens=1024,
        system=[{"type": "text", "text": SYSTEM, "cache_control": {"type": "ephemeral"}}],
        output_config={"effort": "low", "format": {"type": "json_schema", "schema": SCHEMA}},
        messages=[{"role": "user", "content": it["user"]}])) for cid, it in todo.items()]
    batch = c.messages.batches.create(requests=requests)
    print(f"batch {batch.id}: {len(requests)} requests")
    while True:
        b = c.messages.batches.retrieve(batch.id)
        if b.processing_status == "ended":
            break
        print(f"  {b.processing_status}: {b.request_counts.processing} processing, "
              f"{b.request_counts.succeeded} done")
        time.sleep(poll_s)
    retry: list[str] = []
    for r in c.messages.batches.results(batch.id):
        item = todo[r.custom_id]
        if r.result.type == "succeeded":
            msg = r.result.message
            if msg.stop_reason == "refusal":
                retry.append(r.custom_id)
                continue
            text = next(bl.text for bl in msg.content if bl.type == "text")
            store(con, item, json.loads(text), "batch")
        else:
            retry.append(r.custom_id)
    con.commit()
    for cid in retry:  # declined or errored: once more, synchronously, with fallbacks
        item = todo[cid]
        res = claude.call_json(kb=item["kb"], kind="relevance-retry", system=SYSTEM, user=item["user"],
                               schema=SCHEMA, effort="low", max_tokens=1024)
        store(con, item, res, "sync")
    con.commit()
    print(f"  stored {len(todo) - len(retry)} from batch, {len(retry)} retried synchronously")


def grades_for(kb: str, qid: str, text_hash: str, con: sqlite3.Connection | None = None) -> dict | None:
    con = con or db()
    row = con.execute("SELECT grade, nugget_ids, rationale FROM grades WHERE kb=? AND qid=? AND text_hash=? "
                      "AND model=? AND prompt=?", (kb, qid, text_hash, claude.MODEL, PROMPT_VERSION)).fetchone()
    return {"grade": row[0], "nugget_ids": json.loads(row[1]), "rationale": row[2]} if row else None


def main(argv: list[str]) -> None:
    con = db()
    todo = pending_pairs([Path(a) for a in argv], con)
    print(f"{len(todo)} uncached (question, passage) pairs")
    if todo:
        run_batch(todo, con)


if __name__ == "__main__":
    main(sys.argv[1:])
