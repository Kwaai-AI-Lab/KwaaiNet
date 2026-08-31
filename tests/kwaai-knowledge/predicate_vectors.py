#!/usr/bin/env python3
"""Are predicates better expressed as vectors than as symbols?

Entities have identity — two names for one person resolve to one node. Predicates
do not work that way: `nursed_by` and `cared_for` are not the same edge under two
names, they are neighbouring points in a space of meanings. Coercing the first
into `associated_with` throws that away.

This embeds every predicate D6 declares and every one the extractor emitted but
the ontology rejected, then asks what a vector would have done with each.
"""
import json, os, sys, urllib.request, math, yaml

WT = ("/private/tmp/claude-501/-Users-rezarassool-Source-KwaaiNet-tests-kwaai-knowledge/"
      "78a22fa9-8e6c-421a-aed2-e802084d04bf/scratchpad/pr-wt")
EMBED = "http://localhost:11434/api/embeddings"
CACHE = "results/predicate_vectors.json"

def embed(text):
    body = json.dumps({"model": "nomic-embed-text", "prompt": text}).encode()
    req = urllib.request.Request(EMBED, data=body,
                                 headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=60) as r:
        return json.load(r)["embedding"]

def phrase(p):
    """A predicate reads as a relation, not an identifier. Embed it that way."""
    return p.replace("_", " ").strip()

def cos(a, b):
    d = sum(x*y for x, y in zip(a, b))
    na = math.sqrt(sum(x*x for x in a)); nb = math.sqrt(sum(x*x for x in b))
    return d / (na*nb) if na and nb else 0.0

ont = yaml.safe_load(open(f"{WT}/tests/kwaai-knowledge/ontologies/D6.yaml"))
KIN = ["parent_of","child_of","spouse_of","sibling_of","half_sibling_of","grandparent_of",
       "grandchild_of","uncle_of","aunt_of","niece_of","nephew_of","cousin_of",
       "foster_parent_of","foster_child_of"]
declared = [r["name"] for r in ont["relation_types"]] + KIN
lost = [p for p, _ in json.load(open("results/predicate_collapse.json"))["lost"]]

if os.path.exists(CACHE):
    V = json.load(open(CACHE))
else:
    V = {}
    for p in declared + lost:
        if p not in V:
            V[p] = embed(phrase(p))
            print(f"  embedded {p}", file=sys.stderr)
    json.dump(V, open(CACHE, "w"))

rows = []
for p in lost:
    sims = sorted(((cos(V[p], V[d]), d) for d in declared), reverse=True)
    rows.append({"predicate": p, "best": sims[0][1], "sim": sims[0][0],
                 "second": sims[1][1], "sim2": sims[1][0]})
rows.sort(key=lambda r: -r["sim"])
json.dump({"declared": declared, "rows": rows}, open("results/predicate_nn.json", "w"), indent=1)

print(f"{len(lost)} rejected predicates vs {len(declared)} declared\n")
print(f"{'rejected':26}{'nearest declared':24}{'cos':>7}   {'2nd':<20}")
print("-"*82)
for r in rows:
    print(f"{r['predicate']:26}{r['best']:24}{r['sim']:>7.3f}   {r['second']:<20}")
