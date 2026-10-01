"""Read a KB's chunk store read-only, and BM25 search over it.

Chunk ids are the same ids `rag eval --dump-jsonl` records (the little-endian i64 in bytes
16..24 of the key, after the tenant UUID; `MetaStore::chunk_key` in kwaai-rag/src/meta_store.rs),
so gold passages found here anchor directly to retrieved chunks. `graph build --reset-graph`
keeps chunks, so ids are stable across every dream snapshot of a KB.
"""
from __future__ import annotations

import json
import re
import sqlite3
import struct
from dataclasses import dataclass
from functools import lru_cache

from common import data_dir, tenant_of, text_hash


@dataclass(frozen=True)
class Chunk:
    chunk_id: int
    doc_name: str
    chunk_index: int
    text: str
    hash: str


def _printable(text: str) -> float:
    if not text:
        return 0.0
    ok = sum(ch.isprintable() or ch in "\n\t" for ch in text)
    ascii_ish = sum(ch.isascii() for ch in text)
    return min(ok, ascii_ish) / len(text)


@lru_cache(maxsize=None)
def load_chunks(kb_name: str) -> tuple[Chunk, ...]:
    """All usable chunks of a KB. Binary junk (mis-ingested files) is dropped."""
    tenant = tenant_of(kb_name)
    path = data_dir(kb_name) / f"{tenant}.db"
    con = sqlite3.connect(f"file:{path}?mode=ro", uri=True)
    out = []
    for key, value in con.execute("SELECT key, CAST(value AS TEXT) FROM chunks"):
        (cid,) = struct.unpack("<q", key[16:24])
        d = json.loads(value)
        text = d.get("text") or ""
        if len(text.strip()) < 20 or _printable(text) < 0.85:
            continue
        out.append(Chunk(cid, d.get("doc_name", ""), int(d.get("chunk_index", 0)), text, text_hash(text)))
    con.close()
    return tuple(sorted(out, key=lambda c: c.chunk_id))


_TAG = re.compile(r"<[^>]{0,500}>")
_ENTITY = re.compile(r"&(nbsp|amp|lt|gt|quot|#\d+|#x[0-9a-f]+|[a-z]+);", re.I)
_URL = re.compile(r"https?://\S+")


def clean_text(text: str) -> str:
    """Text for NLI: HTML tags, entities and URLs removed, whitespace collapsed.

    Two corpora (PythonDocs, part of Manhattan) were ingested with their HTML markup, which
    drives an MNLI model to uniform ~0.4 scores. The chunk itself (and its hash) is unchanged;
    only what the scorer reads is cleaned.
    """
    import html

    text = _TAG.sub(" ", text)
    text = html.unescape(_ENTITY.sub(lambda m: html.unescape(m.group(0)), text))
    text = _URL.sub(" ", text)
    return " ".join(text.split())


_TOK = re.compile(r"[a-z0-9]+")
_STOP = frozenset("the a an of and or to in on for by with is was were are be been as at that this which "
                  "from it its into their his her they he she what who when where how why did does do".split())


def tokens(text: str) -> list[str]:
    return [t for t in _TOK.findall(text.lower()) if t not in _STOP]


class BM25Index:
    def __init__(self, kb_name: str):
        from rank_bm25 import BM25Okapi

        self.chunks = load_chunks(kb_name)
        self.bm25 = BM25Okapi([tokens(c.text) for c in self.chunks])

    def search(self, query: str, k: int = 30) -> list[Chunk]:
        scores = self.bm25.get_scores(tokens(query))
        order = sorted(range(len(scores)), key=lambda i: scores[i], reverse=True)[:k]
        return [self.chunks[i] for i in order if scores[i] > 0]
