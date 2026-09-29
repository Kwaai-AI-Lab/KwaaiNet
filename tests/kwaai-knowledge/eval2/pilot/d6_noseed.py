#!/usr/bin/env python3
"""D6 without seed data: does recall rise with dream cycles? First-10% trial.

Plan and pre-registered verdict: projects/kwaai-knowledge/plans/D6-NoSeed-Dream-plan.md.

The graph is built from the first 10% of D6 in reading order, with no `graph seed` (Eval v2
re-seeded d6_family_tree.yaml, which plants hand-written descriptions that state eval answers).
Vector and BM25 search still cover the whole book. Two arms then dream for 24 cycles each:
  A  `dream run --no-relations`   descriptions and types only
  B  `dream run`                  relation completion as well
Evals run the 31 questions with a nugget whose gold passage is inside the slice, and score only
those nuggets (work/gold/D6_s10_gold.json).

Everything runs on this Mac (D6 never leaves it), one heavy job at a time: the Mac hung twice
running local Ollama alongside other jobs. Before each job the driver waits for >= 20% free
memory and >= 15 GB free disk. NLI scoring runs last, after every eval, so the NLI model and
Ollama never share memory.

    .venv/bin/python pilot/d6_noseed.py            # full run, resumable
    .venv/bin/python pilot/d6_noseed.py --smoke    # build, one cycle per arm, one eval; then stop
    .venv/bin/python pilot/d6_noseed.py --graph-only   # re-evaluate saved snapshots, graph only

State: results/d6_noseed_s10/state.json (a finished step is never redone; a failed step is retried
once). Progress: results/d6_noseed_s10/progress.json. Log: results/d6_noseed_s10/run.log.
"""
from __future__ import annotations

import json
import math
import os
import re
import shutil
import sqlite3
import subprocess
import sys
import time
import traceback
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "driver"))
import build  # noqa: E402
from build import LOCAL_URL, clone, graph_score, restore_metadata, sqlite_copy  # noqa: E402
from common import KNOWLEDGE_TESTS, RESULTS, WORK, data_dir, read_jsonl, tenant_of  # noqa: E402
from run_experiment import restore_graph  # noqa: E402

EVAL2 = Path(__file__).resolve().parents[1]
BIN = str(Path.home() / ".cargo/bin/kwaainet-d6s")
build.KWAAINET = BIN  # graph_score runs `rag graph score`: use the binary that built and dreamed
MODEL = "llama3.1:8b"
SRC = "D6"          # the original KB; only read (cloned), never written
BASE = "D6_s10"     # clones: D6_s10 (build), D6_s10A / D6_s10B (arms), D6_s10E (evals)
PCT = 10
ENTITY_TYPES = "Person,Place,Organization,Legislation,Publication"  # Eval v2's D6 build
MAX_CYCLE = 24
EVAL_CYCLES = (0, 1, 2, 4, 8, 12, 16, 20, 24)
C0_REPEATS = 3      # cycle 0, iterative
END_REPEATS = 3     # cycle 24 per arm: r1 plus two repeats
COMPLETIONS = 200   # per cycle, as Eval v2
DREAM_WORKERS = 2
GENERATION_FAILURES = ("(error:", "(inference error:", "(no response")
MIN_FREE_PCT = 20
MIN_FREE_GB = 15

OUT = RESULTS.parent / "d6_noseed_s10"
SNAP, DUMPS, LOGS = OUT / "snapshots", OUT / "dumps", OUT / "logs"
STATE, PROGRESS = OUT / "state.json", OUT / "progress.json"
GOLD = WORK / "gold" / f"{BASE}_gold.json"
QUESTIONS = OUT / f"{BASE}_questions.json"
# The seed file Eval v2 applied after the build. Its hand-written descriptions must not appear in
# the no-seed graph (slice_check); the phrases are read from the file, not written out here.
SEED_FILE = KNOWLEDGE_TESTS / "d6_family_tree.yaml"


