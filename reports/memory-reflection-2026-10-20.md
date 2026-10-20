# Memory reflection — 2026-10-20

## Mechanical — checker output, verbatim

```
Checking 51 memory and plan files
[... 101 warning(s), all `(proposed?)` file/function references inside
projects/*/plans/*.md documents describing not-yet-built work, plus a
handful of line-number drifts in the same plans directory ...]

0 broken reference(s), 101 warning(s) across 51 files
```

All 101 warnings are pre-existing and confined to `projects/*/plans/*.md` —
none touch the 12 `CLAUDE.md` files or `GLOSSARY.md` this reflection is
scoped to. 0 broken references. Full output not reproduced verbatim above
for length; re-run `python3 scripts/check_memory_integrity.py` to see it in
full.

## Semantic — claims now false, with file, line, and the commit responsible

**Nothing found.** No commits landed in the last 36 hours — the most recent
commit (`913e0bc`, "fix(updater): don't self-update a musl or non-x86_64
host to the gnu-cuda build") is from 2026-10-14, six days before this run.
There is no recent history to check a memory claim against.

The items below were found while reading the 12 `CLAUDE.md` files end to
end for Step 3 (accretion). They are real, verified inconsistencies, but I
can't tie any of them to a specific recent commit — the squash-merge
history from the September repo restructuring (`13515c2`) makes `git blame`
on these files point at one giant landing commit rather than the change
that actually introduced the claim, so I'm reporting them as accretion
rather than guessing a responsible commit.

## Accretion — what could be removed and why

**1. Two files claim VPK vector storage is homomorphically encrypted; it
isn't, and a third file says so correctly.**
`core/crates/kwaai-storage/CLAUDE.md:3` and `:15` state "multi-tenant
homomorphic-encrypted vector storage" and "Homomorphic vector search" for
`src/vectors.rs`. `projects/kwaai-knowledge/GLOSSARY.md` §1 repeats it:
"VPK — the encrypted multi-tenant storage fabric Eve nodes run." Both are
contradicted by `projects/kwaai-storage/CLAUDE.md:6`, which correctly says
vectors are "plaintext f32 today — encryption is planned" with PHE
(Partial Homomorphic Encryption) running as a **separate, not-yet-built
service in a separate repo**. I read `kwaai-storage/src/vectors.rs` and
`db.rs` directly: plain cosine similarity over `Vec<f32>`, persisted as
little-endian bytes via SQLite — no cipher, no PHE call, anywhere in the
crate. Fix the crate `CLAUDE.md` and the glossary entry to match the
project-level doc's accurate framing, or an agent trusting the crate-level
file will tell a user VPK already encrypts data it does not.

**2. A "do not" rule in kwaai-trust is already broken by shipped code.**
`projects/kwaai-trust/CLAUDE.md:65`: "Do not store credentials unencrypted
on disk." `core/crates/kwaai-trust/src/storage.rs` stores every
Verifiable Credential as plain `serde_json::to_string_pretty` written with
`std::fs::write` — the module doc even says so ("Credentials are stored as
individual JSON files... The store is intentionally simple"). No
encryption path exists in the crate today. Either the rule is aspirational
and should say so explicitly ("not yet true — do not regress further"),
or this is a real gap worth its own issue; as worded it reads as a
guarantee that doesn't hold.

**3. kwaai-network's CLAUDE.md says p2p inference relay is still
unbuilt; it's shipped, and two sibling docs already rely on it.**
`projects/kwaai-network/CLAUDE.md:63` lists "P2P relay routing for
inference (route through p2p network instead of direct TCP)" under
"In progress / planned," and `:33` says "Fix needed: p2p relay routing for
inference requests." But `core/crates/kwaai-cli/src/ollama_proxy.rs`
already handles `p2p://`/`mux://` URL schemes (`strip_prefix("p2p://")`),
and both `projects/kwaai-compute/CLAUDE.md` and
`projects/kwaai-knowledge/CLAUDE.md` document `p2p://PEER_ID` /
`mux://PEER_ID` as the preferred, already-working `--inference-urls`
scheme, with worked examples. `kwaai-network/CLAUDE.md` is the odd one out
among its own siblings.

**4. Dangling session-local plan path.**
`projects/kwaai-compute/CLAUDE.md:75` points the "dedicated inference
thread" in-progress item at `~/.claude/plans/cached-jingling-creek.md` —
a Claude Code session-local plans path, not part of this repo. It doesn't
exist in this checkout and won't exist in any other session or machine.
Either fold a summary of that plan into the repo (e.g. under
`projects/kwaai-compute/plans/`) or drop the dangling pointer.

**5. Duplicated, hand-maintained peer-ID tables.**
The same three lab machines' libp2p peer IDs (`metro-linux`, `metro-win`,
`jerome`) are hardcoded in both `projects/kwaai-compute/CLAUDE.md` and
`projects/kwaai-knowledge/CLAUDE.md`, each with its own copy of the
`--inference-urls` example string. Not wrong today, but it's exactly the
kind of machine-specific state called out as a drift risk: if any peer is
re-keyed or retired, both files need a manual, synchronized edit. Low
priority; worth consolidating into one source (e.g. the project map in
root `CLAUDE.md`) if it starts to actually drift.

**6. Minor duplicate table row.**
`projects/kwaai-trust/CLAUDE.md` lists `kwaai-cli/src/identity.rs` twice
in its "Key source files" table (back-to-back rows with slightly
different descriptions). Harmless, but should be one row.

**7. Minor line-number drift the checker doesn't catch.**
`core/crates/kwaai-inference/CLAUDE.md:20` and
`projects/kwaai-compute/CLAUDE.md:88` both cite `shard.rs:104` for the
RoPE `broadcast_mul` fix; the actual `broadcast_mul` calls are at
`shard.rs:109-116` (same function, five lines off). Low-confidence/
cosmetic — but the root `CLAUDE.md`'s own "Reviewing PRs" section argues
for anchoring findings to symbols rather than line numbers for exactly
this reason, so these two entries are inconsistent with the repo's own
stated practice.

## Nothing found

No broken mechanical references, and no commits in the review window to
check for semantic drift (see above).
