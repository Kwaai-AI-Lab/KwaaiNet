"""Thin Claude API helper for eval2: structured JSON calls with refusal handling and usage logging.

Every call names the KB whose text it carries and is refused unless that KB is on the public
allowlist (common.PUBLIC_KBS), so the memoir can never reach the API by accident.

Credentials: the SDK resolves ANTHROPIC_API_KEY (or an `ant auth login` profile) itself.
"""
from __future__ import annotations

import json
import time
from pathlib import Path

import anthropic

from common import WORK, require_public

MODEL = "claude-opus-5"
FALLBACK_BETA = "server-side-fallback-2026-07-01"
USAGE_LOG = WORK / "cache" / "claude_usage.jsonl"

_client: anthropic.Anthropic | None = None


KEY_FILE = Path.home() / ".anthropic" / "eval2_key"  # chmod 600; used when ANTHROPIC_API_KEY is unset
WORKSPACE_FILE = Path.home() / ".anthropic" / "eval2_workspace"


def client() -> anthropic.Anthropic:
    global _client
    if _client is None:
        import os

        key = os.environ.get("ANTHROPIC_API_KEY") or (KEY_FILE.read_text().strip() if KEY_FILE.exists() else None)
        # The key is organization-scoped, so each request names the workspace to use.
        ws = os.environ.get("ANTHROPIC_WORKSPACE_ID") or (
            WORKSPACE_FILE.read_text().strip() if WORKSPACE_FILE.exists() else None)
        headers = {"anthropic-workspace-id": ws} if ws else None
        _client = anthropic.Anthropic(api_key=key, max_retries=4, default_headers=headers)
    return _client


class Refused(RuntimeError):
    pass


def _log(kind: str, kb: str, resp, extra: dict | None = None) -> None:
    USAGE_LOG.parent.mkdir(parents=True, exist_ok=True)
    u = resp.usage
    rec = {"ts": time.time(), "kind": kind, "kb": kb, "model": resp.model,
           "request_id": getattr(resp, "_request_id", None), "stop_reason": resp.stop_reason,
           "input_tokens": u.input_tokens, "output_tokens": u.output_tokens,
           "cache_read": getattr(u, "cache_read_input_tokens", 0) or 0,
           "cache_write": getattr(u, "cache_creation_input_tokens", 0) or 0}
    rec.update(extra or {})
    with USAGE_LOG.open("a") as f:
        f.write(json.dumps(rec) + "\n")


def call_json(*, kb: str, kind: str, system: str, user: str, schema: dict,
              effort: str = "medium", max_tokens: int = 8000) -> dict:
    """One structured call; returns the parsed JSON object.

    `system` should be the stable part (rubric, instructions) so repeated calls share a
    cached prefix; `user` carries the per-item content.
    """
    require_public(kb)
    resp = client().beta.messages.create(
        model=MODEL,
        max_tokens=max_tokens,
        betas=[FALLBACK_BETA],
        extra_body={"fallbacks": "default"},
        system=[{"type": "text", "text": system, "cache_control": {"type": "ephemeral"}}],
        output_config={"effort": effort, "format": {"type": "json_schema", "schema": schema}},
        messages=[{"role": "user", "content": user}],
    )
    _log(kind, kb, resp)
    if resp.stop_reason == "refusal":
        raise Refused(f"{kind} refused for {kb}: {getattr(resp, 'stop_details', None)}")
    if resp.stop_reason == "max_tokens":
        raise RuntimeError(f"{kind} hit max_tokens for {kb}")
    text = next(b.text for b in resp.content if b.type == "text")
    return json.loads(text)


def cost_summary(path: Path = USAGE_LOG) -> dict:
    """Token totals from the usage log (dollar cost is left to the reader: rates change)."""
    tot = {"calls": 0, "input": 0, "output": 0, "cache_read": 0, "cache_write": 0}
    if path.exists():
        for line in path.read_text().splitlines():
            r = json.loads(line)
            tot["calls"] += 1
            tot["input"] += r["input_tokens"]
            tot["output"] += r["output_tokens"]
            tot["cache_read"] += r["cache_read"]
            tot["cache_write"] += r["cache_write"]
    return tot
