#!/usr/bin/env python3
"""Render the predicate-collapse chart from measured A/B data."""
import html, json

d = json.load(open("results/predicate_collapse.json"))
B, LOST = d["buckets"], d["lost"]
ORDER = [("control", "control<br><span>no ontology</span>"),
         ("run1", "run 1<br><span>vocabulary unenforced</span>"),
         ("run2", "run 2<br><span>enforcement on</span>"),
         ("run3", "run 3<br><span>+ aliases</span>")]
CLS = [("declared", "Declared predicate", "s1"),
       ("nuance",   "Undeclared, carries meaning", "s2"),
       ("escape",   "associated_with / related_to", "s3"),
       ("junk",     "Undeclared, noise", "s4")]
MAX = max(sum(B[k][c] for c, _, _ in CLS) for k, _ in ORDER)

bars = []
for key, label in ORDER:
    b = B[key]
    total = sum(b[c] for c, _, _ in CLS)
    segs = []
    for c, name, cl in CLS:
        v = b[c]
        if not v:
            continue
        segs.append(
            f'<div class="seg {cl}" style="flex:{v}" '
            f'title="{html.escape(name)}: {v} edges">'
            + (f'<span class="segv">{v}</span>' if v >= 12 else "")
            + "</div>")
    bars.append(
        f'<div class="row"><div class="rowlab">{label}</div>'
        f'<div class="track" style="width:{100*total/MAX:.1f}%">{"".join(segs)}</div>'
        f'<div class="rowtot">{total}<span class="ty">{b["types"]} types</span></div></div>')

# The lost predicates themselves — the point of the whole chart.
lost_items = "".join(
    f'<li><code>{html.escape(p)}</code>'
    + (f'<span class="n">{c}</span>' if c > 1 else "")
    + "</li>" for p, c in LOST)

tpl = open("collapse_template.html").read()
open("predicate_collapse.html", "w").write(
    tpl.replace("<!--BARS-->", "\n".join(bars))
       .replace("<!--LOST-->", lost_items)
       .replace("{{NLOST}}", str(len(LOST)))
       .replace("{{ELOST}}", str(sum(c for _, c in LOST)))
       .replace("{{PCT}}", f'{100*sum(c for _,c in LOST)/(sum(c for _,c in LOST)+B["run1"]["junk"]):.0f}'))
print("predicate_collapse.html")
