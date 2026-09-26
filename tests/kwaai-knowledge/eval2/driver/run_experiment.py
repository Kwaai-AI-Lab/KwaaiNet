#!/usr/bin/env python3
"""Dream trajectories and snapshot evaluations for eval2.

Jobs (each runs on exactly one GPU peer at a time; one job per peer):
  dream  (kb, arm, cycle)   one `rag dream run` on the arm's clone, then score + snapshot.
                            Arm A (--no-relations) dreams on metro-linux, arm B on metro-win.
  eval   (kb, arm, cycle, rep, mode)
                            restore the snapshot into <KB>_e2E, then `kwaainet-eval2 rag eval
                            --dump-jsonl`. Pinned to one peer per corpus, so a trajectory's
                            measurements never change hardware.

Cycle 0 is the shared c00 snapshot written by driver/build.py. Every cycle is snapshotted (cheap,
and it makes a crash resumable from the last completed cycle); only EVAL_CYCLES are evaluated.
Cycle 0 is also evaluated REPEATS times (noise floor) and once with --mode vector (graph-free).

    .venv/bin/python driver/run_experiment.py [KB ...]

State: results/eval2/state.json (resumable). Progress: results/eval2/eval2_progress.json.
"""
from __future__ import annotations

import json
import shutil
import sqlite3
import subprocess
import sys
import threading
import time
import traceback
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
sys.path.insert(0, str(Path(__file__).resolve().parent))
from build import LOCAL, PEERS, SNAP, build, clone, graph_score, sqlite_copy, url, _file_sha1  # noqa: E402
from common import KBS, KNOWLEDGE_TESTS, RESULTS, data_dir, tenant_of  # noqa: E402

KWAAINET = "kwaainet"
KWAAINET_EVAL = str(Path.home() / ".cargo/bin/kwaainet-eval2")
MODEL = "llama3.1:8b"
MAX_CYCLE = 12
EVAL_CYCLES = (0, 1, 2, 4, 8, 12)
REPEATS = 3  # cycle-0 evaluations per corpus (noise floor)
COMPLETIONS = 200
DREAM_WORKERS = 2  # per peer
ARM_PEER = {"A": "metro-linux", "B": "metro-win"}
# D6 (the memoir) runs entirely on this Mac: build, both arms and evals. One machine per
# trajectory, and the private text never leaves the Mac.
LOCAL_KBS = {"D6"}
WORKERS = [*PEERS, LOCAL]
EVAL_PEER = {"Manhattan": "metro-linux", "DeepSea": "metro-linux", "Climate": "metro-win",
             "D6": LOCAL, "Legal": "metro-win"}
ORDER = ["Manhattan", "D6", "DeepSea", "Legal", "Climate"]
# Cycle-0 builds, one peer each (two builds run side by side).
BUILD_PEER = {"Manhattan": "metro-linux", "D6": LOCAL, "DeepSea": "metro-linux",
              "Legal": "metro-win", "Climate": "metro-win"}
STATE = RESULTS / "state.json"
PROGRESS = RESULTS / "eval2_progress.json"
DUMPS = RESULTS / "dumps"
LOGS = RESULTS / "logs"
_lock = threading.Lock()


# -- state ---------------------------------------------------------------------------------------
def load_state() -> dict:
    return json.loads(STATE.read_text()) if STATE.exists() else {"jobs": {}}


def save_state(state: dict) -> None:
    STATE.write_text(json.dumps(state, indent=1))
    running = {k: v for k, v in state["jobs"].items() if v["status"] == "running"}
    done = sum(v["status"] == "done" for v in state["jobs"].values())
    failed = {k: v.get("error", "")[:200] for k, v in state["jobs"].items() if v["status"] == "failed"}
    PROGRESS.write_text(json.dumps({"updated": time.strftime("%Y-%m-%dT%H:%M:%S"), "done": done,
                                    "running": running, "failed": failed}, indent=1))


def job_key(j: dict) -> str:
    if j["type"] == "build":
        return f"build:{j['kb']}"
    if j["type"] == "dream":
        return f"dream:{j['kb']}:{j['arm']}:c{j['cycle']:02d}"
    return f"eval:{j['kb']}:{j['arm']}:c{j['cycle']:02d}:r{j['rep']}:{j['mode']}"


