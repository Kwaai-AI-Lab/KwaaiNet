#!/usr/bin/env python3
"""Arm A control for the per-KB ontology bakeoff (PerKBOntology-plan.md §5.2).

Reads every built graph and reports the instrument-independent metrics the plan
names in §5.4 — escape-hatch rate, type distributions, and an ontology-coverage
proxy — without an eval run and without rebuilding anything.

Why these and not `graph score`: the scorer is hardcoded to the memoir ontology
(plan §1.5), so it cannot compare ontology arms. These read the graph directly.

Usage: ontology_baseline.py [kb ...]      (default: every KB with a graph)
"""
import json, os, sqlite3, sys, glob, datetime
from collections import Counter

RAG = os.path.expanduser("~/.kwaainet/rag")
CONFIG = os.path.expanduser("~/.kwaainet/config.yaml")
QDIR = os.path.dirname(os.path.abspath(__file__))
RESULTS = os.path.join(QDIR, "results")

# Predicates that assert an association without saying what kind. The framing
# doc's null result was caused by these crowding out everything informative.
ESCAPE_HATCH = {"associated_with", "related_to"}
KINSHIP = {
    "parent_of","child_of","spouse_of","sibling_of","half_sibling_of",
    "grandparent_of","grandchild_of","uncle_of","aunt_of","niece_of",
    "nephew_of","cousin_of","foster_parent_of","foster_child_of",
}

def load_kbs():
    """KB name -> (data_dir, tenant_id) from the CLI's own config.

    Resolving via config rather than globbing matters: several KB directories
    hold more than one graph-*.db (D6 has a stale empty one alongside the live
    graph), and globbing silently picks the wrong file.
    """
    import yaml
    cfg = yaml.safe_load(open(CONFIG)) or {}
    out = {}
    for name, e in (cfg.get("rag_kbs") or {}).items():
        tid = e.get("tenant_id")
        if not tid:
            continue
        out[name] = (e.get("rag_data_dir") or os.path.join(RAG, name), tid)
    return out

KBS = load_kbs()

def graph_db(kb):
    ent = KBS.get(kb)
    if not ent:
        return None
    d, tid = ent
    path = os.path.join(d, f"graph-{tid}.db")
    return path if os.path.exists(path) else None

def col(conn, table, path):
    """Counter over a JSON field of every row in `table`."""
    c = Counter()
    for (v,) in conn.execute(
        f"SELECT json_extract(CAST(value AS TEXT), '{path}') FROM {table}"
    ):
        c[v if v is not None else "(null)"] += 1
    return c

def analyse(kb):
    db = graph_db(kb)
    if not db:
        return None
    conn = sqlite3.connect(f"file:{db}?mode=ro", uri=True)
    try:
        ents = col(conn, "entities", "$.entity_type")
        rels = col(conn, "relations", "$.relation_type")
    finally:
        conn.close()

    n_e, n_r = sum(ents.values()), sum(rels.values())
    # Relation density is the metric that actually discriminates here: the
    # KBs whose declared entity types admit no predicates produce ~no edges.
    hatch = sum(n for t, n in rels.items() if t in ESCAPE_HATCH)
    kin = sum(n for t, n in rels.items() if t in KINSHIP)
    unknown = ents.get("Unknown", 0) + ents.get("(null)", 0)
    return {
        "kb": kb,
        "entities": n_e,
        "relations": n_r,
        # §5.4 escape-hatch rate: share of edges that assert association
        # without saying what kind. High = the ontology did not take.
        "escape_hatch_rate": round(hatch / n_r, 4) if n_r else None,
        # How much of the vocabulary actually in use is the memoir ontology.
        "kinship_rate": round(kin / n_r, 4) if n_r else None,
        # Coverage proxy: entities the type system could not place.
        "unknown_entity_rate": round(unknown / n_e, 4) if n_e else None,
        "relations_per_entity": round(n_r / n_e, 4) if n_e else None,
        "distinct_entity_types": len(ents),
        "distinct_relation_types": len(rels),
        "entity_types": dict(ents.most_common()),
        "relation_types": dict(rels.most_common()),
    }

def main():
    if len(sys.argv) > 1:
        kbs = sys.argv[1:]
    else:
        kbs = sorted(k for k in KBS if graph_db(k))
    rows = [r for r in (analyse(kb) for kb in kbs) if r]
    os.makedirs(RESULTS, exist_ok=True)
    stamp = datetime.datetime.now().strftime("%Y%m%d_%H%M%S")
    jpath = os.path.join(RESULTS, f"ontology_baseline_{stamp}.json")
    with open(jpath, "w") as f:
        json.dump({"generated": datetime.datetime.now(datetime.timezone.utc).isoformat() + "Z",
                   "kbs": rows}, f, indent=2)

    mpath = os.path.join(RESULTS, f"ontology_baseline_{stamp}.md")
    with open(mpath, "w") as f:
        w = f.write
        w(f"# Ontology bakeoff — arm A (control) — {datetime.datetime.now(datetime.timezone.utc):%Y-%m-%dT%H:%M:%SZ}\n\n")
        w("Instrument-independent metrics read straight from the built graphs\n")
        w("(`PerKBOntology-plan.md` §5.4). No eval run, no rebuild.\n\n")
        w("`graph score` is deliberately absent: it is hardcoded to the memoir\n")
        w("ontology (plan §1.5) and cannot compare arms.\n\n")
        w("| kb | entities | relations | rel/ent | escape-hatch | kinship | ent types | rel types |\n")
        w("|---|---|---|---|---|---|---|---|\n")
        for r in rows:
            pct = lambda x: "—" if x is None else f"{x*100:.1f}%"
            d = "—" if r["relations_per_entity"] is None else f"{r['relations_per_entity']:.4f}"
            w(f"| {r['kb']} | {r['entities']} | {r['relations']} | {d} | "
              f"{pct(r['escape_hatch_rate'])} | {pct(r['kinship_rate'])} | "
              f"{r['distinct_entity_types']} | "
              f"{r['distinct_relation_types']} |\n")
        w("\n## Per-KB vocabulary in use\n")
        for r in rows:
            w(f"\n### {r['kb']}\n\n")
            w(f"**Entity types ({r['distinct_entity_types']}):** ")
            w(", ".join(f"{k} {v}" for k, v in list(r["entity_types"].items())[:20]) or "(none)")
            w(f"\n\n**Relation types ({r['distinct_relation_types']}):** ")
            w(", ".join(f"{k} {v}" for k, v in list(r["relation_types"].items())[:30]) or "(none)")
            w("\n")
    print(mpath)
    print(jpath)

if __name__ == "__main__":
    main()
