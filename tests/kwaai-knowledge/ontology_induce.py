#!/usr/bin/env python3
"""Arm C of the ontology bakeoff — induce a per-KB ontology draft from the corpus.

Follows the shape GraphRAG's auto prompt tuning uses (PerKBOntology-plan.md §8.4):
sample text units, infer the domain, propose entity and relation types. Adds the
grounding requirement the plan insists on (§8.4, §7.2): every proposed predicate
must cite a verbatim span from the sampled text, and ungrounded proposals are
dropped rather than trusted.

Output is a DRAFT for human ratification, never applied automatically — an LLM
asked twice gives two answers, and the ontology is a versioned artifact.

Usage: ontology_induce.py <KB> [--n 24] [--model llama3.1:8b]
"""
import argparse, glob, json, os, random, sqlite3, sys, urllib.request, datetime

CONFIG = os.path.expanduser("~/.kwaainet/config.yaml")
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "ontology_drafts")

def kb_paths(kb):
    import yaml
    cfg = yaml.safe_load(open(CONFIG)) or {}
    e = (cfg.get("rag_kbs") or {}).get(kb)
    if not e:
        sys.exit(f"KB '{kb}' not in {CONFIG}")
    d = e.get("rag_data_dir") or os.path.expanduser(f"~/.kwaainet/rag/{kb}")
    return os.path.join(d, f"{e['tenant_id']}.db"), os.path.join(d, f"graph-{e['tenant_id']}.db")

def sample_chunks(meta_db, n, seed=17):
    """Random sample, matching GraphRAG's default selection method."""
    conn = sqlite3.connect(f"file:{meta_db}?mode=ro", uri=True)
    try:
        rows = [
            (json.loads(v)["doc_name"], json.loads(v)["text"])
            for (v,) in conn.execute("SELECT CAST(value AS TEXT) FROM chunks")
        ]
    finally:
        conn.close()
    rows = [r for r in rows if len(r[1].strip()) > 200]
    random.Random(seed).shuffle(rows)
    return rows[:n]

PROMPT = """You are an ontology engineer. Below are {n} excerpts sampled at random from a single document corpus.

Your job is to propose the knowledge schema that this corpus actually needs — the entity types and relation types that carry THIS domain's structure of meaning. Do not propose a generic schema. A climate science corpus has no use for kinship predicates; a memoir has no use for `measured_by`.

STRICT GROUNDING RULE: every entity type and every relation type you propose MUST be justified by a verbatim quotation from the excerpts below. If you cannot quote text that instantiates it, do not propose it. A schema element that cannot be instantiated is worthless.

Return ONLY valid JSON, no markdown fences:
{{
  "domain": "a short name for this corpus's domain",
  "entity_types": [
    {{"name":"...","description":"...","evidence":"verbatim quote from the excerpts"}}
  ],
  "relation_types": [
    {{"name":"snake_case_predicate","domain":["SubjectType"],"range":["ObjectType"],"evidence":"verbatim quote showing this relation holding"}}
  ]
}}

Propose at most 8 entity types and at most 12 relation types. Prefer few, well-grounded types over many speculative ones.

EXCERPTS:
{excerpts}
"""

def ask(url, model, prompt):
    # /api/chat with explicit num_ctx — /v1/chat/completions ignores options and
    # Ollama's 131072 default splits the model 65/35 across CPU/GPU.
    body = json.dumps({
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
        "stream": False,
        "options": {"num_ctx": 8192, "temperature": 0.2},
    }).encode()
    req = urllib.request.Request(f"{url}/api/chat", data=body,
                                 headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=900) as r:
        return json.load(r)["message"]["content"]

def parse(raw):
    t = raw.strip()
    if t.startswith("```"):
        t = t.split("```")[1]
        t = t[4:] if t.startswith("json") else t
    i, j = t.find("{"), t.rfind("}")
    return json.loads(t[i:j + 1])

import re

def _norm(s):
    """Lowercase, strip punctuation, collapse whitespace.

    Needed because the model reproduces quotes with different punctuation and
    unicode than the source ("1.0°C", curly quotes, hyphen variants). Without
    this the grounding check produces false negatives and silently discards
    correctly-grounded proposals — verified against Climate's GHG quote.
    """
    return " ".join(re.sub(r"[^a-z0-9 ]", " ", s.lower()).split())