def snap_path(kb: str, arm: str, cycle: int) -> Path:
    return SNAP / kb / "c00.db" if cycle == 0 else SNAP / kb / f"{arm}_c{cycle:02d}.db"


# -- jobs ----------------------------------------------------------------------------------------
def all_jobs(kbs: list[str]) -> list[dict]:
    jobs = []
    for kb in kbs:
        jobs.append({"type": "build", "kb": kb, "arm": "0", "cycle": 0, "peer": BUILD_PEER[kb]})
        for arm in ("A", "B"):
            for c in range(1, MAX_CYCLE + 1):
                jobs.append({"type": "dream", "kb": kb, "arm": arm, "cycle": c,
                             "peer": LOCAL if kb in LOCAL_KBS else ARM_PEER[arm]})
        for rep in range(1, REPEATS + 1):
            jobs.append({"type": "eval", "kb": kb, "arm": "0", "cycle": 0, "rep": rep, "mode": "iterative",
                         "peer": EVAL_PEER[kb]})
        jobs.append({"type": "eval", "kb": kb, "arm": "0", "cycle": 0, "rep": 1, "mode": "vector",
                     "peer": EVAL_PEER[kb]})
        for arm in ("A", "B"):
            for c in EVAL_CYCLES[1:]:
                jobs.append({"type": "eval", "kb": kb, "arm": arm, "cycle": c, "rep": 1, "mode": "iterative",
                             "peer": EVAL_PEER[kb]})
    return jobs


def ready(j: dict, state: dict) -> bool:
    jobs = state["jobs"]
    if j["type"] == "build":
        return True
    if not (SNAP / j["kb"] / "c00.db").exists():
        return False
    if j["type"] == "dream":
        return j["cycle"] == 1 or jobs.get(job_key({**j, "cycle": j["cycle"] - 1}), {}).get("status") == "done"
    if j["cycle"] == 0:
        return True
    return jobs.get(job_key({"type": "dream", "kb": j["kb"], "arm": j["arm"], "cycle": j["cycle"]}),
                    {}).get("status") == "done"


def ensure_clone(kb: str, suffix: str) -> str:
    name = f"{kb}_e2{suffix}"
    with _lock:  # clone() edits config.yaml
        clone(f"{kb}_e2", name)
    return name


def restore_graph(snapshot: Path, name: str) -> None:
    dst = data_dir(name) / f"graph-{tenant_of(name)}.db"
    for side in (dst.with_name(dst.name + "-wal"), dst.with_name(dst.name + "-shm")):
        side.unlink(missing_ok=True)
    sqlite_copy(snapshot, dst)


def run_logged(args: list[str], log: Path) -> None:
    log.parent.mkdir(parents=True, exist_ok=True)
    with log.open("a") as f:
        f.write(f"\n$ {' '.join(args)}\n")
        f.flush()
        r = subprocess.run(args, stdout=f, stderr=subprocess.STDOUT)
    if r.returncode != 0:
        raise RuntimeError(f"exit {r.returncode}: {' '.join(args[:5])} (see {log})")


def do_dream(j: dict) -> dict:
    kb, arm, c = j["kb"], j["arm"], j["cycle"]
    name = ensure_clone(kb, arm)
    # Resume exactness: start every cycle from the previous cycle's snapshot.
    restore_graph(snap_path(kb, arm, c - 1), name)
    args = [KWAAINET, "rag", "dream", "run", "--kb", name, "--model", MODEL,
            "--inference-urls", url(j["peer"]), "--workers", str(DREAM_WORKERS),
            "--max-completions", str(COMPLETIONS)]
    if arm == "A":
        args.append("--no-relations")
    t0 = time.time()
    run_logged(args, LOGS / f"dream_{kb}_{arm}.log")
    secs = time.time() - t0
    snap = snap_path(kb, arm, c)
    sqlite_copy(data_dir(name) / f"graph-{tenant_of(name)}.db", snap)
    score = graph_score(name, save_to=snap.with_suffix(".scores.json"))
    report = data_dir(name) / f"dream-report-{tenant_of(name)}.json"
    if report.exists():
        shutil.copy(report, snap.with_suffix(".dream.json"))
    meta = {"kb": kb, "arm": arm, "cycle": c, "seconds": round(secs), "peer": j["peer"], "score": score,
            "sha1": _file_sha1(snap), "ts": time.time()}
    snap.with_suffix(".json").write_text(json.dumps(meta, indent=1))
    return {"seconds": round(secs), "overall": score["overall"], "entities": score["entities"],
            "relations": score["relations"]}


