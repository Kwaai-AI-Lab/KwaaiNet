# Deck branding

Renders the course markdown to a Kwaai-branded pptx.

```bash
pandoc ../DreamRAG-Course-Weeks-3-16.md -o /tmp/base.pptx --slide-level=2
python3 brand_deck.py /tmp/base.pptx kwaai-logo.png \
  ../../../../rendered/projects/kwaai-knowledge/teaching/DreamRAG-Course-Weeks-3-16.pptx
```

`kwaai-logo.png` was extracted from the title slide of `Kwaai Intern Program.pdf`
and composited with its alpha mask, so it has a transparent background.

Palette sampled from the logo itself:

| | hex | used for |
|---|---|---|
| Kwaai blue | `#0A63DC` | content slide titles |
| deep blue | `#0A3FB4` | title slide, section headings |
| brown | `#4A3520` | subtitle, section rules |

The script places the logo bottom-right on content slides — top-right collides
with pandoc's content placeholders and tables, which start at y≈0.22in — and
positions each section rule from its own title's geometry rather than a fixed
offset, so it stays put if a heading wraps to two lines.

Requires `python-pptx`. Verify after any change: no shape may extend past the
canvas, and no logo may overlap content.
