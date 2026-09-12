# Memory Reflection — 2026-09-12

Scope: default branch, HEAD `e66dd62273f367fbee4a18ad4cf4607bbc489926` (v0.7.0 release).
Commits reviewed: 21, `git log --since="36 hours ago"` (2026-09-10 20:23 to 2026-09-11 19:21, all times as committed).

## Mechanical — checker output, verbatim

```
Checking 13 memory files

0 broken reference(s), 0 drift warning(s) across 13 files
```

Clean. `scripts/check_memory_integrity.py` is present on this branch and passing.

## Semantic — claims now false, with file, line, and the commit responsible

**1. `projects/kwaai-network/CLAUDE.md`, project-scope line ("kwaai-network owns the P2P
transport fabric: ... Yamux stream multiplexing, **the p2pd daemon**, RPC protocol...")**

Commit `4fed65f` ("chore: drop the Go p2pd daemon — every node is native (#191)", landed
2026-09-09, inside this window's release but before the 36h cutoff for the commit itself —
its effects are what v0.7.0 ships) removed the Go p2pd daemon outright. That same commit
edited two sibling memory files to reflect it:
- root `CLAUDE.md`: `kwaai-p2p-daemon` → `kwaai-p2p-daemon (control-socket protocol)` in the
  project table, to distinguish the surviving Rust control-socket crate from the removed
  Go binary.
- `core/crates/kwaai-p2p/CLAUDE.md`: dropped "and the p2pd daemon" from the crate's scope
  description entirely.

`projects/kwaai-network/CLAUDE.md` — the file both of those point to as "full project
context" — was not touched by that commit and still reads "the p2pd daemon" unqualified in
its own scope line, the same phrasing the other two files were specifically edited to
remove. A reader of this file alone has no way to know the daemon it names was deleted;
the crate that remains (`kwaai-p2p-daemon`) is a different thing (control-socket protocol
only, per the now-correct root table).

Fix: reword to match root `CLAUDE.md`'s "kwaai-p2p-daemon (control-socket protocol)", or
drop "the p2pd daemon" the way `core/crates/kwaai-p2p/CLAUDE.md` did.

**2. `projects/kwaai-network/CLAUDE.md`, lines ~33 and ~63**

- Line 33: "Fix needed: p2p relay routing for inference requests (user intent: 'use p2p
  relay')"
- Line 63 ("In progress / planned"): "P2P relay routing for inference (route through p2p
  network instead of direct TCP)"

This is shipped, not planned. `core/crates/kwaai-cli/src/shard_cmd.rs` implements
`mux://PEER_ID` and `p2p://PEER_ID` (including `p2p://auto` DHT-based peer selection,
`shard_cmd.rs:2428` on) for exactly this purpose, and the sibling file
`projects/kwaai-compute/CLAUDE.md` already documents it as shipped infrastructure ("GPU
inference machines (via p2p relay — no IP/DNS needed)... `mux://` is preferred"). The
v0.7.0 release commit `e66dd62` in this window is direct field verification: it reports
metro-win, behind NAT, going from advertising zero DHT addresses on v0.6.7 (the exact
failure this "fix needed" line describes) to advertising five and completing a real
"remote RAG chat completion" / inference run over the relay on this build.

I could not pin the exact commit that first shipped the p2p/mux inference routing itself
(both files were last touched together on 2026-09-04, before this window, so the code
likely predates the 36h cutoff) — flagging this as a stale status label rather than a
commit-introduced inversion. Confidence: the code and the sibling file both contradict the
"in progress" status; I'm not fully certain when it flipped, only that it has.

## Accretion — what could be removed and why

- Same two lines as semantic finding #2 above should either be deleted from "In progress /
  planned" or moved to "Shipped" (there is no "Shipped" list in this file today, unlike
  `kwaai-compute/CLAUDE.md`, which does have one and already lists it there).
- No hardcoded paths, peer IDs, or superseded plan documents found stale enough to flag with
  evidence in the 12 CLAUDE.md files or GLOSSARY.md — the peer IDs in
  `projects/kwaai-compute/CLAUDE.md` (metro-linux/metro-win/jerome) are real infrastructure
  references with no evidence of having changed, so left alone.

## Nothing found

- `core/crates/kwaai-inference/CLAUDE.md`, `core/crates/kwaai-storage/CLAUDE.md`,
  `core/crates/kwaai-trust/CLAUDE.md`, `projects/kwaai-storage/CLAUDE.md`,
  `projects/kwaai-trust/CLAUDE.md`, `projects/kwaai-knowledge/CLAUDE.md`, and
  `projects/kwaai-knowledge/GLOSSARY.md` describe crates/domains none of this window's 21
  commits touched (confirmed via `git log --since="36 hours ago" --name-only`); no claim in
  them was checked against new code because none could plausibly have been invalidated by
  this window's changes.
- `core/crates/kwaai-p2p/CLAUDE.md` and root `CLAUDE.md` were already corrected as part of
  `4fed65f` (#191) and remain accurate.
- The `only_global_ips` (#190), IPv6 (#186), and `log_level` (#202) changes landed this
  window but none of the 13 memory files make a claim about their specific defaults, so
  nothing to invalidate there.
