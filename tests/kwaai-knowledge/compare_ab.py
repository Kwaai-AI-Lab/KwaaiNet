#!/usr/bin/env python3
"""Compare two KB graphs on the instrument-independent metrics from the test plan.

Reports the five predictions recorded in D6-OntologyAB-testplan.md §4, so the
run can disconfirm them rather than merely describe itself.

Usage: compare_ab.py CONTROL_KB ONTOLOGY_KB
"""
import json, os, sqlite3, sys, yaml
from collections import Counter

CFG = yaml.safe_load(open(os.path.expanduser("~/.kwaainet/config.yaml")))["rag_kbs"]
ESCAPE = {"associated_with", "related_to"}

# An edge is UNINFORMATIVE if it either names an escape-hatch predicate or sits
# on a predicate no ontology declares. The second half matters: run 1 scored a
# 5.2% escape-hatch rate while carrying 98 edges on invented one-off predicates
# (`was_defenestrated_at`, `gazed_at`). Counting only the literal string
# "associated_with" rewarded a model for inventing a name instead of admitting
# it had nothing to say — measured honestly, that arm was WORSE than its control
# (55.7% vs 52.7%). Any escape-hatch metric applied to an unenforced vocabulary
# is measuring naming, not meaning.
def declared_predicates():
    import yaml, glob
    names = set()
    for f in glob.glob(os.path.join(os.path.dirname(os.path.abspath(__file__)),
                                    "ontologies", "*.yaml")):
        d = yaml.safe_load(open(f)) or {}
        for r in (d.get("relation_types") or []):
            names.add(r["name"])
    # genealogy module, inherited via `extends`
    names |= KIN_MODULE
    return names

KIN_MODULE = {"parent_of","child_of","spouse_of","sibling_of","half_sibling_of",
              "grandparent_of","grandchild_of","uncle_of","aunt_of","niece_of",
              "nephew_of","cousin_of","foster_parent_of","foster_child_of"}
KIN = {"parent_of","child_of","spouse_of","sibling_of","half_sibling_of","grandparent_of",
       "grandchild_of","uncle_of","aunt_of","niece_of","nephew_of","cousin_of",
       "foster_parent_of","foster_child_of"}

def graph(kb):
    e = CFG[kb]; d = e.get("rag_data_dir") or os.path.expanduser(f"~/.kwaainet/rag/{kb}")
    c = sqlite3.connect(f"file:{d}/graph-{e['tenant_id']}.db?mode=ro", uri=True)
    ents = Counter(); rels = Counter(); names = {}
    for (v,) in c.execute("SELECT CAST(value AS TEXT) FROM entities"):
        j = json.loads(v); ents[j["entity_type"]] += 1
        names.setdefault(j["entity_type"], []).append(j["name"])
    for (v,) in c.execute("SELECT CAST(value AS TEXT) FROM relations"):
        rels[json.loads(v)["relation_type"]] += 1
    c.close()
    return ents, rels, names

def summarise(ents, rels):
    ne, nr = sum(ents.values()), sum(rels.values())
    hatch = sum(n for t, n in rels.items() if t in ESCAPE)
    kin = sum(n for t, n in rels.items() if t in KIN)
    decl = declared_predicates()
    undeclared = sum(n for t, n in rels.items() if t not in decl and t not in ESCAPE)
    return dict(entities=ne, relations=nr,
                density=nr/ne if ne else 0.0,
                escape=hatch/nr if nr else None,
                undeclared=undeclared/nr if nr else None,
                uninformative=(hatch+undeclared)/nr if nr else None,
                kinship=kin/nr if nr else None,
                kin_n=kin, ent_types=len(ents), rel_types=len(rels))

def pct(x): return "—" if x is None else f"{x*100:.1f}%"

def main():
    ctl_kb, ont_kb = sys.argv[1], sys.argv[2]
    ce, cr, cn = graph(ctl_kb); oe, orl, on = graph(ont_kb)
    c, o = summarise(ce, cr), summarise(oe, orl)

    print(f"\n{'metric':22}{ctl_kb:>16}{ont_kb:>16}   delta")
    print("-"*72)
    for k, fmt in [("entities",str),("relations",str),("ent_types",str),("rel_types",str)]:
        d = o[k]-c[k]
        print(f"{k:22}{c[k]:>16}{o[k]:>16}   {d:+d}")
    print(f"{'density (rel/ent)':22}{c['density']:>16.4f}{o['density']:>16.4f}   {o['density']-c['density']:+.4f}")
    for k in ("escape","undeclared","uninformative","kinship"):
        d = "—" if (c[k] is None or o[k] is None) else f"{(o[k]-c[k])*100:+.1f}pp"
        print(f"{k+' rate':22}{pct(c[k]):>16}{pct(o[k]):>16}   {d}")

    print("\n── predictions (testplan §4)")
    new_types = set(oe) - set(ce)
    P1 = "Address" in oe
    print(f"  1 Address populates & splits from Place  : "
          f"{'PASS' if P1 else 'FAIL'}  (Address={oe.get('Address',0)}, Place={oe.get('Place',0)})")
    P2 = oe.get("Doctrine", 0) > 0
    print(f"  2 Doctrine populates                     : "
          f"{'PASS' if P2 else 'FAIL'}  ({oe.get('Doctrine',0)})")
    P3 = (o["uninformative"] is not None and c["uninformative"] is not None
          and o["uninformative"] < c["uninformative"])
    print(f"  3 uninformative edges fall               : "
          f"{'PASS' if P3 else 'FAIL'}  {pct(c['uninformative'])} -> {pct(o['uninformative'])}"
          f"   (naive escape-hatch: {pct(c['escape'])} -> {pct(o['escape'])})")
    P4 = o["density"] >= c["density"] * 0.7
    print(f"  4 density does not collapse (>=70% ctl)  : "
          f"{'PASS' if P4 else 'FAIL'}  {c['density']:.4f} -> {o['density']:.4f}")
    P5 = o["kin_n"] >= c["kin_n"] * 0.8
    print(f"  5 kinship unharmed (>=80% of control)    : "
          f"{'PASS' if P5 else 'FAIL'}  {c['kin_n']} -> {o['kin_n']} edges")

    print("\n── entity types only the ontology arm produced")
    for t in sorted(new_types, key=lambda t: -oe[t]):
        ex = ", ".join(on[t][:4])
        print(f"    {t:24}{oe[t]:>5}   e.g. {ex[:60]}")
    print("\n── predicates only the ontology arm produced")
    for t in sorted(set(orl)-set(cr), key=lambda t: -orl[t]):
        print(f"    {t:24}{orl[t]:>5}")
    lost = {t: cr[t] for t in set(cr)-set(orl)}
    if lost:
        print("\n── predicates the ontology arm stopped producing")
        for t, n in sorted(lost.items(), key=lambda x: -x[1]):
            print(f"    {t:24}{n:>5}")
    json.dump({"control": c, "ontology": o,
               "control_entity_types": dict(ce), "ontology_entity_types": dict(oe),
               "control_relation_types": dict(cr), "ontology_relation_types": dict(orl),
               "predictions": {"address": P1, "doctrine": P2, "escape_falls": P3,
                               "density_held": P4, "kinship_held": P5}},
              open(f"results/ab_compare_{ctl_kb}_{ont_kb}.json","w"), indent=2)

if __name__ == "__main__":
    main()
