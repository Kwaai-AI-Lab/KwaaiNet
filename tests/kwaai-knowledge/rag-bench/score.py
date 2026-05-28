#!/usr/bin/env python3
"""Score kwaainet and Gemini results using LLM-as-judge, golden F1, and retrieval metrics."""

import json
import os
import re
import sys
import time
from pathlib import Path

import requests

RESULTS_DIR = Path(__file__).parent / "results"
JUDGE_URL = os.environ.get("JUDGE_URL", "http://localhost:8080")

JUDGE_PROMPT = """\
You are a RAG evaluation judge. Score the answer below on three dimensions, each 0–3.

Question: {question}
Answer: {answer}
{context_section}
Dimensions:
  Faithfulness  (0=hallucinated or unsupported, 3=fully grounded in the documents)
  Relevance     (0=misses the question, 3=directly and completely answers it)
  Completeness  (0=key information missing, 3=covers all key aspects)

For out-of-scope questions (facts not in the documents), a correct response
that says "the answer is not in the documents" scores 3 on all dimensions.
An answer that invents information scores 0 on Faithfulness.

Reply as JSON only, no other text:
{{"faithfulness": X, "relevance": X, "completeness": X}}"""


def llm_judge(question: str, answer: str, context_texts: list[str]) -> dict:
    """Call kwaainet shard API to judge an answer. Returns scores or None on failure."""
    if not answer.strip():
        return {"faithfulness": 0, "relevance": 0, "completeness": 0}

    context_section = ""
    if context_texts:
        ctx = "\n".join(f"[{i+1}] {t[:300]}" for i, t in enumerate(context_texts[:3]))
        context_section = f"Retrieved context:\n{ctx}\n"

    prompt = JUDGE_PROMPT.format(
        question=question,
        answer=answer[:800],
        context_section=context_section,
    )

    for attempt in range(3):
        try:
            resp = requests.post(
                f"{JUDGE_URL}/v1/chat/completions",
                json={"messages": [{"role": "user", "content": prompt}], "stream": False},
                timeout=30,
            )
            resp.raise_for_status()
            content = resp.json()["choices"][0]["message"]["content"]
            # Extract JSON block even if there's surrounding text
            m = re.search(r'\{[^}]+\}', content)
            if m:
                scores = json.loads(m.group())
                return {
                    "faithfulness": float(scores.get("faithfulness", 0)),
                    "relevance": float(scores.get("relevance", 0)),
                    "completeness": float(scores.get("completeness", 0)),
                }
        except Exception as e:
            if attempt == 2:
                print(f"    Judge failed after 3 attempts: {e}")
    return None


def golden_f1(answer: str, reference: str) -> float:
    """Fraction of key phrases from reference that appear in answer (case-insensitive)."""
    if not reference or not answer:
        return 0.0
    # Split reference into key phrases (sentences or comma-separated segments)
    phrases = [p.strip().lower() for p in re.split(r'[;,\n]', reference) if len(p.strip()) > 3]
    if not phrases:
        return 0.0
    answer_lower = answer.lower()
    matched = sum(1 for p in phrases if p in answer_lower)
    return matched / len(phrases)


def hit_at_k(sources: list[dict], source_hint: str, k: int = 5) -> bool:
    """True if source_hint doc appears in the top-k sources."""
    if not source_hint or not sources:
        return None  # not applicable
    top_k = sources[:k]
    return any(r.get("doc", "") == source_hint for r in top_k)


def mrr(sources: list[dict], source_hint: str) -> float:
    """Reciprocal rank of the first matching source."""
    if not source_hint or not sources:
        return None
    for i, r in enumerate(sources, 1):
        if r.get("doc", "") == source_hint:
            return 1.0 / i
    return 0.0


def score_results(results: list[dict], system_name: str) -> dict:
    judge_scores = []
    f1_scores = []
    hit5_values = []
    mrr_values = []
    retrieval_ms_values = []
    generation_ms_values = []
    total_ms_values = []

    for i, r in enumerate(results, 1):
        print(f"  [{i:02d}/{len(results)}] {r['id']} ({system_name}): judging...")

        context_texts = [s["text"] for s in r.get("sources", [])] if "sources" in r else []
        scores = llm_judge(r["question"], r["answer"], context_texts)
        if scores:
            avg = (scores["faithfulness"] + scores["relevance"] + scores["completeness"]) / 3
            judge_scores.append(avg)
            r[f"judge_{system_name}"] = {**scores, "avg": round(avg, 3)}
        else:
            r[f"judge_{system_name}"] = None

        if r.get("reference_answer"):
            f1 = golden_f1(r["answer"], r["reference_answer"])
            f1_scores.append(f1)
            r[f"golden_f1_{system_name}"] = round(f1, 3)

        if "sources" in r:
            h = hit_at_k(r["sources"], r.get("source_hint"))
            if h is not None:
                hit5_values.append(h)
            m = mrr(r["sources"], r.get("source_hint"))
            if m is not None:
                mrr_values.append(m)

        if r.get("retrieval_ms") is not None:
            retrieval_ms_values.append(r["retrieval_ms"])
        if r.get("generation_ms") is not None:
            generation_ms_values.append(r["generation_ms"])
        if r.get("total_ms") is not None:
            total_ms_values.append(r["total_ms"])

        time.sleep(0.5)  # avoid overwhelming the judge

    def avg(lst):
        return round(sum(lst) / len(lst), 3) if lst else None

    return {
        "judge_score": avg(judge_scores),
        "golden_f1": avg(f1_scores),
        "hit_at_5": avg([1 if h else 0 for h in hit5_values]) if hit5_values else None,
        "mrr": avg(mrr_values) if mrr_values else None,
        "avg_retrieval_ms": avg(retrieval_ms_values),
        "avg_generation_ms": avg(generation_ms_values),
        "avg_total_ms": avg(total_ms_values),
        "n_judged": len(judge_scores),
        "n_golden": len(f1_scores),
    }


def main():
    kw_file = RESULTS_DIR / "kwaainet_results.json"
    gm_file = RESULTS_DIR / "gemini_results.json"

    if not kw_file.exists() and not gm_file.exists():
        print("No results found. Run run_kwaainet.py and/or run_gemini.py first.")
        sys.exit(1)

    # Verify judge is reachable
    try:
        requests.get(f"{JUDGE_URL}/v1/models", timeout=5).raise_for_status()
    except Exception as e:
        print(f"ERROR: LLM judge not reachable at {JUDGE_URL}: {e}")
        print("Start it with: kwaainet shard api --port 8080")
        sys.exit(1)

    scores = {}

    if kw_file.exists():
        print(f"\n=== Scoring kwaainet ({kw_file}) ===")
        kw = json.loads(kw_file.read_text())
        scores["kwaainet"] = score_results(kw, "kwaainet")
        kw_file.write_text(json.dumps(kw, indent=2))  # write back with judge scores

    if gm_file.exists():
        print(f"\n=== Scoring gemini ({gm_file}) ===")
        gm = json.loads(gm_file.read_text())
        scores["gemini"] = score_results(gm, "gemini")
        gm_file.write_text(json.dumps(gm, indent=2))

    out_file = RESULTS_DIR / "scores.json"
    out_file.write_text(json.dumps(scores, indent=2))
    print(f"\nScores written to {out_file}")
    print(json.dumps(scores, indent=2))

if __name__ == "__main__":
    main()
