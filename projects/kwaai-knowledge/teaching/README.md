# DreamRAG course deck

`DreamRAG-Course-Weeks-3-16.md` holds the course content. The live teaching copy
is in **Google Slides**.

## Build and import

```bash
pandoc DreamRAG-Course-Weeks-3-16.md -o /tmp/course.pptx --slide-level=2
```

Upload to Drive → *Open with Google Slides* → **Theme → Import theme** → select
the existing Kwaai intern deck. The logo and styling then come from the master.

## Which copy is authoritative

**After the first import, the Google Slides copy is the live artifact.** Edit
there week to week; do not re-render on top of it.

Re-render only for a *structural* rewrite — if the week-7 cut review reshapes the
back half of the course, revising fourteen weeks of markdown and re-importing
beats restructuring 83 slides by hand. In that case, import as a new deck and
re-apply the theme rather than merging.

## Notes

- 83 slides, weeks 3–16, four workstreams.
- Headings, bullets, tables, emphasis and code blocks survive import. Long code
  lines may need hand-wrapping — Slides does not reflow them.
- No speaker notes yet. Worth adding for whoever presents, as markdown under each
  slide or directly in Slides.
- Curriculum rationale: `../plans/DreamRAG-Intern-Curriculum.md`.
