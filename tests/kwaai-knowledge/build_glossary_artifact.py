#!/usr/bin/env python3
"""Render GLOSSARY.md as a filterable reference page."""
import html, re, os

SRC = "/Users/rezarassool/Source/KwaaiNet/projects/kwaai-knowledge/GLOSSARY.md"
OUT = "/Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/glossary.html"
md = open(SRC).read()

def inline(t):
    t = html.escape(t)
    t = re.sub(r"`([^`]+)`", r"<code>\1</code>", t)
    t = re.sub(r"\*\*([^*]+)\*\*", r"<strong>\1</strong>", t)
    t = re.sub(r"(?<!\*)\*([^*]+)\*(?!\*)", r"<em>\1</em>", t)
    return t

# Split on the section headings themselves. Splitting on "---" first swallowed
# section 1, because the intro is separated from it by the same rule.
sections = re.split(r"(?m)^## ", md)[1:]
out, nav = [], []
for sec in sections:
    sec = sec.strip()
    if not sec or sec.startswith("9. Reading order"):
        if sec.startswith("9. Reading order"):
            title = "Reading order"
            sid = "reading-order"
            nav.append((sid, "9", title, 0))
            rest9 = sec.split("\n", 1)[1] if "\n" in sec else ""
            # List items wrap across lines; capture until the next number or a
            # blank line, or the trailing prose gets truncated mid-sentence.
            items = [" ".join(m.group(1).split())
                     for m in re.finditer(r"^\d+\.\s+(.+?)(?=\n\d+\.\s|\n\n|\Z)",
                                          rest9, re.M | re.S)]
            # Prose after the list is guidance, not an item — keep it.
            tail = re.split(r"(?m)^\d+\.\s.*?(?=\n\n(?!\s))", rest9, flags=re.S)[-1]
            tailp = [" ".join(t.split()) for t in tail.strip().split("\n\n")
                     if t.strip() and not re.match(r"^\d+\.", t.strip())]
            lead = [" ".join(t.split()) for t in rest9.split("\n\n")
                    if t.strip() and not re.match(r"^\d+\.", t.strip())][:1]
            out.append(f'<section id="{sid}"><h2><span class="num">9</span>{title}</h2>'
                       + "".join(f'<p class="pre">{inline(l)}</p>' for l in lead)
                       + "<ol class='reading'>"
                       + "".join(f"<li>{inline(i)}</li>" for i in items) + "</ol>"
                       + "".join(f'<p class="note">{inline(t)}</p>'
                                 for t in tailp if t not in lead) + "</section>")
        continue
    m = re.match(r"^(\d+)\.\s+(.+)", sec)
    if not m:
        continue
    num, title = m.group(1), m.group(2).split("\n")[0].strip()
    sid = re.sub(r"[^a-z0-9]+", "-", title.lower()).strip("-")
    rest = sec[len(m.group(0)):]

    # A leading paragraph before the first term is section preamble.
    pre = []
    for para in rest.split("\n\n"):
        if para.strip().startswith("**"):
            break
        if para.strip() and not para.strip().startswith(">"):
            pre.append(para.strip())
    terms = re.findall(r"\*\*(.+?)\*\*\s+—\s+(.*?)(?=\n\n\*\*|\Z)", rest, re.S)
    nav.append((sid, num, title, len(terms)))

    h = [f'<section id="{sid}"><h2><span class="num">{num}</span>{html.escape(title)}</h2>']
    for p in pre[:1]:
        h.append(f'<p class="pre">{inline(" ".join(p.split()))}</p>')
    h.append('<dl>')
    for term, defn in terms:
        d = " ".join(defn.split())
        # A trailing italic sentence starting "Caveat" is a warning, not prose.
        cav = ""
        cm = re.search(r"\*Caveat:(.+?)\*\s*$", d)
        if cm:
            cav = f'<p class="caveat">{inline(cm.group(1).strip())}</p>'
            d = d[:cm.start()].strip()
        src = ""
        sm = re.search(r"\*\((`?[\w./,\s`-]+`?)\)\*\s*$", d)
        if sm:
            src = f'<span class="src">{inline(sm.group(1))}</span>'
            d = d[:sm.start()].strip()
        h.append(f'<div class="term" data-t="{html.escape((term+" "+d).lower())}">'
                 f'<dt>{inline(term)}{src}</dt><dd>{inline(d)}{cav}</dd></div>')
    h.append('</dl></section>')
    out.append("\n".join(h))

navhtml = "".join(
    f'<li><a href="#{s}"><span class="n">{n}</span>{html.escape(t)}'
    + (f'<span class="ct">{c}</span>' if c else "") + "</a></li>"
    for s, n, t, c in nav)
total = sum(c for _, _, _, c in nav)

tpl = open("/Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/glossary_template.html").read()
open(OUT, "w").write(tpl.replace("<!--NAV-->", navhtml)
                        .replace("<!--BODY-->", "\n".join(out))
                        .replace("{{TOTAL}}", str(total))
                        .replace("{{SECTIONS}}", str(len(nav))))
print(OUT, f"({total} terms, {len(nav)} sections)")
