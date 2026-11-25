#!/usr/bin/env python3
"""Method data-flow diagram for the AIAS+ forgetting abstract, from one SVG template.

    python3 gen_method_diagram.py

Writes beside this script:
  fig0_method.svg       paper version: literal ink on white (rendered to .png/.pdf by headless Chrome)
  fig0_method_web.svg   webpage version: ink is currentColor, the accent a CSS variable, so it
                        follows the page's light/dark theme
The accent marks the one mechanism the paper is about: what the forgetting rule withholds.
"""
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
W, H = 1030, 456
BW, BH = 160, 64  # box size; columns at x = 20, 220, 460, 680, 880; rows at y = 50, 200, 350


def box(x, y, title, sub, w=BW, accent=False):
    stroke = "ACCENT" if accent else "INK"
    return (f'<rect x="{x}" y="{y}" width="{w}" height="{BH}" rx="8" style="fill:SURFACE;stroke:{stroke}" '
            f'stroke-width="{1.6 if accent else 1.2}"/>'
            f'<text x="{x + w / 2}" y="{y + 27}" text-anchor="middle" font-size="14" font-weight="600" '
            f'style="fill:INK">{title}</text>'
            f'<text x="{x + w / 2}" y="{y + 47}" text-anchor="middle" font-size="11.5" style="fill:INK2">{sub}</text>')


def path(d, accent=False, dashed=False):
    c = "ACCENT" if accent else "INK"
    m = "url(#arrA)" if accent else "url(#arr)"
    dash = ' stroke-dasharray="5 4"' if dashed else ""
    return (f'<path d="{d}" style="fill:none;stroke:{c}" stroke-width="{1.8 if accent else 1.2}"{dash} '
            f'marker-end="{m}"/>')


def label(x, y, text, anchor="middle", accent=False, size=11.5):
    c = "ACCENT" if accent else "INK2"
    return f'<text x="{x}" y="{y}" text-anchor="{anchor}" font-size="{size}" style="fill:{c}">{text}</text>'


parts = [
    # memory = the searchable text of both stores
    '<rect x="210" y="38" width="180" height="406" rx="12" style="fill:none;stroke:INK2" stroke-width="1" '
    'stroke-dasharray="2 4"/>',
    label(300, 438, "memory = searchable characters", size=11),
    box(20, 50, "Documents", "first 10% of each", w=140),
    box(220, 50, "Chunk store", "short-term · BM25 + dense"),
    box(220, 350, "Entity graph", "long-term · text cards"),
    box(460, 50, "Forgetting rule", "chunk · probe · entity", accent=True),
    box(460, 200, "Dream cycle", "40k tokens · ×16"),
    box(680, 50, "Retrieve", "20 cards + 10 chunks"),
    box(680, 200, "Answer", "llama3.1:8b · temp 0"),
    box(680, 350, "Recall", "NLI vs gold facts"),
    box(880, 200, "Forgetting curve", "memory ↓ vs recall ↓", w=130),
    # encoding
    path("M160,82 L216,82"), label(188, 74, "chunk"),
    path("M250,114 L250,346"), label(258, 226, "cycle 0:", anchor="start"),
    label(258, 241, "extract", anchor="start"),
    # sleep: evidence in, cards out, marks to the rule, the rule forgets
    path("M330,114 L330,232 L456,232"), label(384, 225, "evidence", anchor="end"),
    label(452, 250, "F: minus", anchor="end", accent=True, size=10.5),
    label(452, 264, "forgotten", anchor="end", accent=True, size=10.5),
    path("M540,264 L540,382 L384,382"), label(462, 375, "rewrites cards"),
    path("M540,200 L540,118"), label(548, 162, "chunks read", anchor="start"),
    path("M460,82 L384,82", accent=True), label(424, 74, "forgets", accent=True, size=10.5),
    # wake: what each snapshot answers from
    path("M300,50 L300,24 L760,24 L760,46"), label(580, 18, "remembered chunks"),
    path("M380,398 L660,398 L660,82 L676,82"), label(625, 392, "cards"),
    path("M760,114 L760,196"), label(768, 160, "prompt", anchor="start"),
    path("M760,264 L760,346"), label(768, 310, "answer", anchor="start"),
    path("M840,382 L945,382 L945,268"), label(892, 375, "per question"),
    label(937, 300, "at c1, 2, 4, 8, 16", anchor="end", size=10.5),
]
body = "".join(parts)
defs = ('<defs>'
        '<marker id="arr" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto">'
        '<path d="M0,0 L10,5 L0,10 z" style="fill:INK"/></marker>'
        '<marker id="arrA" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto">'
        '<path d="M0,0 L10,5 L0,10 z" style="fill:ACCENT"/></marker></defs>')
ARIA = ("Method data flow: documents are chunked into a short-term chunk store; cycle 0 extracts an entity "
        "graph of text cards; each dream cycle reads evidence chunks and rewrites cards; the forgetting rule "
        "turns chunks read into forgotten (withheld) chunks; each snapshot answers questions from 20 cards and 10 "
        "remembered chunks, scored for recall against gold facts, and plotted against memory as a "
        "forgetting curve.")


def svg(ink, ink2, surface, accent, extra=""):
    s = (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" role="img" aria-label="{ARIA}" '
         f'font-family="Inter, Helvetica, Arial, sans-serif"{extra}>{defs}{body}</svg>')
    return s.replace("ACCENT", accent).replace("SURFACE", surface).replace("INK2", ink2).replace("INK", ink)


paper = svg("#16152b", "#4c4a66", "#ffffff", "#c2541f")
(HERE / "fig0_method.svg").write_text(paper)
web = svg("currentColor", "var(--ink-2)", "var(--surface)", "var(--cliff)",
          extra=' style="max-width:100%;height:auto;display:block"')
(HERE / "fig0_method_web.svg").write_text(web)

chrome = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
html = HERE / "_fig0_render.html"
html.write_text(f'<html><body style="margin:0;background:#fff">'
                f'<div style="width:{W}px;height:{H}px">{paper}</div></body></html>')
common = [chrome, "--headless=new", "--disable-gpu", "--hide-scrollbars", "--no-pdf-header-footer"]
subprocess.run(common + [f"--window-size={W},{H}", "--force-device-scale-factor=3",
                         f"--screenshot={HERE / 'fig0_method.png'}", html.as_uri()],
               check=True, capture_output=True)
html.write_text(f'<html><head><style>@page{{size:{W}px {H}px;margin:0}}</style></head>'
                f'<body style="margin:0">{paper}</body></html>')
subprocess.run(common + [f"--print-to-pdf={HERE / 'fig0_method.pdf'}", html.as_uri()],
               check=True, capture_output=True)
html.unlink()
print("wrote fig0_method.{svg,png,pdf} and fig0_method_web.svg", file=sys.stderr)
