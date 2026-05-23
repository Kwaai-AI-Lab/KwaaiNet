#!/usr/bin/env python3
"""Run the RAG benchmark question set against kwaainet rag serve."""

import json
import os
import sys
import time
from pathlib import Path

import requests

KWAAINET_URL = os.environ.get("KWAAINET_URL", "http://localhost:9090")
QUESTIONS_FILE = Path(__file__).parent / "questions.json"
RESULTS_DIR = Path(__file__).parent / "results"

def main():
    RESULTS_DIR.mkdir(exist_ok=True)
    questions = json.loads(QUESTIONS_FILE.read_text())

    # Verify server is up
    try:
        requests.get(f"{KWAAINET_URL}/v1/models", timeout=5).raise_for_status()
    except Exception as e:
        print(f"ERROR: kwaainet rag serve not reachable at {KWAAINET_URL}: {e}")
        print("Start it with: kwaainet rag serve --port 9090")
        sys.exit(1)

    print(f"Running {len(questions)} questions against {KWAAINET_URL}...")
    results = []

    for i, q in enumerate(questions, 1):
        print(f"  [{i:02d}/{len(questions)}] {q['id']}: {q['question'][:60]}...")
        t0 = time.time()
        try:
            resp = requests.post(
                f"{KWAAINET_URL}/v1/chat/completions",
                json={
                    "messages": [{"role": "user", "content": q["question"]}],
                    "include_sources": True,
                },
                timeout=60,
            )
            resp.raise_for_status()
            body = resp.json()
            total_ms = int((time.time() - t0) * 1000)

            answer = (
                body.get("choices", [{}])[0]
                .get("message", {})
                .get("content", "")
            )
            sources = body.get("sources", [])
            usage = body.get("usage", {})

            results.append({
                "id": q["id"],
                "question": q["question"],
                "category": q["category"],
                "source_hint": q.get("source_hint"),
                "reference_answer": q.get("reference_answer"),
                "answer": answer,
                "sources": sources,
                "retrieval_ms": usage.get("retrieval_ms"),
                "generation_ms": usage.get("generation_ms"),
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
                "sources": [],
                "retrieval_ms": None,
                "generation_ms": None,
                "total_ms": total_ms,
                "error": str(e),
            })

    out_file = RESULTS_DIR / "kwaainet_results.json"
    out_file.write_text(json.dumps(results, indent=2))
    ok = sum(1 for r in results if not r["error"])
    print(f"\nDone. {ok}/{len(results)} succeeded → {out_file}")

if __name__ == "__main__":
    main()