# -- bookkeeping ---------------------------------------------------------------------------------
def log(msg: str) -> None:
    line = f"[{time.strftime('%H:%M:%S')}] {msg}"
    print(line, flush=True)
    with (OUT / "run.log").open("a") as f:
        f.write(line + "\n")


def load_state() -> dict:
    return json.loads(STATE.read_text()) if STATE.exists() else {"steps": {}}


def save_state(st: dict, current: str = "") -> None:
    STATE.write_text(json.dumps(st, indent=1))
    failed = {k: v.get("error", "")[:200] for k, v in st["steps"].items() if v.get("status") == "failed"}
    done = sum(v.get("status") == "done" for v in st["steps"].values())
    PROGRESS.write_text(json.dumps({"updated": time.strftime("%Y-%m-%dT%H:%M:%S"), "current": current,
                                    "done": done, "failed": failed}, indent=1))


def step(st: dict, key: str, fn):
    s = st["steps"].get(key, {})
    if s.get("status") == "done":
        return s["result"]
    if s.get("attempts", 0) >= 2:
        raise RuntimeError(f"{key}: failed twice; needs a human")
    wait_for_resources(key)
    st["steps"][key] = {"status": "running", "attempts": s.get("attempts", 0) + 1, "started": time.time()}
    save_state(st, key)
    log(f"start {key}")
    try:
        res = fn()
    except KeyboardInterrupt:
        raise  # left "running": the next start retries it without counting an attempt
    except BaseException as e:  # noqa: BLE001
        st["steps"][key].update(status="failed", error=f"{e!r}\n{traceback.format_exc()[-1500:]}")
        save_state(st)
        log(f"FAILED {key}: {e!r}")
        raise
    st["steps"][key].update(status="done", result=res, finished=time.time())
    save_state(st)
    log(f"done {key}: {json.dumps(res)[:300]}")
    return res


def run(args: list[str], logfile: Path, env: dict | None = None) -> None:
    logfile.parent.mkdir(parents=True, exist_ok=True)
    with logfile.open("a") as f:
        f.write(f"\n$ {' '.join(args)}\n")
        f.flush()
        r = subprocess.run(args, stdout=f, stderr=subprocess.STDOUT, env={**os.environ, **(env or {})})
    if r.returncode != 0:
        raise RuntimeError(f"exit {r.returncode}: {' '.join(args[:5])} (see {logfile})")


def free_memory_pct() -> int | None:
    out = subprocess.run(["memory_pressure"], capture_output=True, text=True).stdout
    m = re.search(r"free percentage:\s*(\d+)%", out)
    return int(m.group(1)) if m else None


def wait_for_resources(key: str) -> None:
    warned = False
    while True:
        mem = free_memory_pct()
        disk = shutil.disk_usage("/System/Volumes/Data").free / 2**30
        if (mem is None or mem >= MIN_FREE_PCT) and disk >= MIN_FREE_GB:
            if warned:
                log(f"resources ok again (memory {mem}% free, disk {disk:.0f} GB): starting {key}")
            return
        if not warned:
            log(f"WAITING before {key}: memory {mem}% free (need {MIN_FREE_PCT}), "
                f"disk {disk:.0f} GB free (need {MIN_FREE_GB})")
            warned = True
        time.sleep(60)


# -- the slice -----------------------------------------------------------------------------------
def slice_chunks() -> tuple[int, set[bytes]]:
    """(cutoff, chunk keys) of the first PCT% of D6 in reading order, as `--sample-pct` takes it."""
    db = data_dir(SRC) / f"{tenant_of(SRC)}.db"
    con = sqlite3.connect(f"file:{db}?mode=ro", uri=True)
    rows = [(k[-8:], json.loads(v)) for k, v in con.execute("SELECT key, value FROM chunks")]
    con.close()
    rows.sort(key=lambda r: (r[1]["doc_name"], r[1]["chunk_index"]))
    n = math.ceil(len(rows) * PCT / 100)  # rag_cmd: (len * pct).div_ceil(100)
    assert len({r[1]["doc_name"] for r in rows}) == 1, "slice logic assumes a single document"
    # make_gold compares gold passages' chunk_index with this count: valid only without gaps.
    assert [r[1]["chunk_index"] for r in rows] == list(range(len(rows))), "chunk_index has gaps"
    return n, {k for k, _ in rows[:n]}


