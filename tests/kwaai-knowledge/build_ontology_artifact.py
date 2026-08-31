#!/usr/bin/env python3
"""Build the ontology diagram reference page from ontologies/*.yaml."""
import glob, html, json, os, yaml
import importlib.util
spec = importlib.util.spec_from_file_location("om", "ontology_mermaid.py")
om = importlib.util.module_from_spec(spec); spec.loader.exec_module(om)

HERE = os.path.dirname(os.path.abspath(__file__))
hyg, base = om.load_metrics()

MODULES = {"narrative-history", "scholarly", "software-docs"}
docs = {}
for f in sorted(glob.glob(os.path.join(HERE, "ontologies", "*.yaml"))):
    n = os.path.basename(f)[:-5]
    key = n.replace("_module_", "").replace("_", "-") if n.startswith("_module_") else n
    docs[key] = yaml.safe_load(open(f))

def esc(s): return html.escape(str(s))

# Group: modules first, then the KBs that extend each, then standalone KBs.
order, children = [], {m: [] for m in MODULES}
standalone = []
for k, d in docs.items():
    if k in MODULES: continue
    p = (d.get("ontology") or {}).get("extends")
    (children[p] if p in children else standalone).append(k)
def _rank(k):
    dv = (docs[k].get("provenance") or {}).get("derivation")
    # derived schemas first; D6 leads as the only one built from a full reading
    return (0 if k == "D6" else 1 if dv == "derived" else 2, k)
# Standalone schemas lead, D6 first: they are the result. The genre modules are
# supporting material and go last — putting them first buried the only fully
# derived schema thirteen cards down the page.
for k in sorted(standalone, key=_rank):
    order.append((k, docs[k], []))
for m in ["narrative-history", "scholarly", "software-docs"]:
    order.append((m, docs[m], sorted(children[m])))
    for k in sorted(children[m]):
        order.append((k, docs[k], []))

def resolved(key):
    """Effective ontology: the parent module's types merged with this KB's deltas.

    A child file like WarPeace.yaml declares one entity type and one predicate;
    the ontology it actually applies is narrative-history plus that delta. The
    diagram should show what runs, not what the file happens to spell out.
    """
    d = docs[key]
    p = (d.get("ontology") or {}).get("extends")
    if p not in docs:
        return d, []
    par = docs[p]
    own_e = {e["name"] for e in (d.get("entity_types") or [])}
    own_r = {r["name"] for r in (d.get("relation_types") or [])}
    merged = dict(d)
    merged["entity_types"] = (par.get("entity_types") or []) + (d.get("entity_types") or [])
    merged["relation_types"] = (par.get("relation_types") or []) + (d.get("relation_types") or [])
    merged["streams"] = d.get("streams") or par.get("streams") or []
    if d.get("fallback_predicate", "unset") == "unset":
        merged["fallback_predicate"] = par.get("fallback_predicate")
    return merged, sorted(own_e | own_r)

