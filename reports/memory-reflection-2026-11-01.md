# Memory reflection — 2026-11-01

## Mechanical — checker output, verbatim

```
Checking 56 memory and plan files
...
0 broken reference(s), 134 warning(s) across 56 files

Drift is a prompt to re-read, not a failure. Reflect on whether the doc still says something true, then touch it or fix it.
```

(Full warning list omitted here for brevity — rerun `python3 scripts/check_memory_integrity.py` for the complete listing.) All 134 warnings fall in `projects/*/plans/*.md` forward-looking plan documents (mostly `tests/kwaai-knowledge/` eval plans referencing not-yet-created YAML files, and a handful of function/line drift from refactors). The 12 `CLAUDE.md` files and `projects/kwaai-knowledge/GLOSSARY.md` — the files agents actually load every session — have **zero** warnings and zero broken references.

## Semantic — claims now false, with file, line, and the commit responsible

`git log --since="36 hours ago" --stat` returned no commits. Nothing landed in the window this step is scoped to, so there is nothing to check against. (The findings below come from Step 3's accretion pass, not Step 2 — they are pre-existing staleness, not caused by a recent commit.)

## Accretion — what could be removed and why

- **Dead peer ID still given as a live example**, in `projects/kwaai-compute/CLAUDE.md:32,36` and `projects/kwaai-knowledge/CLAUDE.md:39,103,111`: both list `metro-linux` as peer ID `12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs`. This ID was written in commit `59cdf3c9` (2026-09-11). Three weeks later, `projects/kwaai-knowledge/plans/DreamRAG-Eval2-plan.md:51` (committed 2026-10-01) explicitly calls this exact ID "the **dead** metro-linux id" and gives the replacement, `12D3KooWA33TMz7ss8K2oPQr99KsLwW6AbLYhrL7P1V36vJXXM38` — which is what `tests/kwaai-knowledge/eval2/driver/build.py:28` actually uses. An agent copying the `--inference-urls` example from either CLAUDE.md today dials a dead peer.

- **Relay routing documented as "planned" after it shipped**, in `projects/kwaai-network/CLAUDE.md:33` ("Fix needed: p2p relay routing for inference requests") and `:62-63` ("In progress / planned: P2P relay routing for inference"). `core/crates/kwaai-cli/src/ollama_proxy.rs::resolve_inference_urls` (line 617 handles `p2p://` URLs) and `inference_mux.rs` implement exactly this, and both files trace to the *same* commit, `59cdf3c9` (2026-09-11), that wrote the "planned" lines — self-contradictory since the day it was written. `kwaai-compute/CLAUDE.md` and `kwaai-knowledge/CLAUDE.md` already document `p2p://PEER_ID` routing as the normal, shipped mechanism with working examples, so `kwaai-network/CLAUDE.md` is the one file out of step.

- **Internal contradiction in `projects/kwaai-storage/CLAUDE.md`**: line 90 says "Do not store unencrypted vectors for `eve`/`bob` modes" (from commit `59cdf3c9`, 2026-09-11), while line 6 (scope) and line 57 (Shipped) say "vectors are plaintext f32 today — encryption is planned" / "Multi-tenant vector tables (plaintext f32 vectors today)" (from commit `1c0b342`, three days later, 2026-09-14). The later, more current text says the thing the "do not" rule forbids is already the shipped state. The do-not line needs rewording — e.g. "do not treat today's plaintext storage as acceptable for shipping `eve`/`bob` modes without encryption landing first" — or removal if the plaintext state is intentionally accepted for now.

- **Duplicate table row** in `projects/kwaai-trust/CLAUDE.md:56-57`: `kwaai-cli/src/identity.rs` is listed twice — once as "`kwaainet identity` command handler", once as "`kwaainet identity` handler — DID, VC import/list/verify" — both added in commit `59cdf3c9`. Pure duplication; collapse to one row.

- **Personal, out-of-repo plan path**, repeated three times: `projects/kwaai-compute/CLAUDE.md:75`, `TODO.md:11`, and `roadmap.md:17` all point to `~/.claude/plans/cached-jingling-creek.md` for the "dedicated inference thread with session pool" work. That path is one developer's local Claude Code plans directory — unreadable by any other contributor or agent working from a fresh clone. Either inline the plan's substance into the repo (e.g. as a `projects/kwaai-compute/plans/*.md` file, matching how other in-progress work is tracked) or drop the dead reference.

- **Stale "New in this branch" framing**, `projects/kwaai-knowledge/GLOSSARY.md:151`: the per-KB ontology section says "New in this branch; see `plans/PerKBOntology-plan.md`." That text dates to commit `59cdf3c9` (2026-09-11), which has been on `main` for ~7 weeks. Not factually wrong, just branch-relative language that no longer matches reality — minor, wording-only fix.

## Nothing found

- No broken references in any of the 12 `CLAUDE.md` files or `GLOSSARY.md` (mechanical check clean).
- No semantic drift from recent commits — there were none in the last 36 hours.
- `core/crates/kwaai-inference/CLAUDE.md`, `core/crates/kwaai-p2p/CLAUDE.md`, `core/crates/kwaai-rag/CLAUDE.md`, `core/crates/kwaai-storage/CLAUDE.md`, `core/crates/kwaai-trust/CLAUDE.md`, `projects/kwaai-platform/CLAUDE.md`, and the root `CLAUDE.md` were reviewed for accretion and are clean — all file references resolve and no contradictions were found.