def make_gold(cutoff: int) -> dict:
    """Slice gold: keep nuggets with a gold passage inside the slice; questions with at least one."""
    full = json.loads((WORK / "gold" / "D6_gold.json").read_text())
    kept = []
    for q in full["questions"]:
        ns = [n for n in q["nuggets"] if any(gp["chunk_index"] < cutoff for gp in n.get("gold_passages", []))]
        if ns:
            kept.append({**q, "nuggets": ns})
    GOLD.write_text(json.dumps({**full, "questions": kept, "slice": {"kb": SRC, "pct": PCT, "cutoff": cutoff}},
                               indent=1))
    ids = {q["id"] for q in kept}
    qs = [q for q in json.loads((KNOWLEDGE_TESTS / "d6_eval_questions.json").read_text()) if q["id"] in ids]
    QUESTIONS.write_text(json.dumps(qs, indent=1))
    reach = sum(n.get("status") in ("verified", "supported") for q in kept for n in q["nuggets"])
    return {"cutoff": cutoff, "questions": len(qs), "reachable_nuggets": reach}


def seed_fingerprints() -> list[str]:
    """The opening of every hand-written description in SEED_FILE, whitespace-normalised."""
    import yaml

    seed = yaml.safe_load(SEED_FILE.read_text())
    return [" ".join(e["description"].split())[:60]
            for group in seed.values() if isinstance(group, list)
            for e in group if isinstance(e, dict) and e.get("description")]


def graph_db(name: str) -> Path:
    return data_dir(name) / f"graph-{tenant_of(name)}.db"


def slice_check(snapshot: Path, slice_keys: set[bytes]) -> dict:
    """The cycle-0 graph is built from the slice only, with no relations and no seed content."""
    con = sqlite3.connect(f"file:{snapshot}?mode=ro", uri=True)
    chunk_keys = [bytes(k) for (k,) in con.execute("SELECT key FROM chunk_entity")]
    relations = con.execute("SELECT count(*) FROM relations").fetchone()[0]
    blobs = b" ".join(bytes(v) for (v,) in con.execute("SELECT value FROM entities"))
    con.close()
    outside = [k.hex() for k in chunk_keys if k not in slice_keys]
    text = " ".join(blobs.decode("utf-8", "ignore").split())
    seeded = [p for p in seed_fingerprints() if p in text]
    if outside or relations or seeded:
        raise RuntimeError(f"slice check failed: {len(outside)} chunks outside the slice, "
                           f"{relations} relations, seed phrases {seeded}")
    return {"chunks_with_entities": len(chunk_keys), "relations": relations}


# -- jobs ----------------------------------------------------------------------------------------
def snap(arm: str, c: int) -> Path:
    return SNAP / ("c00.db" if c == 0 else f"{arm}_c{c:02d}.db")


def do_build(slice_keys: set[bytes]) -> dict:
    clone(SRC, BASE)
    t0 = time.time()
    run([BIN, "rag", "graph", "build", "--kb", BASE, "--model", MODEL, "--inference-urls", LOCAL_URL,
         "--workers", str(DREAM_WORKERS), "--entity-types", ENTITY_TYPES, "--no-relations",
         "--graph-window", "1", "--reset-graph", "--sample-pct", str(PCT)], LOGS / "build.log")
    secs = time.time() - t0
    restore_metadata(SRC, BASE)  # --reset-graph drops the document titles `rag eval` uses
    SNAP.mkdir(parents=True, exist_ok=True)
    sqlite_copy(graph_db(BASE), snap("0", 0))
    check = slice_check(snap("0", 0), slice_keys)
    score = graph_score(BASE, save_to=SNAP / "c00.scores.json")
    return {"seconds": round(secs), **check, "score": score}


