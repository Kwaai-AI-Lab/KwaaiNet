#!/usr/bin/env python3
"""
GLiNER NER server for KwaaiNet graph entity extraction.

Exposes a single endpoint:
  POST /ner
  Request:  {"text": "...", "labels": ["person"], "threshold": 0.4}
  Response: [{"text": "Joe Rassool", "label": "person", "score": 0.92}, ...]

Usage:
  pip install gliner fastapi uvicorn
  python scripts/gliner_server.py [--host 0.0.0.0] [--port 8080] [--model urchade/gliner_small-v2.1]

Then pass --gliner-url http://localhost:8080 to `kwaainet rag graph build`.

Recommended models (all run on CPU, ~0.5B params, ~5ms/chunk):
  urchade/gliner_small-v2.1   — fast, good general NER
  urchade/gliner_medium-v2.1  — more accurate, 2× slower
  knowledgator/gliner-bi-small-v1.0 — bidirectional, strong on historical names
"""

import argparse
import logging
from typing import Any

import uvicorn
from fastapi import FastAPI
from fastapi.responses import JSONResponse
from gliner import GLiNER
from pydantic import BaseModel

logging.basicConfig(level=logging.INFO, format="%(asctime)s %(levelname)s %(message)s")
log = logging.getLogger(__name__)

app = FastAPI(title="GLiNER NER Server")
_model: GLiNER | None = None


class NERRequest(BaseModel):
    text: str
    labels: list[str] = ["person"]
    threshold: float = 0.4


@app.on_event("startup")
async def _load_model() -> None:
    global _model
    log.info("Loading GLiNER model %s …", _args.model)
    _model = GLiNER.from_pretrained(_args.model)
    log.info("Model ready.")


@app.post("/ner")
async def ner(req: NERRequest) -> Any:
    if _model is None:
        return JSONResponse(status_code=503, content={"error": "model not loaded"})
    entities = _model.predict_entities(req.text, req.labels, threshold=req.threshold)
    result = [
        {"text": e["text"], "label": e["label"], "score": round(float(e["score"]), 4)}
        for e in entities
    ]
    return result


@app.get("/health")
async def health() -> dict:
    return {"status": "ok", "model_loaded": _model is not None}


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="GLiNER NER server")
    parser.add_argument("--host", default="127.0.0.1")
    parser.add_argument("--port", type=int, default=8080)
    parser.add_argument("--model", default="urchade/gliner_small-v2.1")
    _args = parser.parse_args()

    log.info("Starting GLiNER server on %s:%d", _args.host, _args.port)
    uvicorn.run(app, host=_args.host, port=_args.port, log_level="info")
