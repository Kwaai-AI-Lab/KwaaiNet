#!/usr/bin/env python3
"""Dump short corpus excerpts per KB, for foundation-model ontology authoring (arm B)."""
import json, os, sys, random, sqlite3, yaml, re
CONFIG = os.path.expanduser("~/.kwaainet/config.yaml")
cfg = yaml.safe_load(open(CONFIG))["rag_kbs"]
def dump(kb, n=6, w=520):
    e = cfg[kb]; d = e.get("rag_data_dir") or os.path.expanduser(f"~/.kwaainet/rag/{kb}")
    db = os.path.join(d, f"{e['tenant_id']}.db")
    c = sqlite3.connect(f"file:{db}?mode=ro", uri=True)
    rows = [json.loads(v) for (v,) in c.execute("SELECT CAST(value AS TEXT) FROM chunks")]
    c.close()
    rows = [r for r in rows if len(r["text"].strip()) > 300]
    random.Random(5).shuffle(rows)
    print(f"\n{'='*70}\n### {kb}  ({len(rows)} chunks)\n{'='*70}")
    for r in rows[:n]:
        t = re.sub(r"\s+", " ", r["text"]).strip()
        print(f"\n[{r['doc_name'][:60]}]\n{t[:w]}")
for kb in sys.argv[1:]:
    dump(kb)
