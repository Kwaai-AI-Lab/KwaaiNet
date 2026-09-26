#!/usr/bin/env python3
"""Cycle-0 builds: clone each KB to <KB>_e2, rebuild its graph from scratch, score, snapshot.

The original KBs are never written. A clone copies every SQLite store with `.backup` (safe while
another process reads the source) plus the tantivy index, and registers a `rag_kbs` entry with the
same tenant_id. `graph build --reset-graph` keeps chunks, so chunk ids match the originals.

    .venv/bin/python driver/build.py [KB ...]        # default: all experiment KBs, in order

Progress: results/eval2/build_progress.json. Snapshots: results/eval2/snapshots/<KB>/c00.db.
Resumable: a KB whose c00 snapshot exists is skipped.
"""
from __future__ import annotations

import json
import re
import shutil
import sqlite3
import subprocess
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from common import KBS, KNOWLEDGE_TESTS, KWAAINET_HOME, RESULTS, data_dir, tenant_of  # noqa: E402

PEERS = {
    "metro-linux": "12D3KooWA33TMz7ss8K2oPQr99KsLwW6AbLYhrL7P1V36vJXXM38",
    "metro-win": "12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE",
}
LOCAL = "local"  # this Mac's Ollama (Metal)
LOCAL_URL = "http://localhost:11434"


def url(peer: str) -> str:
    """Inference URL for a worker: a GPU peer over the p2p relay, or the local Ollama."""
    return LOCAL_URL if peer == LOCAL else f"p2p://{PEERS[peer]}"


BOTH = ",".join(url(p) for p in PEERS)
KWAAINET = "kwaainet"
ORDER = ["Manhattan", "D6", "DeepSea", "Legal", "Climate"]
PROGRESS = RESULTS / "build_progress.json"
SNAP = RESULTS / "snapshots"


def progress(**kw) -> None:
    PROGRESS.parent.mkdir(parents=True, exist_ok=True)
    state = json.loads(PROGRESS.read_text()) if PROGRESS.exists() else {"kbs": {}}
    kb = kw.pop("kb", None)
    if kb:
        state["kbs"].setdefault(kb, {}).update(kw)
        state["current_kb"] = kb
    state["updated"] = time.strftime("%Y-%m-%dT%H:%M:%S")
    PROGRESS.write_text(json.dumps(state, indent=1))


def sqlite_copy(src: Path, dst: Path) -> None:
    s = sqlite3.connect(f"file:{src}?mode=ro", uri=True)
    d = sqlite3.connect(dst)
    s.backup(d)
    d.close()
    s.close()


def clone(kb: str, name: str) -> None:
    cfg_path = KWAAINET_HOME / "config.yaml"
    cfg = cfg_path.read_text()
    src, dst = data_dir(kb), data_dir(name)
    if re.search(rf"(?m)^  {re.escape(name)}:$", cfg):
        return  # already registered (and copied)
    if dst.exists():
        raise SystemExit(f"{dst} exists but {name} is not registered; refusing to overwrite")
    dst.mkdir(parents=True)
    for db in src.glob("*.db"):
        sqlite_copy(db, dst / db.name)
    if (src / "tantivy").exists():
        shutil.copytree(src / "tantivy", dst / "tantivy")
    m = re.search(rf"(?m)^  {re.escape(kb)}:\n(?:    .*\n)+", cfg)
    block = m.group(0).replace(f"  {kb}:", f"  {name}:", 1)
    block = re.sub(r"(?m)^(    rag_data_dir: ).*$", rf"\g<1>{dst}", block)
    cfg_path.write_text(cfg[:m.end()] + block + cfg[m.end():])


def run(args: list[str], log: Path) -> None:
    with log.open("a") as f:
        f.write(f"\n$ {' '.join(args)}\n")
        f.flush()
        r = subprocess.run(args, stdout=f, stderr=subprocess.STDOUT)
    if r.returncode != 0:
        raise RuntimeError(f"{args[:4]} exited {r.returncode}; see {log}")