def card(key, d, kids):
    o = (d.get("ontology") or {})
    eff, delta = resolved(key)
    ents = eff.get("entity_types") or []
    rels = eff.get("relation_types") or []
    streams = eff.get("streams") or []
    d = eff
    kind, label, detail = om.status_for(key, hyg, base)
    if key in MODULES:
        kind, label = "module", "shared module"
        detail = "extended by " + ", ".join(kids) if kids else ""
    prov = (d.get("provenance") or {})
    ax = d.get("axioms") or []
    fb = d.get("fallback_predicate", "unset")
    irr = [e["name"] for e in ents if e.get("irreducible")]

    stats = [(len(ents), "entity types" + (" (incl. inherited)" if delta else "")),
             (len(rels), "predicates")]
    if streams: stats.append((len(streams), "streams"))
    if ax: stats.append((sum(len(a.get('pairs') or a.get('relations') or []) for a in ax), "axioms"))

    s = [f'<section class="ont" id="{esc(key)}">']
    s.append('<header class="ont-head">')
    s.append(f'<div class="ont-id"><h2>{esc(key)}</h2>'
             f'<code class="ont-name">{esc(o.get("name",""))}</code></div>')
    s.append('<div class="chips">')
    dv = prov.get("derivation")
    if dv == "derived":
        s.append('<span class="chip chip-derived">derived from corpus</span>')
    elif dv == "unverified":
        s.append('<span class="chip chip-unverified">unverified</span>')
    s.append(f'<span class="chip chip-{kind}">{esc(label)}</span>')
    s.append('</div></header>')
    if prov.get("note"):
        s.append(f'<p class="prov">{esc(prov["note"])}</p>')
    if detail:
        s.append(f'<p class="ont-detail">{esc(detail)}</p>')
    s.append('<dl class="stats">')
    for v, l in stats:
        s.append(f'<div><dt>{l}</dt><dd>{v}</dd></div>')
    s.append('</dl>')

    meta = []
    if o.get("extends") and o["extends"] != "none":
        meta.append(f'extends <code>{esc(o["extends"])}</code>')
    if streams:
        meta.append("streams: " + ", ".join(
            f'<code>{esc(x["name"])}</code>' + (' <span class="skip">skip</span>'
            if x.get("ingest") == "skip" else '') for x in streams))
    if irr:
        meta.append("irreducible: " + ", ".join(f'<code>{esc(i)}</code>' for i in irr))
    if delta:
        meta.append("adds to its module: " + ", ".join(
            f'<code>{esc(x)}</code>' for x in delta))
    meta.append("fallback predicate: " + (f'<code>{esc(fb)}</code>' if fb else
                '<span class="none">none — a vague edge is a bug here</span>'))
    s.append('<ul class="meta">' + "".join(f'<li>{m}</li>' for m in meta) + '</ul>')

    ev = [(e["name"], e["evidence"]) for e in ents if e.get("evidence") is not None]
    if ev:
        ev.sort(key=lambda x: -x[1])
        top = ev[0][1]
        s.append('<div class="evidence"><h3>Corpus evidence '
                 '<span>share of chunks carrying each type\u2019s vocabulary</span></h3><ul>')
        for n, v in ev:
            s.append(f'<li><span class="en">{esc(n)}</span>'
                     f'<span class="bar"><i style="width:{max(2, v/top*100):.0f}%"></i></span>'
                     f'<span class="ev">{v:.1f}%</span></li>')
        s.append('</ul></div>')

    rej = d.get("rejected_types") or []
    if rej:
        s.append('<div class="rejected"><h3>Considered and rejected</h3><ul>')
        for r in rej:
            s.append(f'<li><span class="en">{esc(r["name"])}</span>'
                     f'<span class="ev">{r["evidence"]:.1f}%</span>'
                     f'<span class="why">{esc(r["reason"])}</span></li>')
        s.append('</ul></div>')

    if ents or rels:
        s.append('<figure class="plate"><pre class="mermaid">'
                 + esc(om.diagram(d, key)) + '</pre></figure>')
    s.append('</section>')
    return "\n".join(s)

# Children appear inside their module's nested list; emitting them again at top
# level duplicated all nine of them in the index.
child_keys = {k for _, _, kids in order for k in kids}
nav = []
prev_group = None
for key, d, kids in order:
    if key in child_keys:
        continue
    group = ("Genre modules and their corpora" if key in MODULES
             else "Corpus schemas")
    if group != prev_group:
        nav.append(f'<li class="navhead">{group}</li>')
        prev_group = group
    kind, label, _ = om.status_for(key, hyg, base)
    if key in MODULES: kind = "module"
    unv = (d.get("provenance") or {}).get("derivation") == "unverified"
    mark = '<span class="unv" title="corpus not sampled">!</span>' if unv else ''
    nav.append(f'<li><a href="#{esc(key)}"><span class="dot dot-{kind}"></span>{esc(key)}{mark}</a>'
               + ("<ul>" + "".join(
                   f'<li><a href="#{esc(k)}"><span class="dot dot-'
                   f'{om.status_for(k, hyg, base)[0]}"></span>{esc(k)}'
                   + ('<span class="unv" title="corpus not sampled">!</span>'
                      if (docs[k].get("provenance") or {}).get("derivation") == "unverified"
                      else '') + '</a></li>'
                   for k in kids) + "</ul>" if kids else "") + '</li>')

body = "\n".join(card(k, d, kids) for k, d, kids in order)
n_ent = len({e["name"] for d in docs.values() for e in (d.get("entity_types") or [])})
n_rel = len({r["name"] for d in docs.values() for r in (d.get("relation_types") or [])})

TPL = open(os.path.join(HERE, "ontology_artifact_template.html")).read()
out = os.path.join(HERE, "ontology_diagrams.html")
open(out, "w").write(TPL.replace("<!--NAV-->", "\n".join(nav))
                        .replace("<!--BODY-->", body)
                        .replace("{{NENT}}", str(n_ent))
                        .replace("{{NREL}}", str(n_rel))
                        .replace("{{NONT}}", str(len(docs)))
                        .replace("{{NUNV}}", str(sum(
                            1 for d in docs.values()
                            if (d.get("provenance") or {}).get("derivation") == "unverified"))))
print(out)