def questions_for(kb: str) -> Path:
    return KNOWLEDGE_TESTS / KBS[kb].questions


def do_eval(j: dict) -> dict:
    kb, arm, c = j["kb"], j["arm"], j["cycle"]
    name = ensure_clone(kb, "E")
    snap = snap_path(kb, arm if c else "0", c)
    restore_graph(snap, name)
    tag = f"{kb}:{arm}:c{c:02d}:r{j['rep']}:{j['mode']}"
    stem = tag.replace(":", "_")
    DUMPS.mkdir(parents=True, exist_ok=True)
    args = [KWAAINET_EVAL, "rag", "eval", "--kb", name, "--questions", str(questions_for(kb)),
            "--inference-url", url(j["peer"]), "--model", MODEL, "-k", "20",
            "--mode", j["mode"], "--semantic-score", "--semantic-low", "0.30", "--semantic-high", "0.85",
            "--output", str(DUMPS / f"{stem}.md"), "--progress-file", str(DUMPS / f"{stem}.progress.json"),
            "--dump-jsonl", str(DUMPS / f"{stem}.jsonl"), "--run-tag", tag]
    t0 = time.time()
    run_logged(args, LOGS / f"eval_{kb}.log")
    n = sum(1 for _ in (DUMPS / f"{stem}.jsonl").open())
    return {"seconds": round(time.time() - t0), "records": n, "snapshot_sha1": _file_sha1(snap)}


# -- scheduler -----------------------------------------------------------------------------------
def worker(peer: str, kbs: list[str], state: dict, stop: threading.Event) -> None:
    rank = {kb: i for i, kb in enumerate(kbs)}
    while not stop.is_set():
        with _lock:
            pending = [j for j in all_jobs(kbs) if j["peer"] == peer
                       and state["jobs"].get(job_key(j), {}).get("status") not in ("done", "running")
                       and ready(j, state)]
            # Earlier corpora first; within a corpus, evals of ready snapshots before further dreaming
            # so results for the minimum viable set arrive early.
            order = {"build": 0, "eval": 1, "dream": 2}
            # Builds before anything else (they unblock whole corpora), then by corpus.
            pending.sort(key=lambda j: (j["type"] != "build", rank[j["kb"]], order[j["type"]], j["cycle"],
                                        j.get("arm", "")))
            if not pending:
                job = None
            else:
                job = pending[0]
                state["jobs"][job_key(job)] = {"status": "running", "peer": peer, "started": time.time()}
                save_state(state)
        if job is None:
            outstanding = [j for j in all_jobs(kbs) if j["peer"] == peer
                           and state["jobs"].get(job_key(j), {}).get("status") != "done"]
            if not outstanding:
                return
            time.sleep(60)
            continue
        try:
            if job["type"] == "build":
                build(job["kb"], peers=[peer], workers=DREAM_WORKERS)  # skips if c00 exists
                result = {}
            elif job["type"] == "dream":
                result = do_dream(job)
            else:
                result = do_eval(job)
            status = {"status": "done", "peer": peer, "finished": time.time(), **result}
        except Exception as e:  # noqa: BLE001 — record and move on; a failed job is retried on restart
            status = {"status": "failed", "peer": peer, "error": f"{e}\n{traceback.format_exc()[-800:]}"}
        with _lock:
            state["jobs"][job_key(job)] = status
            save_state(state)
        print(f"[{peer}] {job_key(job)} -> {status['status']} {({k: v for k, v in status.items() if k not in ('error',)})}",
              flush=True)


def main(argv: list[str]) -> None:
    kbs = argv or ORDER
    state = load_state()
    for k, v in state["jobs"].items():  # a job left "running" by a crash is redone
        if v["status"] in ("running", "failed"):
            v["status"] = "pending"
    save_state(state)
    stop = threading.Event()
    threads = [threading.Thread(target=worker, args=(p, kbs, state, stop), daemon=True) for p in WORKERS]
    for t in threads:
        t.start()
    try:
        for t in threads:
            t.join()
    except KeyboardInterrupt:
        stop.set()


if __name__ == "__main__":
    main(sys.argv[1:])
