#!/usr/bin/env python3
"""Build the AIAS+ forgetting extended abstract (.docx) on the EasyChair Word template.

    python3 projects/kwaai-knowledge/papers/aias-forgetting/build_docx.py

Reuses the pandoc pre/post-processing of ../aias2026/build_manuscript_docx.py (its --short mode:
the .md places its own figure). Edit the .md, never the .docx.
"""
import importlib.util
import subprocess
import tempfile
from pathlib import Path

from docx import Document

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("aias2026_build", HERE.parent / "aias2026/build_manuscript_docx.py")
b = importlib.util.module_from_spec(spec)
spec.loader.exec_module(b)

b.SRC = HERE / "DreamRAG-Forgetting-extended-abstract.md"
b.SHORT = True
OUT = HERE / "forgetting-extended-abstract.docx"

import re

src = b.SRC.read_text()


def front(key: str) -> str:
    m = re.search(rf'^{key}: "(.*)"$', src, re.M)
    if not m:
        raise SystemExit(f"front matter has no {key}:")
    return m[1]


# AIAS+ extended abstracts carry a named author block, not the anonymous one the 2026 build writes.
md = b.preprocess(src)
for anon, key in (("Anonymous author(s)", "authors"), ("Anonymous institution(s)", "affiliation")):
    assert md.count(anon) == 1, anon
    md = md.replace(anon, front(key))
md = md.replace(front("affiliation"), front("affiliation") + "\n:::\n\n::: {custom-style=\"Institute\"}\n"
                + front("note"), 1)

with tempfile.TemporaryDirectory() as tmp:
    mid = Path(tmp) / "abstract.md"
    mid.write_text(md)
    subprocess.run(["pandoc", str(mid), "-f", "markdown", "--reference-doc", str(b.TEMPLATE),
                    "--resource-path", str(HERE), "-o", str(OUT)], check=True)
b.postprocess(OUT)
doc = Document(OUT)
doc.core_properties.title = "Extended abstract"
doc.save(OUT)
print("wrote", OUT.relative_to(b.REPO))