def verify_grounding(proposal, excerpts, shingle=8):
    """Drop any proposal whose evidence is not actually in the sampled text.

    The plan's stated mitigation for induction's failure mode: an LLM will
    happily invent a plausible vocabulary the corpus does not support. Matching
    is on normalised 8-word shingles, so a real quote carrying an appended
    citation still grounds, while a paraphrase ("permafrost thaw is related to
    climate change") does not.
    """
    hay = _norm(" ".join(t for _, t in excerpts))
    def ok(item):
        w = _norm(item.get("evidence") or "").split()
        if len(w) < 5:
            return False
        if len(w) <= shingle:
            return " ".join(w) in hay
        return any(" ".join(w[i:i + shingle]) in hay
                   for i in range(len(w) - shingle + 1))
    kept_e = [e for e in proposal.get("entity_types", []) if ok(e)]
    kept_r = [r for r in proposal.get("relation_types", []) if ok(r)]
    return kept_e, kept_r

def to_yaml(kb, domain, ents, rels):
    L = [f"# Induced ontology DRAFT for KB '{kb}' — arm C, NOT RATIFIED.",
         f"# Generated {datetime.datetime.now(datetime.timezone.utc):%Y-%m-%dT%H:%M:%SZ} by ontology_induce.py",
         "# Every element below cited a verbatim span from the sampled corpus.",
         "# Review before `kwaainet rag graph schema set`.", "",
         "ontology:", f"  name: {domain}", "  version: 1", "  extends: none", "",
         "entity_types:"]
    for e in ents:
        L += [f"  - name: {e['name']}",
              f"    description: >-",
              f"      {e.get('description','').strip()}",
              f"    irreducible: false",
              f"    # evidence: {json.dumps(e.get('evidence','')[:160])}"]
    L += ["", "relation_types:"]
    for r in rels:
        L += [f"  - name: {r['name']}",
              f"    domain: [{', '.join(r.get('domain') or [])}]",
              f"    range:  [{', '.join(r.get('range') or [])}]",
              f"    # evidence: {json.dumps(r.get('evidence','')[:160])}"]
    return "\n".join(L) + "\n"

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("kb")
    ap.add_argument("--n", type=int, default=24)
    ap.add_argument("--model", default="llama3.1:8b")
    ap.add_argument("--url", default="http://localhost:11434")
    a = ap.parse_args()

    meta_db, _ = kb_paths(a.kb)
    ex = sample_chunks(meta_db, a.n)
    if not ex:
        sys.exit(f"no chunks for {a.kb}")
    blob = "\n\n---\n\n".join(f"[{d}]\n{t[:1200]}" for d, t in ex)
    raw = ask(a.url, a.model, PROMPT.format(n=len(ex), excerpts=blob))

    try:
        prop = parse(raw)
    except Exception as e:
        sys.exit(f"unparseable response for {a.kb}: {e}\n{raw[:600]}")

    ents, rels = verify_grounding(prop, ex)

    # Never clobber a good draft with a failed run. A sweep overwrote Legal's
    # 9/9-entity 12/12-relation draft with an empty 0/9 0/14 one, destroying it;
    # induction is non-deterministic (see the Climate double-run), so a fixed
    # output filename is a data-loss bug, not a convenience.
    if not ents and not rels:
        sys.exit(f"{a.kb}: induction produced nothing grounded — refusing to write "
                 f"(proposed {len(prop.get('entity_types',[]))} ents / "
                 f"{len(prop.get('relation_types',[]))} rels, none verifiable)")

    os.makedirs(OUT, exist_ok=True)
    stamp = datetime.datetime.now().strftime("%Y%m%d_%H%M%S")
    base = os.path.join(OUT, f"{a.kb}_induced_{stamp}")
    latest = os.path.join(OUT, f"{a.kb}_induced")
    with open(base + ".yaml", "w") as f:
        f.write(to_yaml(a.kb, prop.get("domain", a.kb.lower()), ents, rels))
    with open(base + ".json", "w") as f:
        json.dump({"kb": a.kb, "model": a.model, "n_chunks": len(ex),
                   "domain": prop.get("domain"),
                   "proposed_entity_types": len(prop.get("entity_types", [])),
                   "proposed_relation_types": len(prop.get("relation_types", [])),
                   "grounded_entity_types": len(ents),
                   "grounded_relation_types": len(rels),
                   "raw": prop, "kept": {"entity_types": ents, "relation_types": rels}},
                  f, indent=2)
    # `<KB>_induced.*` always points at the most recent successful run; the
    # timestamped file is the durable record.
    for ext in (".yaml", ".json"):
        if os.path.islink(latest + ext) or os.path.exists(latest + ext):
            os.remove(latest + ext)
        os.symlink(os.path.basename(base + ext), latest + ext)
    print(f"{a.kb}: domain={prop.get('domain')!r} "
          f"entities {len(ents)}/{len(prop.get('entity_types',[]))} grounded, "
          f"relations {len(rels)}/{len(prop.get('relation_types',[]))} grounded "
          f"-> {base}.yaml")

if __name__ == "__main__":
    main()
