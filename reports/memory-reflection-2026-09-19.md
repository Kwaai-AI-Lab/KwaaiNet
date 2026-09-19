# Memory Reflection — 2026-09-19

Reviewed at `b9a73436c314d9345059213250cca6310e24769d` (origin/main).

## Mechanical — checker output, verbatim

```
Checking 48 memory and plan files
...
0 broken reference(s), 45 warning(s) across 48 files

Drift is a prompt to re-read, not a failure. Reflect on whether the doc still says something true, then touch it or fix it.
```

0 broken references. All 45 warnings are `(proposed?)` file/function references inside
`projects/*/plans/*.md` design docs (e.g. `ontology.yaml`, `relation_type_classifier.rs`,
`Ledger-plan.md`) — these name not-yet-created files by design and are not drift. Nothing
to act on here.

## Semantic — claims now false, with file, line, and the commit responsible

Reviewed every commit from the last 36 hours (`38c453c` through `b9a7343`, 8 commits: PR
capping doc, installer/CUDA fix, inference reinstall-hint fix, shard status-hint fix,
Ollama `/api/tags` detection fix, shard auto-pinning explanation, `config set` refusal,
and the Petals bridge roadmap/example removal).

**Nothing found.** None of these eight commits touch code, defaults, flags, or CLI
surface that any of the 13 memory files (12 `CLAUDE.md` + `GLOSSARY.md`) documents:

- `PETALS_BRIDGE_ROADMAP.md` and `petals_bootstrap.rs` (deleted in `b9a7343`) are not
  referenced by any tracked `.md` file — confirmed with a repo-wide grep for both
  filenames.
- The Ollama detection change (`ea76bf0`: prefer `/api/tags` over a filesystem manifest
  scan) — no `CLAUDE.md` documents the old filesystem-scan mechanism.
- The `config set` refusal-while-running change (`e7c581d`) and the shard status hint /
  auto-pinning message changes (`49014ee`, `7163508`) — none of these CLI message
  strings are documented in any `CLAUDE.md`; `projects/kwaai-compute/CLAUDE.md`'s shard
  section only documents the explicit `--start-block`/`--blocks` flags, which are
  unaffected.
- The installer/README and inference reinstall-hint fixes (`5504c76`, `76ab7e1`) touch
  only `README.md` and inline CLI strings, not memory files.

Worth naming even though it's not a memory-file issue: `7163508`'s own commit message
and `e7c581d`'s own commit message both flag a real, still-open inconsistency between
the two — the auto-pinning hint added in `7163508` tells the operator to run
`kwaainet config set start_block <n>`, but `e7c581d` (merged the same day) makes that
exact command refuse while a shard is running. The author already tracked this in both
commit messages as a known rough edge, not blocked on; it isn't stale memory, just an
open code loose end for a future commit.

## Accretion — what could be removed and why

**1. `core/crates/kwaai-storage/CLAUDE.md:3,15` — false claim, high confidence, should
be fixed now, not just noted.**
Says the crate implements "multi-tenant homomorphic-encrypted vector storage" and that
`src/vectors.rs` does "Homomorphic vector search." Verified against
`core/crates/kwaai-storage/src/lib.rs:5` ("no encryption: vectors are opaque `Vec<f32>`")
and `vectors.rs` (plain cosine similarity via `Hnsw<'static, f32, DistCosine>`). This is
the exact overclaim that commit `1c0b342` ("docs(storage): stop claiming homomorphic
search ships", #169, 2026-09-14) corrected — but that commit only fixed
`projects/kwaai-storage/CLAUDE.md` (which now correctly says "plaintext f32 vectors
today, encryption is planned") and three design docs; it never touched this
crate-level file, which still contradicts its own sibling doc five days later.

**2. `projects/kwaai-network/CLAUDE.md:33,62-63` — stale "current state," high
confidence.**
Lists "P2P relay routing for inference" under both "Fix needed" and "In progress /
planned." Verified this has shipped: `core/crates/kwaai-cli/src/ollama_proxy.rs`
(`resolve_inference_urls`) and `inference_mux.rs` fully implement `p2p://` and `mux://`
routing over the p2p relay. Both `projects/kwaai-compute/CLAUDE.md:24-28` and
`projects/kwaai-knowledge/CLAUDE.md:92-94` already document this as ordinary, working
infrastructure with real peer-ID examples. This file is the one lagging. (The other two
items in the same "in progress" list — intent-casting schemas and a
`kwaainet_version` DHT field — were checked and are genuinely still unimplemented; leave
those.)

**3. `plans/windows-self-update-redesign.md` — plan superseded by shipped work,
verified.**
Header reads "Status: Ready to implement," describing replacing a PowerShell-script
based updater with synchronous Rust. `core/crates/kwaai-cli/src/updater.rs` has zero
`.ps1`/PowerShell references today. The plan hasn't been touched since `3a7351c`
(2026-09-01); `updater.rs` has since been rewritten (`13515c2`, `4fed65f`, and others).
The redesign already shipped; this plan should either be marked complete or removed.

**4. Non-portable local path in `projects/kwaai-compute/CLAUDE.md:75`.**
Cites `~/.claude/plans/cached-jingling-creek.md` as the design doc for the dedicated
inference thread's session pool / LRU eviction. That path is outside the repo (under a
home directory), doesn't exist in this checkout, and can't be resolved by any other
agent or machine. Either the content should move into the repo (`projects/kwaai-compute/
plans/`) or the reference should be dropped.

**Lower-confidence, not verified either way:** `projects/kwaai-compute/CLAUDE.md` and
`projects/kwaai-knowledge/CLAUDE.md` both hardcode peer IDs for `metro-linux`,
`metro-win`, and `jerome`. I can't confirm these are dead — I have no way to probe
network liveness from here — but hardcoded peer IDs are a standing accretion risk if a
machine is ever re-keyed or replaced (line 31 of `projects/kwaai-network/CLAUDE.md`
already notes `metro-linux`'s GPU was swapped 2026-08). Flagging for the next person who
touches those machines to confirm, not asserting they're wrong now.

I also checked `projects/kwaai-compute/plans/MacOllamaStopgap-plan.md`, which lacks the
`Status: COMPLETE` marker its sibling `InferenceHostSupervisor-plan.md` has. Its
"Changes" section matches shipped code (`shard_cmd.rs:340`'s macOS gate exists as
described), but its own "Open" section lists two genuinely unresolved items (DHT
discoverability, coverage-loss monitoring) and its header already frames it as an
intentional stopgap "until an Apple fast path lands." This is a live document, not
accretion — no action needed.
