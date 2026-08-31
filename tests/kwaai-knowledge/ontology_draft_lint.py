#!/usr/bin/env python3
"""Deterministic quality guards over induced ontology drafts (arm C).

Grounding-by-quotation turned out to be necessary but NOT sufficient: Climate's
draft passed grounding while collapsing 7 of 9 predicates onto one range,
duplicating `is_related_to`, and reusing a single quote to justify three
different predicates. A real quote attached to a predicate it does not
instantiate still grounds.

These are the checks that catch that, and they are all thresholds rather than
LLM judgement — per the parent doc's deterministic-control-loop/LLM split.

Reads ontology_drafts/*_induced.json, writes results/ontology_draft_lint_*.md
"""
import glob, json, os, datetime
from collections import Counter

HERE = os.path.dirname(os.path.abspath(__file__))
DRAFTS = os.path.join(HERE, "ontology_drafts")
RESULTS = os.path.join(HERE, "results")

# Predicates that assert association without saying what kind. Inducing these
# recreates the exact problem the ontology work exists to remove.
VAGUE = {
    "is_related_to", "related_to", "associated_with", "is_associated_with",
    "has_impact_on", "impacts", "affects", "is_influencing", "influences",
    "is_influenced_by", "has_effect", "relates_to", "connected_to",
    "has_relationship_with", "is_connected_to", "involves",
}

def lint(d):
    rels = d["kept"]["relation_types"]
    ents = d["kept"]["entity_types"]
    names = [r["name"] for r in rels]
    ranges = Counter(tuple(r.get("range") or []) for r in rels)
    ev = Counter((r.get("evidence") or "")[:80] for r in rels)

    dupes = [n for n, c in Counter(names).items() if c > 1]
    vague = [n for n in names if n.lower() in VAGUE]
    reused = [q for q, c in ev.items() if c > 1 and q]
    top_range, top_n = (ranges.most_common(1) or [((), 0)])[0]
    collapse = top_n / len(rels) if rels else 0.0

    # Each failure mode costs a quarter of the score; floor at zero.
    penalties = {
        "duplicate_predicates": len(dupes),
        "vague_predicates": len(vague),
        "reused_evidence": len(reused),
        "range_collapse": round(collapse, 2),
    }
    # Sufficiency first. An empty draft is a FAILED induction, not a clean one:
    # the first version of this linter scored Legal's 0-entity 0-relation draft
    # a perfect 1.0 because it had nothing to find fault with.
    kept_rate = (len(ents) + len(rels)) / max(
        1, d["proposed_entity_types"] + d["proposed_relation_types"])
    if not rels or len(ents) < 3 or len(rels) < 4:
        return_score = 0.0
    else:
        return_score = 1.0
        return_score -= 0.25 * len(dupes) / len(rels)
        return_score -= 0.25 * len(vague) / len(rels)
        return_score -= 0.25 * len(reused) / len(rels)
        return_score -= 0.25 * max(0.0, (collapse - 0.34) / 0.66)
        # A draft where most proposals failed grounding is a weak draft even if
        # the survivors are clean — the model was mostly guessing.
        return_score *= min(1.0, kept_rate / 0.7)
    score = max(0.0, round(return_score, 3))

    return {
        "kb": d["kb"],
        "domain": d.get("domain"),
        "entity_types": len(ents),
        "relation_types": len(rels),
        "grounding_kept_rate": round(kept_rate, 3),
        "sufficient": bool(rels and len(ents) >= 3 and len(rels) >= 4),
        "dupes": dupes, "vague": vague,
        "reused_evidence_count": len(reused),
        "range_collapse": round(collapse, 3),
        "usable_score": score,
        "penalties": penalties,
    }

def main():
    rows = []
    for f in sorted(glob.glob(os.path.join(DRAFTS, "*_induced.json"))):
        rows.append(lint(json.load(open(f))))
    rows.sort(key=lambda r: -r["usable_score"])
    os.makedirs(RESULTS, exist_ok=True)
    stamp = datetime.datetime.now().strftime("%Y%m%d_%H%M%S")
    out = os.path.join(RESULTS, f"ontology_draft_lint_{stamp}.md")
    with open(out, "w") as f:
        w = f.write
        w(f"# Arm C draft quality — {datetime.datetime.now(datetime.timezone.utc):%Y-%m-%dT%H:%M:%SZ}\n\n")
        w("Deterministic guards over induced drafts. Grounding-by-quotation is\n")
        w("necessary but not sufficient — a real quote attached to a predicate it\n")
        w("does not instantiate still grounds. These catch that.\n\n")
        w("`usable_score` 1.0 = no duplicate predicates, no vague predicates, no\n")
        w("reused evidence, no range collapse. Below ~0.6 the draft needs rewriting\n")
        w("rather than editing.\n\n")
        w("| kb | domain | ents | rels | grounded | dupes | vague | reused ev | range collapse | usable |\n")
        w("|---|---|---|---|---|---|---|---|---|---|\n")
        for r in rows:
            w(f"| {r['kb']} | {r['domain']} | {r['entity_types']} | {r['relation_types']} | "
              f"{r['grounding_kept_rate']*100:.0f}% | {len(r['dupes'])} | {len(r['vague'])} | "
              f"{r['reused_evidence_count']} | {r['range_collapse']*100:.0f}% | "
              f"**{r['usable_score']}** |\n")
        w("\n## Detail\n")
        for r in rows:
            w(f"\n### {r['kb']} — {r['usable_score']}\n\n")
            if r["dupes"]:
                w(f"- duplicate predicates: `{'`, `'.join(r['dupes'])}`\n")
            if r["vague"]:
                w(f"- vague predicates (the `associated_with` problem renamed): `{'`, `'.join(r['vague'])}`\n")
            if r["reused_evidence_count"]:
                w(f"- {r['reused_evidence_count']} quote(s) justifying more than one predicate\n")
            if r["range_collapse"] > 0.5:
                w(f"- {r['range_collapse']*100:.0f}% of predicates share one range — degenerate\n")
            if not (r["dupes"] or r["vague"] or r["reused_evidence_count"] or r["range_collapse"] > 0.5):
                w("- clean\n")
    with open(os.path.join(RESULTS, f"ontology_draft_lint_{stamp}.json"), "w") as f:
        json.dump(rows, f, indent=2)
    print(out)

if __name__ == "__main__":
    main()