def do_dream(arm: str, c: int) -> dict:
    name = f"{BASE}{arm}"
    clone(BASE, name)
    restore_graph(snap(arm, c - 1), name)  # every cycle starts from the previous snapshot
    args = [BIN, "rag", "dream", "run", "--kb", name, "--model", MODEL, "--inference-url", LOCAL_URL,
            "--workers", str(DREAM_WORKERS), "--max-completions", str(COMPLETIONS)]
    if arm == "A":
        args.append("--no-relations")
    t0 = time.time()
    run(args, LOGS / f"dream_{arm}.log")
    secs = time.time() - t0
    report = data_dir(name) / f"dream-report-{tenant_of(name)}.json"
    if not report.exists() or report.stat().st_mtime < t0:
        raise RuntimeError(f"dream wrote no fresh report ({report})")
    rep = json.loads(report.read_text())
    if rep["cycle_errors"]:
        raise RuntimeError(f"dream cycle had errors: {rep['cycle_errors'][:3]}")
    work = rep["entities_summary_completed"] + rep["entities_type_completed"] + rep["entities_relations_added"]
    sqlite_copy(graph_db(name), snap(arm, c))
    shutil.copy(report, snap(arm, c).with_suffix(".dream.json"))
    score = graph_score(name, save_to=snap(arm, c).with_suffix(".scores.json"))
    # Zero work is saturation, not failure: with ~150 entities the arm can run out of things to
    # complete before cycle 24. The graph is snapshotted unchanged and the curve shows the plateau.
    return {"seconds": round(secs), "work": work, "saturated": work == 0, "score": score}


def do_eval(arm: str, c: int, rep: int, mode: str) -> dict:
    name = f"{BASE}E"
    clone(BASE, name)
    restore_graph(snap(arm, c), name)
    tag = f"{BASE}:{arm if c else '0'}:c{c:02d}:r{rep}:{mode}"
    stem = tag.replace(":", "_")
    DUMPS.mkdir(parents=True, exist_ok=True)
    dump = DUMPS / f"{stem}.jsonl"
    dump.unlink(missing_ok=True)  # a retried eval must not append to a partial dump
    t0 = time.time()
    run([BIN, "rag", "eval", "--kb", name, "--questions", str(QUESTIONS), "--inference-url", LOCAL_URL,
         "--model", MODEL, "-k", "20", "--mode", mode, "--semantic-score", "--semantic-low", "0.30",
         "--semantic-high", "0.85", "--output", str(DUMPS / f"{stem}.md"),
         "--progress-file", str(DUMPS / f"{stem}.progress.json"), "--dump-jsonl", str(dump),
         "--run-tag", tag], LOGS / "eval.log")
    recs = read_jsonl(dump)
    # `rag eval` records a failed generation as its answer: "(error: ...)", "(inference error: ...)",
    # or "(no response...)" when Ollama answers without content (e.g. it shed the request under
    # memory pressure). Each would be scored as a wrong answer inside a "done" step.
    bad = [r["qid"] for r in recs if r["answer"].startswith(GENERATION_FAILURES)]
    if bad:
        raise RuntimeError(f"{len(bad)} generation failures in {stem}.jsonl: {bad[:5]}")
    expected = len(json.loads(QUESTIONS.read_text()))
    if len(recs) != expected:
        raise RuntimeError(f"{stem}.jsonl has {len(recs)} records, expected {expected}")
    return {"seconds": round(time.time() - t0), "records": len(recs)}


def do_metrics() -> dict:
    out = OUT / "metrics.jsonl"
    dumps = sorted(str(p) for p in DUMPS.glob("*.jsonl"))
    run([str(EVAL2 / ".venv/bin/python"), str(EVAL2 / "metrics.py"), *dumps], LOGS / "metrics.log",
        env={"EVAL2_METRICS_OUT": str(out)})
    return {"rows": len(read_jsonl(out))}


