#!/usr/bin/env python3
"""Does embedding a predicate's EVIDENCE beat embedding its NAME?

The label carries whatever the extractor chose to call the relation. The evidence
is the text the relation was actually read out of. If meaning lives in the second,
evidence vectors should cluster same-predicate edges more tightly and separate
different predicates more widely than name vectors do.

Excludes auto-generated inverses: `parent_of` and `child_of` are written from one
sentence, so their evidence is identical by construction and no embedding can
separate them. Direction is the ontology's job (`inverse:`), not the vector's.
"""
import json, math, os, sqlite3, statistics as st, sys, urllib.request, yaml

CACHE = "results/evidence_vectors.json"
cfg = yaml.safe_load(open(os.path.expanduser("~/.kwaainet/config.yaml")))["rag_kbs"]["D6"]
g = sqlite3.connect(f"file:{cfg['rag_data_dir']}/graph-{cfg['tenant_id']}.db?mode=ro", uri=True)
m = sqlite3.connect(f"file:{cfg['rag_data_dir']}/{cfg['tenant_id']}.db?mode=ro", uri=True)

chunks = {}
for (k, v) in m.execute("SELECT key, CAST(value AS TEXT) FROM chunks"):
    chunks[int.from_bytes(k[16:], "little", signed=True)] = json.loads(v)["text"]
R = [json.loads(v) for (v,) in g.execute("SELECT CAST(value AS TEXT) FROM relations")]

# One side of each inverse pair only.
DROP = {"child_of", "grandchild_of", "foster_child_of"}
KEEP = {"located_in", "parent_of", "spouse_of", "sibling_of", "half_sibling_of",
        "grandparent_of", "belongs_to", "member_of", "lived_in", "foster_parent_of",
        "associated_with"}
groups = {}
for r in R:
    p = r["relation_type"]
    if p in DROP or p not in KEEP:
        continue
    txt = " ".join(chunks.get(c, "") for c in r["evidence_chunk_ids"][:2])[:900]
    if len(txt) > 120:
        groups.setdefault(p, []).append(txt)

def embed(t):
    body = json.dumps({"model": "nomic-embed-text", "prompt": t}).encode()
    rq = urllib.request.Request("http://localhost:11434/api/embeddings", data=body,
                                headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(rq, timeout=60) as r:
        return json.load(r)["embedding"]

if os.path.exists(CACHE):
    V = json.load(open(CACHE))
else:
    V = {}
    for p, texts in groups.items():
        V[p] = [embed(t) for t in texts[:12]]   # cap per predicate for cost
        print(f"  embedded {p}: {len(V[p])}", file=sys.stderr)
    json.dump(V, open(CACHE, "w"))

def cos(a, b):
    d = sum(x*y for x, y in zip(a, b))
    na = math.sqrt(sum(x*x for x in a)); nb = math.sqrt(sum(x*x for x in b))
    return d/(na*nb) if na and nb else 0.0

preds = [p for p in V if len(V[p]) >= 3]
within = {}
for p in preds:
    vs = V[p]
    pairs = [cos(vs[i], vs[j]) for i in range(len(vs)) for j in range(i+1, len(vs))]
    within[p] = st.mean(pairs)
between = []
for i, a in enumerate(preds):
    for b in preds[i+1:]:
        between.append(st.mean(cos(x, y) for x in V[a] for y in V[b]))

print(f"{'predicate':22}{'n':>4}{'within-cohesion':>18}")
print("-"*46)
for p in sorted(preds, key=lambda p: -within[p]):
    print(f"{p:22}{len(V[p]):>4}{within[p]:>18.3f}")
w, b = st.mean(within.values()), st.mean(between)
print()
print(f"  mean WITHIN-predicate cohesion : {w:.3f}")
print(f"  mean BETWEEN-predicate similarity: {b:.3f}")
print(f"  separation (within - between)   : {w-b:+.3f}")
