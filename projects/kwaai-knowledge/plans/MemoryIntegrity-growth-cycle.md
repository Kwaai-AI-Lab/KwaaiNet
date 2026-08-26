# Can pai-seed's growth cycle keep our memory honest?

2026-08-26. Prompted by [`Kwaai-AI-Lab/pai-seed`](https://github.com/Kwaai-AI-Lab/pai-seed).

**Short answer: half of it, and not the half we most need.** The growth cycle is
the right model for *accretion* — documents that swell and are never pruned. Our
worse problem is *staleness*, and that has a different cause and needs a
different mechanism. Both are worth adopting; only one of them is pai-seed's.

---

## 1. What pai-seed proposes

A Personal AI grown for one Principal, structured as a tree: **roots** (model
providers), **trunk** (shared session/artifact infrastructure), **boughs** (life
domains), **branches** (current working context), **leaves** (insights and
actions, which fall and feed growth).

The engine is a per-session rhythm — **opening** (orient), **working**,
**closing** (reflect on what worked and what caused friction) — with the closing
reflection named as "the engine of growth" and never to be skipped. Governing it
is the **Bonsai Principle**: *"grow what you need to accomplish more of what you
are trying to do. No more."* Before adding a term or a process, ask whether its
absence has caused real friction, repeatedly.

Three artifacts emerge from use: a **glossary** ("the most important artifact"),
**processes** (written only after repeated friction), and **session notes**.
Renaming, restructuring and pruning are explicitly framed as growth, not failure.

## 2. Where the analogy holds

Our repository already has the shape.

| pai-seed | KwaaiNet |
|---|---|
| Roots | model providers, p2p inference peers, Ollama |
| Trunk | root `CLAUDE.md`, the build, `~/.claude/.../memory/` |
| Boughs | the six `projects/kwaai-*` domains |
| Branches | feature branches and active plans |
| Leaves | commits, plan documents, `tests/*/results/` |

Three of its claims land directly:

- **The glossary is the most important artifact.** We built one yesterday for
  independent reasons and immediately found it was the thing that made a
  reading order possible. pai-seed treats this as the central artifact rather
  than a nicety; that is the stronger position and we should adopt it.
- **Pruning is growth.** Our `CLAUDE.md` files only ever accrete. The D6 rebuild
  command in `projects/kwaai-knowledge/CLAUDE.md` carries three hardcoded p2p
  peer IDs; nothing has ever removed anything from that file.
- **Write a process only after repeated friction.** We have the inverse habit —
  the plans directory has 30+ documents, several never actioned.

## 3. Where it does not reach — and this is the important part

**A Personal AI's artifacts describe its Principal. A repository's `CLAUDE.md`
describes code.**

The Principal's vocabulary and priorities change only when the Principal
changes, and the Principal is in the room at the closing reflection. Code changes
without anyone opening the document. There is no session boundary at which a
refactor announces that it has invalidated a file map three directories away.

Measured across our twelve tracked `CLAUDE.md` files:

```
core/crates/kwaai-rag/CLAUDE.md          94 days of drift
projects/kwaai-platform/CLAUDE.md        94 days
core/crates/kwaai-p2p/CLAUDE.md          93 days
core/crates/kwaai-inference/CLAUDE.md    54 days
core/crates/kwaai-storage/CLAUDE.md      54 days
```

("Drift" = the code the file describes has been edited this many days more
recently than the file itself.)

And the drift is not merely age. **10 broken references across 5 files**:

- `core/crates/kwaai-p2p/CLAUDE.md` maps `src/network.rs` and `src/hivemind.rs`.
  Neither exists; the crate was refactored into `behaviour.rs`, `service.rs`,
  `dht_service.rs`.
- `projects/kwaai-storage/CLAUDE.md` cites `src/api/mod.rs` — the module was
  flattened to `api.rs` — and `src/mcp/server.rs`, gone.
- **The root `CLAUDE.md`, loaded into every single session, names `trust.rs` in
  its project map. There is no such file anywhere in the repository.**

Yesterday produced a semantic version of the same failure that no path check
would catch: `projects/kwaai-knowledge/CLAUDE.md` said *"Do not re-enable
relation extraction for 8B models — precision is too low."* That predates the
Phase-4 axiomatic pipeline built specifically to solve it. A contributor
following it would have avoided the pipeline this whole branch depends on.

**These files are read by agents as authoritative, so a stale claim is amplified
rather than noticed.** That is the asymmetry against a personal AI, where the
Principal reads their own glossary and would spot a term they no longer use.

## 4. What we should actually do

**Adopt the reflection ritual** for accretion, and **add verification** for
staleness. They address different failure modes and neither substitutes.

### 4a. Verification — built, and now in CI

`scripts/check_memory_integrity.py`, wired as the `memory` job in `ci.yml`. It
runs in seconds with no toolchain:

- every `` `path/file.rs` `` in a memory file must resolve, and resolve *at that
  path* (a file that moved is reported as moved);
- every `` `fn_name()` `` must exist in the crates;
- a doc untouched for 60+ days while its subject kept moving is a **warning**,
  not a failure — a prompt to re-read, not a gate.

Broken references fail the build. Drift does not, because the correct response
to drift is judgement, and a red build teaches people to silence it.

This is precisely what pai-seed cannot offer: a personal AI has no compiler to
check its glossary against. We do.

### 4b. Reflection — proposed, not built

The closing-reflection habit, scoped to what a repository can honestly sustain:

- **At the end of a session that changed code**, ask the one question the
  checker cannot: *does any memory file now claim something that is no longer
  true?* Yesterday's `entity_cap` change invalidated a table row three
  directories away, and only a human-or-agent reading caught it.
- **Prune in the same commit.** The D6 rebuild command's peer IDs, the plans
  never actioned. pai-seed's framing — that removal is growth — is the
  permission this repository lacks.
- **Let the glossary be the anchor.** New vocabulary goes there first, and
  `CLAUDE.md` files reference it rather than redefining terms locally.

### 4c. What not to adopt

The full tree taxonomy. Our `projects/` directories already *are* boughs and
renaming them buys nothing — which is itself the Bonsai Principle applied to
pai-seed itself. Take the growth cycle and the restraint; leave the vocabulary.

## 5. Immediate follow-up

The checker currently fails. Ten broken references are real and should be fixed
before the `memory` job is allowed to gate merges — most urgently the root
`CLAUDE.md`'s `trust.rs`, since that file is loaded into every session.
