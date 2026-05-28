#!/usr/bin/env python3
"""Run the RAG benchmark question set against Google AI Studio (Gemini File API)."""

import json
import os
import sys
import time
from pathlib import Path

CORPUS_DIR = Path(os.environ.get("CORPUS_DIR", "/Users/rezarassool/Source/PreRAG/output"))
QUESTIONS_FILE = Path(__file__).parent / "questions.json"
RESULTS_DIR = Path(__file__).parent / "results"

# Load .env from the same directory if present
_env_file = Path(__file__).parent / ".env"
if _env_file.exists():
    for _line in _env_file.read_text().splitlines():
        if "=" in _line and not _line.startswith("#"):
            _k, _v = _line.split("=", 1)
            os.environ.setdefault(_k.strip(), _v.strip())
FILE_REFS_CACHE = RESULTS_DIR / "gemini_file_refs.json"
GEMINI_MODEL = os.environ.get("GEMINI_MODEL", "gemini-1.5-pro")

def get_api_key():
    key = os.environ.get("GOOGLE_API_KEY")
    if not key:
        print("ERROR: GOOGLE_API_KEY env var not set.")
        print("Get one from https://aistudio.google.com/app/apikey")
        sys.exit(1)
    return key

def upload_corpus(genai):
    """Upload all corpus docs to Gemini File API; cache URIs to avoid re-uploading."""
    RESULTS_DIR.mkdir(exist_ok=True)
    docs = sorted(CORPUS_DIR.glob("*.docx")) + sorted(CORPUS_DIR.glob("*.pdf")) + sorted(CORPUS_DIR.glob("*.txt"))

    if FILE_REFS_CACHE.exists():
        cached = json.loads(FILE_REFS_CACHE.read_text())
        # Verify files are still available
        print(f"Found cached file refs ({len(cached)} files). Verifying...")
        try:
            for ref in cached[:2]:
                genai.get_file(ref["name"])
            print("Cache valid — skipping upload.")
            return cached
        except Exception:
            print("Cache stale — re-uploading.")

    print(f"Uploading {len(docs)} corpus files to Gemini File API...")
    refs = []
    for i, doc in enumerate(docs, 1):
        print(f"  [{i:02d}/{len(docs)}] {doc.name}...")
        try:
            uploaded = genai.upload_file(str(doc), display_name=doc.name)
            refs.append({"name": uploaded.name, "uri": uploaded.uri, "display_name": doc.name})
        except Exception as e:
            print(f"    WARNING: failed to upload {doc.name}: {e}")

    FILE_REFS_CACHE.write_text(json.dumps(refs, indent=2))
    print(f"Uploaded {len(refs)} files. Refs cached to {FILE_REFS_CACHE}")
    return refs

def build_file_parts(genai, refs):
    """Return a list of Part objects for all uploaded files."""
    import google.generativeai as genai_module
    return [genai_module.types.Part.from_uri(r["uri"], mime_type="application/vnd.openxmlformats-officedocument.wordprocessingml.document") for r in refs]

def main():
    get_api_key()
    try:
        import google.generativeai as genai
    except ImportError:
        print("ERROR: google-generativeai not installed.")
        print("Run: pip install google-generativeai")
        sys.exit(1)

    genai.configure(api_key=os.environ["GOOGLE_API_KEY"])

    questions = json.loads(QUESTIONS_FILE.read_text())
    refs = upload_corpus(genai)

    model = genai.GenerativeModel(GEMINI_MODEL)

    # Build a single prompt prefix with all docs attached
    doc_parts = []
    for ref in refs:
        try:
            f = genai.get_file(ref["name"])
            doc_parts.append(f)
        except Exception as e:
            print(f"  WARNING: could not load {ref['display_name']}: {e}")

    print(f"\nRunning {len(questions)} questions against Gemini ({GEMINI_MODEL}) with {len(doc_parts)} docs...")
    results = []

    system_prompt = (
        "You are a research assistant. Answer the question using ONLY the provided documents. "
        "If the answer is not found in the documents, say so clearly — do not invent information."
    )

    for i, q in enumerate(questions, 1):
        print(f"  [{i:02d}/{len(questions)}] {q['id']}: {q['question'][:60]}...")
        t0 = time.time()
        try:
            prompt_parts = doc_parts + [f"{system_prompt}\n\nQuestion: {q['question']}"]
            response = model.generate_content(prompt_parts)
            total_ms = int((time.time() - t0) * 1000)
            answer = response.text.strip() if response.text else ""
            results.append({
                "id": q["id"],
                "question": q["question"],
                "category": q["category"],
                "source_hint": q.get("source_hint"),
                "reference_answer": q.get("reference_answer"),
                "answer": answer,
                "total_ms": total_ms,
                "error": None,
            })
        except Exception as e:
            total_ms = int((time.time() - t0) * 1000)
            print(f"    ERROR: {e}")
            results.append({
                "id": q["id"],
                "question": q["question"],
                "category": q["category"],
                "source_hint": q.get("source_hint"),
                "reference_answer": q.get("reference_answer"),
                "answer": "",
                "total_ms": total_ms,
                "error": str(e),
            })
        # Rate limiting: Gemini free tier is 2 RPM for 1.5 Pro
        time.sleep(1)

    out_file = RESULTS_DIR / "gemini_results.json"
    out_file.write_text(json.dumps(results, indent=2))
    ok = sum(1 for r in results if not r["error"])
    print(f"\nDone. {ok}/{len(results)} succeeded → {out_file}")

if __name__ == "__main__":
    main()
