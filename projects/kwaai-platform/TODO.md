# kwaai-platform — TODO

## Features
- [ ] Apple code signing — Developer Program + notarization in release.yml (long-term fix for Gatekeeper)
- [ ] `kwaainet update` — self-update command (fetch and install latest release)
- [ ] RISCV target — complete `release-riscv.yml`

## Map server rewrite

The map now lives in its own repo (https://github.com/Kwaai-AI-Lab/KwaaiNetMap). The deployed one is the Python
crawler on `main` there, not the Rust rewrite parked on `map-v2`
(the live host serves `/api/v1/state`; the crate serves `/api/nodes`, which
404s). Its schema is inherited from Petals and no longer describes what a
KwaaiNet node is. Evidence below is from the live payload on 2026-08-21.

- [ ] **Rewrite the map**, ideally node-served so it moves with the
      node. Ultimately each node should discover and serve its own map.

- [ ] **Model is not a column, it is a set.** The schema is
      `model_reports[] -> server_rows[]`, so a node exists under exactly one
      `dht_prefix`. That was true for Petals block servers; it is false now. A
      Mac serving whole models through Ollama serves *every* model it holds —
      the target comes from the request body, not config — and one node was
      observed serving ten. The node/model relationship is many-to-many and the
      display has to express that.

- [ ] **Status must be honest, and today it contradicts itself.** Within a
      single row, `state: "unreachable"` sat beside
      `span.server_info.state: "online"` for the same peer. Separately,
      `announce_state()` (`config.rs:906`) returns `2` only when
      `shard_is_ready()` and `0` otherwise, and `0` reads as OFFLINE — so a Mac
      actively serving whole-model inference over `/kwaai/ollama-proxy/1.0.0`
      reports itself offline, because the vocabulary has no way to say "serving,
      just not blocks". A node that is up, reachable and answering must not
      render as offline.

- [ ] **A crawl failure must not render as an empty fleet.** On 2026-08-21 the
      map showed zero nodes while the network was healthy — `shard chain` had 9
      servers and every node was landing `32/32 stored` at both bootstraps. The
      crawler had failed silently and self-healed. The only tell was
      `update_duration`: 0.0128 s against a normal ~2.9 s, with
      `bootstrap_states` still "online" because liveness succeeded while record
      enumeration died. Surface crawl health as its own state.
      `tests/kwaai-network/map_watch.sh` now detects this.

- [ ] **State transition diagram for a node**, covering the current design and
      the roadmap — the real states (starting, reachable-via-relay, direct,
      serving blocks, serving whole models, draining, offline, tombstoned) and
      the events that move between them. The present two-value `announce_state`
      is the root of the dishonesty above; the diagram should come first and the
      wire encoding follow from it.

- [ ] **Location is misleading *and* a privacy leak.** `peer_ip_info.location`
      is geo-IP on the announced address, which for a relayed node describes the
      relay rather than the node — and `using_relay: true` was set on the rows
      carrying it. Most rows say `"unknown"`, so the column is mostly empty
      anyway. Worse, for one node it resolved a **residential** address to city,
      **zip code and lat/lon**, alongside ISP and AS. That is published on a
      public endpoint for a home machine. Note the row also carried
      `show_public_name: false` while `public_name` was served in the payload
      regardless — the privacy flag is not honoured. Treat this as the priority
      item; it is the only one on this list with a disclosure consequence.
      Tracked separately as [#135](https://github.com/Kwaai-AI-Lab/KwaaiNet/issues/135)
      so it is not blocked behind the rewrite.

- [ ] **Split `public_name` into columns.** It is currently one crammed string —
      `rezarassool-macos-aarch64/v0.6.2` — mixing operator name, platform, arch
      and version. Give them separate fields. A `version` field already exists
      (`kwaai-0.6.2`) and duplicates part of the same string in a different
      format, so the two disagree by construction.

- [ ] **Blocks is wrong for whole-model nodes and static for sharded ones.** A
      Mac that serves no blocks still reported `start_block: 0, end_block: 32`,
      because the node announces a configured range irrespective of whether a
      shard loaded. The derived `cache_tokens_left_per_block: 1562` is therefore
      fabricated. For whole-model nodes the column should not claim a range; for
      sharded nodes it should track the range actually being served and follow
      rebalancing rather than showing config.

- [ ] **Drop the Petals leftovers.** `torch_dtype`, `quant_type`, `adapters`
      and the `*_rps` fields are either always null or meaningless for a node
      serving through Ollama.

## Tests
- [ ] Installer smoke test: curl install → `kwaainet --version` + `p2pd --version` both pass
- [ ] `kwaainet setup --get-deps` smoke test on clean macOS, Linux, Windows

## Docs
- [ ] Design docs: overview.md and data-flows.md (see `design/`)
