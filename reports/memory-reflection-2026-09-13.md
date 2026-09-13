# Memory reflection — 2026-09-13

Scope: 12 `CLAUDE.md` files + `projects/kwaai-knowledge/GLOSSARY.md` (13 files).
Window checked: `git log --since="36 hours ago"` → 13 commits, all dated
2026-09-11, from `8c5660b` to `e66dd62` (HEAD, `release(v0.7.0)`).

## Mechanical — checker output, verbatim

```
Checking 13 memory files

0 broken reference(s), 0 drift warning(s) across 13 files
```

Clean. All file/symbol references in the 13 memory files still resolve.

## Semantic — claims now false, with file, line, and the commit responsible

None of the 13 in-window commits themselves edit a memory file in a way that
leaves another memory file stale — checked directly. The only memory-file edit
in the window is `0222ea3` rewriting the root `CLAUDE.md` merging rules; it is
internally consistent and doesn't contradict anything elsewhere.

However, `e66dd62` (the release commit, in-window) is a 27-commit changelog
summary spanning back to v0.6.8, and reading it surfaced two claims that are
false right now. Both trace to commits **1–2 days older than the 36-hour
window** — flagging them because no earlier memory-reflection report exists
(this is the first run) and they'd otherwise keep aging silently:

1. **`projects/kwaai-network/CLAUDE.md:15`**
   `| kwaai-p2p-daemon | core/crates/kwaai-p2p-daemon/ | Long-running p2pd process |`
   — also line 6 ("the p2pd daemon") and line 42 ("# Run p2pd daemon").
   Invalidated by **`4fed65f`** (2026-09-09, #191, "drop the Go p2pd daemon —
   every node is native"). Verified against the crate itself:
   `core/crates/kwaai-p2p-daemon/src/lib.rs:1` now reads "The p2pd control
   protocol — both halves," i.e. a client + an in-process server used by the
   single native node, not a standalone long-running daemon. `kwaainet start
   --daemon` still exists (`cli.rs:71,229`) but starts the native node, not a
   p2pd process. The root `CLAUDE.md:162` project-map row already says
   "control-socket protocol" — it was fixed by `4fed65f`'s own diff;
   `projects/kwaai-network/CLAUDE.md` was not, and still describes the old
   architecture.

2. **`CLAUDE.md:143-153`** (root, "Workspace") gives
   `cd core && cargo build -p kwaainet --release` as a self-contained first
   build step. Since `4fed65f`+`17850a9` (2026-09-09/10, #204 added `cudarc` as
   a third patched crate), `core/Cargo.toml`'s `[patch.crates-io]`
   unconditionally points `multistream-select`, `libp2p-kad` and `cudarc` at
   local paths under `core/patches/` that exist only after
   `bash core/patches/fetch-patches.sh` runs. `core/patches/README.md` states
   this plainly: "cargo cannot parse the workspace until *every* patched
   source exists." `e66dd62`'s own commit body flags this as an "Upgrade
   note ... now required before any build" whose failure "looks unrelated" to
   its actual cause — but root `CLAUDE.md`, loaded into every session, still
   doesn't mention the step.

I'm treating these two as verified rather than suspected: both are confirmed
against current source (`lib.rs` doc comment; `Cargo.toml` patch section +
`patches/README.md`), not just against commit messages.

## Accretion — what could be removed and why

1. **Cross-file contradiction, not caused by anything recent.**
   `projects/kwaai-knowledge/CLAUDE.md:90` documents "P2P relay inference (GPU,
   preferred — no DNS/IP needed)" with three live peer IDs already used in the
   D6 rebuild command. `projects/kwaai-network/CLAUDE.md:33` ("Fix needed: p2p
   relay routing for inference requests") and `:62-64` ("In progress /
   planned: P2P relay routing for inference... route through p2p network
   instead of direct TCP") say the opposite. Both lines were added in the same
   commit, `ec51c1a` (2026-08-31, #159) — the two files have never agreed.
   `e66dd62`'s release notes now corroborate kwaai-knowledge's side ("inference
   runs on the remote A5000" over relay-discovered addresses, tested on real
   hardware) — worth deleting or rewriting the "Fix needed" / "planned" lines
   in `projects/kwaai-network/CLAUDE.md`.

2. **Unstamped, possibly-stale hardware notes.**
   `projects/kwaai-network/CLAUDE.md:31-32` — "DNS broken: resolves to
   192.168.1.1 (router)" for both metro machines, with no date. Given how much
   moved under this exact machine pair in the last two weeks (see finding
   above), this deserves a re-check or a date stamp next time anyone touches
   this file, rather than sitting as an unqualified present-tense claim
   indefinitely.

## Nothing found

Strictly within the 36-hour window itself: nothing. I grepped all 13 files for
terms tied to what actually shipped in-window (`ipv6`, `log_level`,
`global_ips`, `FIND_NODE`, `QUIC`, `trusted_relays`, `duplicate connection`,
`AutoNAT`) — zero matches. The new IPv6 support, `only_global_ips`, kad
peerstore FIND_NODE, log-level plumbing, and AutoNAT dial-back fixes from
`#186`/`#190`/`#197`/`#199`/`#202`/`#206`/`#209` aren't described in any memory
file yet, so there's nothing for these commits to have made false — they're
simply undocumented, which is a gap for whoever writes `kwaai-network`'s next
update, not drift for this reflection to flag.
