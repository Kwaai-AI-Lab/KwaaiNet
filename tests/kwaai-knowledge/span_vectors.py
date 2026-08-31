#!/usr/bin/env python3
"""Chunk evidence failed (separation 0.000). Does the SPAN work?

A chunk is ~100 words of memoir carrying many relations, so its embedding
represents "a passage from this book", not "a lived_in relation" — every chunk
in one corpus looks alike. The clause that actually states the relation is a
different unit. This extracts the sentence containing both endpoint names and
embeds that instead.
"""
import json, math, os, re, sqlite3, statistics as st, sys, urllib.request, yaml

cfg = yaml.safe_load(open(os.path.expanduser("~/.kwaainet/config.yaml")))["rag_kbs"]["D6"]
m = sqlite3.connect(f"file:{cfg['rag_data_dir']}/{cfg['tenant_id']}.db?mode=ro", uri=True)
g = sqlite3.connect(f"file:{cfg['rag_data_dir']}/graph-{cfg['tenant_id']}.db?mode=ro", uri=True)
ch = {int.from_bytes(k[16:], "little", signed=True): json.loads(v)["text"]
      for k, v in m.execute("SELECT key, CAST(value AS TEXT) FROM chunks")}
ents = {}
for (v,) in g.execute("SELECT CAST(value AS TEXT) FROM entities"):
    e = json.loads(v); ents[e["id"]] = e["name"]
R = [json.loads(v) for (v,) in g.execute("SELECT CAST(value AS TEXT) FROM relations")]

def sentences(t):
    return re.split(r"(?<=[.!?])\s+", re.sub(r"\s+", " ", t))

def span_for(r):
    """The sentence mentioning both endpoints — the clause that states the relation."""
    a, b = ents.get(r["src_id"], ""), ents.get(r["dst_id"], "")
    if not a or not b:
        return None
    # last name is the reliably-present token
    ka, kb = a.split()[-1], b.split()[-1]
    for cid in r["evidence_chunk_ids"]:
        for s in sentences(ch.get(cid, "")):
            if ka in s and kb in s and len(s) > 40:
                return s[:400]
    return None

DROP = {"child_of", "grandchild_of", "foster_child_of"}
groups = {}
for r in R:
    p = r["relation_type"]
    if p in DROP:
        continue
    s = span_for(r)
    if s:
        groups.setdefault(p, []).append(s)

usable = {p: v[:12] for p, v in groups.items() if len(v) >= 3}
print(f"relations with a locatable span: {sum(len(v) for v in groups.values())} "
      f"across {len(groups)} predicates; {len(usable)} predicates have >=3\n", file=sys.stderr)
for p, v in usable.items():
    print(f"  {p:20} {len(v)} spans   e.g. {v[0][:70]}…", file=sys.stderr)

def embed(t):
    body = json.dumps({"model": "nomic-embed-text", "prompt": t}).encode()
    rq = urllib.request.Request("http://localhost:11434/api/embeddings", data=body,
                                headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(rq, timeout=60) as r:
        return json.load(r)["embedding"]

V = {p: [embed(t) for t in v] for p, v in usable.items()}
json.dump({"spans": usable}, open("results/span_samples.json", "w"), indent=1)

def cos(a, b):
    d = sum(x*y for x, y in zip(a, b))
    return d / (math.sqrt(sum(x*x for x in a)) * math.sqrt(sum(x*x for x in b)))

preds = list(V)
within = {p: st.mean(cos(V[p][i], V[p][j]) for i in range(len(V[p]))
                     for j in range(i+1, len(V[p]))) for p in preds if len(V[p]) > 1}
between = [st.mean(cos(x, y) for x in V[a] for y in V[b])
           for i, a in enumerate(preds) for b in preds[i+1:]]
print(f"\n{'predicate':22}{'n':>4}{'within':>10}")
print("-" * 38)
for p in sorted(within, key=lambda p: -within[p]):
    print(f"{p:22}{len(V[p]):>4}{within[p]:>10.3f}")
w, b = st.mean(within.values()), st.mean(between)
print(f"\n  mean WITHIN  : {w:.3f}\n  mean BETWEEN : {b:.3f}\n  separation   : {w-b:+.3f}")