# Graph-only follow-up (2026-09-29): the same snapshots, retrieved with `--mode graph-only`, so the
# model sees entity cards and no chunk text. Tests the thesis that dreaming moves what the chunk
# store holds (short-term memory) into the graph (long-term memory).
GRAPH_ONLY_CYCLES = (1, 4, 12, 24)
GRAPH_ONLY_REPEATS = 2  # at cycle 0, and at cycle 24 per arm


def run_graph_only(st: dict) -> None:
    missing = [p for p in [snap("0", 0)] + [snap(a, c) for a in "AB" for c in GRAPH_ONLY_CYCLES] if not p.exists()]
    if missing:
        raise SystemExit(f"graph-only needs the trial's snapshots; missing {[p.name for p in missing]}")
    for rep in range(1, GRAPH_ONLY_REPEATS + 1):
        step(st, f"eval:0:c00:r{rep}:graph-only", lambda rep=rep: do_eval("0", 0, rep, "graph-only"))
    for arm in "AB":
        for c in GRAPH_ONLY_CYCLES:
            step(st, f"eval:{arm}:c{c:02d}:r1:graph-only", lambda arm=arm, c=c: do_eval(arm, c, 1, "graph-only"))
        for rep in range(2, GRAPH_ONLY_REPEATS + 1):
            step(st, f"eval:{arm}:c{MAX_CYCLE}:r{rep}:graph-only",
                 lambda arm=arm, rep=rep: do_eval(arm, MAX_CYCLE, rep, "graph-only"))
    step(st, "metrics:graph-only", do_metrics)  # metrics.py skips rows it already scored
    log("graph-only evals finished")


# -- the queue -----------------------------------------------------------------------------------
def main(argv: list[str]) -> None:
    smoke = "--smoke" in argv
    for d in (OUT, SNAP, DUMPS, LOGS):
        d.mkdir(parents=True, exist_ok=True)
    if not Path(BIN).exists():
        raise SystemExit(f"{BIN} not installed")
    st = load_state()
    for v in st["steps"].values():
        if v.get("status") == "running":  # killed mid-step (reboot, kill): run it again, and
            v["status"] = "pending"       # don't count the interruption as a failed attempt
            v["attempts"] = max(0, v.get("attempts", 1) - 1)
    save_state(st)
    log(f"d6_noseed start{' (smoke)' if smoke else ''}{' (graph-only)' if '--graph-only' in argv else ''}; "
        f"binary {BIN}")
    if "--graph-only" in argv:
        run_graph_only(st)
        return

    cutoff, slice_keys = slice_chunks()
    step(st, "gold", lambda: make_gold(cutoff))
    step(st, "build", lambda: do_build(slice_keys))
    step(st, "eval:0:c00:r1:iterative", lambda: do_eval("0", 0, 1, "iterative"))
    if smoke:
        for arm in "AB":
            step(st, f"dream:{arm}:c01", lambda arm=arm: do_dream(arm, 1))
        log("smoke run finished: build, one cycle per arm, one eval")
        return
    step(st, "eval:0:c00:r1:vector", lambda: do_eval("0", 0, 1, "vector"))
    for rep in range(2, C0_REPEATS + 1):
        step(st, f"eval:0:c00:r{rep}:iterative", lambda rep=rep: do_eval("0", 0, rep, "iterative"))
    for arm in "AB":
        for c in range(1, MAX_CYCLE + 1):
            step(st, f"dream:{arm}:c{c:02d}", lambda arm=arm, c=c: do_dream(arm, c))
            if c in EVAL_CYCLES:
                step(st, f"eval:{arm}:c{c:02d}:r1:iterative", lambda arm=arm, c=c: do_eval(arm, c, 1, "iterative"))
        for rep in range(2, END_REPEATS + 1):
            step(st, f"eval:{arm}:c{MAX_CYCLE}:r{rep}:iterative",
                 lambda arm=arm, rep=rep: do_eval(arm, MAX_CYCLE, rep, "iterative"))
    step(st, "metrics", do_metrics)
    log("d6_noseed finished")


if __name__ == "__main__":
    main(sys.argv[1:])
