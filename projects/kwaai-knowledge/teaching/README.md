# DreamRAG course deck

`DreamRAG-Course-Weeks-3-16.md` is the source of truth. Everything else is a
build output in gitignored `rendered/`.

## Two render targets, pick by workflow

**Google Slides (recommended for teaching).** The existing intern deck lives
there, so the house theme is already defined; a cohort can co-edit and comment;
and template inheritance is real rather than stamped per slide.

```bash
pandoc DreamRAG-Course-Weeks-3-16.md -o /tmp/course.pptx --slide-level=2
```

Then: upload to Drive → *Open with Google Slides* → **Theme → Import theme** →
pick the existing intern deck. Content flows into the house master.

Do **not** import the branded build for this — it stamps a logo onto every
slide, which would double up against a themed master across all 83 slides.

**PowerPoint / Keynote (offline, no account needed).**

```bash
pandoc DreamRAG-Course-Weeks-3-16.md -o /tmp/base.pptx --slide-level=2
python3 brand/brand_deck.py /tmp/base.pptx brand/kwaai-logo.png \
  ../../../rendered/projects/kwaai-knowledge/teaching/DreamRAG-Course-Weeks-3-16.pptx
```

## What survives the Slides import

Headings, bullets, tables, bold/italic, code blocks, and slide breaks all carry
over. Speaker notes do not exist in the markdown yet. Long code lines may need
manual wrapping — Slides does not reflow them the way pandoc's placeholder does.

## Editing after import

Once it is in Slides, that copy becomes the live artifact for the cohort and
this markdown becomes a record of the original structure. That is a reasonable
trade for a teaching deck — but if slides are going to be revised heavily each
week, edit in Slides and stop re-rendering, rather than keeping two drifting
copies.