def graph_score(name: str, save_to: Path | None = None) -> dict:
    """Completeness from `rag graph score --json`: overall plus the mean of each pillar."""
    out = subprocess.run([KWAAINET, "rag", "graph", "score", "--kb", name, "--json"],
                         capture_output=True, text=True, check=True).stdout
    d = json.loads(out[out.index("{"):])
    es = d["entity_scores"]
    n = max(1, len(es))
    if save_to:
        save_to.write_text(json.dumps(d))
    return {"overall": round(100 * d["overall"], 2), "entities": d["entity_count"],
            "relations": d["relation_count"],
            "pillars": {k: round(100 * sum(e[k] for e in es) / n, 2)
                        for k in ("type_score", "summary_score", "relation_score")}}


def snapshot(name: str, dest: Path) -> str:
    dest.parent.mkdir(parents=True, exist_ok=True)
    g = data_dir(name) / f"graph-{tenant_of(name)}.db"
    sqlite_copy(g, dest)
    return _file_sha1(dest)


def _file_sha1(p: Path) -> str:
    import hashlib

    return hashlib.sha1(p.read_bytes()).hexdigest()


METADATA_KEYS = ("doc_metadata", "document_titles")


def restore_metadata(kb: str, name: str) -> dict:
    """Copy document metadata from the original graph into the clone's.

    `graph build --reset-graph` clears the graph's `metadata` table, dropping `doc_metadata` and
    `document_titles`. `rag eval` builds its "Document being queried: ..." preamble from them, so
    without this the rebuilt KB is evaluated with less context than the original.
    """
    src = data_dir(kb) / f"graph-{tenant_of(kb)}.db"
    dst = data_dir(name) / f"graph-{tenant_of(name)}.db"
    s = sqlite3.connect(f"file:{src}?mode=ro", uri=True)
    rows = s.execute(f"SELECT key, value FROM metadata WHERE key IN ({','.join('?' * len(METADATA_KEYS))})",
                     METADATA_KEYS).fetchall()
    s.close()
    d = sqlite3.connect(dst)
    d.executemany("INSERT OR REPLACE INTO metadata (key, value) VALUES (?, ?)", rows)
    d.commit()
    got = dict(d.execute("SELECT key, length(value) FROM metadata").fetchall())
    d.close()
    for k, _ in rows:
        if k not in got:
            raise RuntimeError(f"metadata {k} not restored into {name}")
    return got


def build(kb: str, peers: list[str] | None = None, workers: int = 4) -> None:
    name = f"{kb}_e2"
    snap = SNAP / kb / "c00.db"
    if snap.exists():
        print(f"{kb}: c00 snapshot exists, skipping")
        return
    log = RESULTS / f"build_{name}.log"
    progress(kb=kb, phase="clone", status="running", started=time.time())
    clone(kb, name)
    progress(kb=kb, phase="graph-build", status="running")
    t0 = time.time()
    run([KWAAINET, "rag", "graph", "build", "--kb", name, "--model", "llama3.1:8b",
         "--inference-urls", ",".join(url(p) for p in (peers or list(PEERS))),
         "--workers", str(workers), "--entity-types", KBS[kb].entity_types,
         "--no-relations", "--graph-window", "1", "--reset-graph"], log)
    build_s = time.time() - t0
    if kb == "D6":  # --reset-graph wiped the curated family tree; re-seed it as D6 was always built
        progress(kb=kb, phase="seed", status="running")
        run([KWAAINET, "rag", "graph", "seed", "--kb", name,
             "--file", str(KNOWLEDGE_TESTS / "d6_family_tree.yaml")], log)
    metadata = restore_metadata(kb, name)
    sha = snapshot(name, snap)
    score = graph_score(name, save_to=SNAP / kb / "c00_scores.json")
    meta = {"kb": kb, "clone": name, "cycle": 0, "build_seconds": round(build_s), "score": score,
            "snapshot": str(snap), "sha1": sha, "peers": peers or list(PEERS), "metadata": metadata,
            "ts": time.time()}
    (SNAP / kb / "c00.json").write_text(json.dumps(meta, indent=1))
    progress(kb=kb, phase="done", status="done", build_seconds=round(build_s),
             overall=score["overall"], entities=score["entities"])
    print(f"{kb}: built in {build_s / 3600:.2f} h, completeness {score['overall']}%, "
          f"{score['entities']} entities, {score['relations']} relations")


def main(argv: list[str]) -> None:
    for kb in argv or ORDER:
        try:
            build(kb)
        except Exception as e:  # keep going: one corpus failing must not stall the rest
            progress(kb=kb, status="failed", error=str(e))
            print(f"{kb}: FAILED {e}")


if __name__ == "__main__":
    main(sys.argv[1:])
