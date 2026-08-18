# KwaaiNet project workspaces

Each directory here is one **domain workspace**: a VS Code multi-root workspace plus the docs for
that domain. Open the `.code-workspace` file to get the crates, the relevant `kwaai-cli` sources,
the project docs, and the tests for that domain in a single window.

## The four pillars

The node architecture has four pillars — **Trust**, **Compute**, **Storage**, **Network** — plus a
cross-cutting platform layer. Pillars are capabilities; workspaces group crates. See the root
`CLAUDE.md` for the full mapping, including the two places where they deliberately diverge.

| Workspace | Pillar | Crates |
|-----------|--------|--------|
| `kwaai-trust` | Trust | kwaai-trust, kwaai-ledger, kwaai-wasm |
| `kwaai-compute` | Compute | kwaai-inference, kwaai-compression, kwaai-distributed |
| `kwaai-storage` | Storage | kwaai-storage |
| `kwaai-knowledge` | Storage | kwaai-rag |
| `kwaai-network` | Network | kwaai-p2p, kwaai-p2p-daemon, kwaai-hivemind-dht, kwaai-rpc, kwaai-network-tests |
| `kwaai-platform` | cross-cutting | kwaai-cli (`kwaainet`), map-server |

Every crate under `core/crates/` belongs to exactly one workspace. A new crate is not finished
until it is claimed here.

## Required structure

Every workspace has the same layout. Keep it that way — the root stays to four markdown files, and
anything longer-lived goes in `design/` or `plans/`.

```
projects/{project}/
  {project}.code-workspace   VS Code multi-root workspace
  CLAUDE.md                  scope, crate ownership, build & test, current state, do-not list
  requirements.md            what this domain must do
  roadmap.md                 phased plan
  TODO.md                    active checklist
  design/
    overview.md              architecture
    data-flows.md            sequence / data-flow diagrams
  plans/                     dated plans, proposals, reports, charts
```

Plans use descriptive names (`DreamRAG-plan.md`, `redb-to-sqlite-migration-plan.md`), never
auto-generated ones, and they live here in git rather than under `~/.claude/plans/` so they
accumulate as long-term history.

## `.code-workspace` folder order

```
1. each owned crate            ../../core/crates/{crate}
2. kwaai-cli sources           ../../core/crates/kwaai-cli/src   (named with the modules it owns)
3. any pillar-specific extra   e.g. ../../.github/workflows for kwaai-platform
4. project docs                .
5. tests                       ../../tests/{project}
```

`settings` is identical everywhere (`rust-analyzer.linkedProjects` → `../../core/Cargo.toml`,
`cargo.features` → `all`). `extensions.recommendations` carries `rust-lang.rust-analyzer` and
`tamasfe.even-better-toml` everywhere, plus at most one domain-specific extra.

## Adding a crate

1. Create `core/crates/{crate}/` and add it to the workspace `Cargo.toml`.
2. Add `core/crates/{crate}/CLAUDE.md` — one paragraph on what it does, a `**Full project
   context:**` line naming its project, a key-source-files table, gotchas, and build commands.
3. Add a row to the owning project's **Crate ownership** table in `projects/{project}/CLAUDE.md`.
4. Add a folder entry to `projects/{project}/{project}.code-workspace`.
5. Add it to the **Project map** in the root `CLAUDE.md`.
