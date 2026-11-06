# Memory reflection — 2026-11-06

Checked at HEAD `e98ca3733da35f6e5dea1abb70fd45f735d2fd67` (= `origin/main`).

## Mechanical — checker output, verbatim

```
Checking 57 memory and plan files
...
0 broken reference(s), 135 warning(s) across 57 files
```

All 135 warnings are in `projects/*/plans/*.md` (proposed/forward-looking files not yet
written, or renamed files the checker's heuristics already suggest a replacement for — e.g.
`core/crates/kwaai-rag/src/schema.rs` → `doc_schema.rs`). None are broken references in the
twelve `CLAUDE.md` files or `GLOSSARY.md`. Exit code 0.

## Semantic — claims now false, with file, line, and the commit responsible

`git log --since="36 hours ago"` returns exactly two commits:

- `e98ca37` — test(rpc): compare the token file's ACE by SID, not SDDL (Windows-only auth test fix)
- `d15511b` — release(v0.7.3) — a version-bump commit (`Cargo.toml`/`Cargo.lock` only, 5 files)

Neither touches anything a `CLAUDE.md` or `GLOSSARY.md` file asserts: no file greps for
`bearer token`, `kwaainet.token`, `SDDL`, `current_user_sid`, or a tracked version number
(`0.7.2`/`0.7.3`/`0.8.0`) in any of the twelve files. Nothing found in this window.

(One stale claim was found — see Accretion below — but it predates this 36-hour window by
months, per the oldest commit this shallow clone has, so it belongs there rather than here.)

## Accretion — what could be removed and why

**`projects/kwaai-network/CLAUDE.md` contradicts itself on whether p2p inference routing is
done.** The "Current state" section lists, under "In progress / planned": *"P2P relay routing
for inference (route through p2p network instead of direct TCP)"*, and the "Metro machines"
infrastructure note says *"Fix needed: p2p relay routing for inference requests (user intent:
'use p2p relay')"*. But the same file's own "Do not" section already states it as settled
policy: *"Do not send inference requests directly over TCP to metro machines; use p2p relay."*
Checking the code confirms the feature shipped: `core/crates/kwaai-cli/src/ollama_proxy.rs`,
`inference_mux.rs`, and `capacity_lease.rs` implement `p2p://PEER_ID` and `mux://PEER_ID`
inference routing, and both `projects/kwaai-compute/CLAUDE.md` and
`projects/kwaai-knowledge/CLAUDE.md` document it as the live, preferred way to reach these
same two machines (`metro-linux`, `metro-win`) today — not as a workaround. These three files
in this shallow clone's oldest commit (`e0764ea`, 2026-09-14) already contain this code, so
the "planned"/"fix needed" framing in `kwaai-network/CLAUDE.md` has been wrong for at least
that long. Recommend deleting the "P2P relay routing for inference" planned-item line and the
"Fix needed: p2p relay routing..." line, keeping the DNS-broken notes (unverified, possibly
still true) and the `OLLAMA_HOST=0.0.0.0` note.

**`core/crates/kwaai-trust/CLAUDE.md`, Key source files table, has a duplicated row.**
`kwaai-cli/src/identity.rs` is listed twice back to back (lines 56–57), once as "`kwaainet
identity` command handler" and once as "`kwaainet identity` handler — DID, VC import/list/
verify". Copy-paste leftover; drop the first, keep the more specific second.

**Minor line-number drift, not checker-visible:** `core/crates/kwaai-inference/CLAUDE.md`
cites "RoPE fix at `shard.rs:104`" for `x1.broadcast_mul(&cos4)`; that call is now at
`shard.rs:109` (and a second one at `:116`). The claim itself is still true, just off by a
few lines — low value to fix on its own but worth folding in next time that file is touched.

**Duplicated peer-ID table, two files, currently in sync but a drift risk.** The
`metro-linux`/`metro-win`/`jerome` peer-ID table appears verbatim in both
`projects/kwaai-compute/CLAUDE.md` and `projects/kwaai-knowledge/CLAUDE.md`. They agree today,
but nothing enforces that; a future hardware change only needs one of the two edited to go
stale. Not fixing this now — no evidence either copy is wrong — but flagging it as the kind of
thing the August audit was about: two sources of truth for the same hard fact.

## Nothing found

No broken mechanical references in the twelve `CLAUDE.md` files or `GLOSSARY.md`. No
`docs/REVIEW_INTEGRITY_RETROSPECTIVE.md`-style reference rot in the root `CLAUDE.md`. Numeric
claims in `GLOSSARY.md` (KB count, eval-set sizes, entity/relation-type counts) were spot-checked
for plausibility against the code but not exhaustively recounted — flagging as unverified rather
than claiming them false: confirming them would need enumerating `ontology.rs`'s full default
type/predicate tables and the live `rag_kbs` config, which this pass didn't do.
