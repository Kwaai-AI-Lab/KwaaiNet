"""Shared definitions for the eval2 experiment (plan: projects/kwaai-knowledge/plans/DreamRAG-Eval2-plan.md)."""
from __future__ import annotations

import hashlib
import json
from dataclasses import dataclass
from pathlib import Path

EVAL2 = Path(__file__).resolve().parent
KNOWLEDGE_TESTS = EVAL2.parent  # tests/kwaai-knowledge
REPO = EVAL2.parents[2]
RESULTS = KNOWLEDGE_TESTS / "results" / "eval2"
WORK = EVAL2 / "work"  # intermediate artefacts (gold drafts, caches); tracked selectively
KWAAINET_HOME = Path.home() / ".kwaainet"
CORPUS_ROOT = Path("/Volumes/WD2/Source/KwaaiNet/tests/rag-bench/Corpus")
QA_TRACKERS = CORPUS_ROOT / "Corpus_Final_Review" / "QA-Trackers"


@dataclass(frozen=True)
class KB:
    name: str
    entity_types: str  # as passed to `rag graph build --entity-types`
    tracker: str | None  # QA-Tracker XLSX with NotebookLM answers
    questions: str  # path relative to tests/kwaai-knowledge
    public: bool  # may its passages be sent to the Anthropic API?


# Entity types from tests/kwaai-knowledge/corpus_rebuild_dream_pipeline.sh (kb_entity_types).
KBS: dict[str, KB] = {
    "Manhattan": KB("Manhattan", "Person,Place,Organization", "Manhattan Project.xlsx",
                    "Manhattan/eval_questions.json", True),
    "PythonDocs": KB("PythonDocs", "Organization,Publication", "Python Documentation.xlsx",
                     "PythonDocs/eval_questions.json", True),
    "DeepSea": KB("DeepSea", "Person,Organization,Publication", "Deep Sea Biology.xlsx",
                  "DeepSea/eval_questions.json", True),
    "DreamMem": KB("DreamMem", "Person,Organization,Publication",
                   "Dream-Based Memory Consolidation and Forgetting.xlsx",
                   "DreamMem/eval_questions.json", True),
    "Climate": KB("Climate", "Person,Place,Organization", "Climate Science.xlsx",
                  "Climate/eval_questions.json", True),
    "Legal": KB("Legal", "Person,Organization,Legislation", "Legal Documents.xlsx",
                "Legal/eval_questions.json", True),
    # DreamMem is excluded: its tracker's NotebookLM column holds the Climate answers (question/answer
    # vocabulary overlap 0.01 vs 0.44-0.67 elsewhere), so it has no usable gold. Legal replaces it.
    # PythonDocs is excluded from the experiment: 80% of its chunks are HTML markup (56% of all
    # chunk text), an ingestion defect that would dominate any measurement. Climate replaces it.
    # The memoir: never sent to the API (private material; also a tuned dev set).
    "D6": KB("D6", "Person,Place,Organization,Legislation,Publication", None,
             "d6_eval_questions.json", False),
}

# Only these KBs may have passages or answers sent to the Anthropic API.
PUBLIC_KBS = frozenset(k for k, v in KBS.items() if v.public)


class PrivateKBError(RuntimeError):
    pass


def require_public(kb: str) -> None:
    """Hard stop before any API call carrying a KB's text."""
    if kb not in PUBLIC_KBS:
        raise PrivateKBError(f"{kb} is not on the public allowlist; its text may not leave this machine")


def text_hash(text: str) -> str:
    """Same digest as `eval_dump::text_hash` in kwaai-cli: whitespace-collapsed, SHA-1 hex.

    Pinned on both sides by fixtures/hash_vectors.json.
    """
    return hashlib.sha1(" ".join(text.split()).encode()).hexdigest()


def read_jsonl(path: Path) -> list[dict]:
    """JSON Lines records, split on "\\n" only.

    Not `str.splitlines()`: it also breaks on U+2028, U+0085, \\x1c and friends, which serde
    (the `--dump-jsonl` writer) leaves unescaped inside strings, so a record containing one is
    cut in half. Manhattan's text has them.
    """
    return [json.loads(line) for line in path.read_text().split("\n") if line.strip()]


def load_questions(kb: str) -> list[dict]:
    raw = json.loads((KNOWLEDGE_TESTS / KBS[kb].questions).read_text())
    return raw if isinstance(raw, list) else raw["questions"]


def tenant_of(kb_name: str) -> str:
    """tenant_id of a KB registered in ~/.kwaainet/config.yaml (a clone shares its source's)."""
    import re

    cfg = (KWAAINET_HOME / "config.yaml").read_text()
    m = re.search(rf"(?m)^  {re.escape(kb_name)}:\n(?:    .*\n)*?    tenant_id: (\S+)", cfg)
    if not m:
        raise KeyError(f"{kb_name} not in config.yaml rag_kbs")
    return m.group(1)


def data_dir(kb_name: str) -> Path:
    return KWAAINET_HOME / "rag" / kb_name
