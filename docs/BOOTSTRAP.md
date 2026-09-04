# Running a bootstrap node

A bootstrap node is not a separate program or a mode. It is an ordinary
KwaaiNet node whose configuration says "serve the DHT, announce nothing,
dial nobody". Anything a bootstrap does, a node can do — the difference is
entirely in `config.yaml`.

It replaces the Python `python -m petals.cli.run_dht`: an in-process libp2p
swarm that answers `rpc_ping` / `rpc_store` / `rpc_find` for other peers, runs
Kademlia in server mode, and offers a circuit relay hop service so NATed nodes
can reserve through it.

Run it like any other node:

```bash
kwaainet start          # or `run-node` for a container entrypoint (foreground)
```

## Configuration

### Required

Without these, the process is an ordinary node, not a bootstrap.

| key | bootstrap value | default | why |
|---|---|---|---|
| `native_p2p` | `true` | `false` | The Rust bootstrap exists only on the native stack. |
| `announce_self` | `false` | `true` | Publishes none of its own records — no blocks, no `_petals.models`, no `_kwaai.inference.nodes`, no VPK entry — and writes no `state = -1` tombstone on exit. It stores and serves what *other* peers publish, while appearing on the map as nothing at all. A bootstrap that announced itself would show up as an inference node offering zero blocks. |
| `identity_key` | path to the key file | *(none)* | **Load, never generate.** A bootstrap's peer ID is pinned into every other node's `initial_peers` multiaddr, so a fresh identity silently orphans the network. Setting this selects the load-don't-generate path: a missing file is an error, not a new identity. |
| `initial_peers` | `[]` | public bootstraps | A bootstrap dials nobody. The empty list is honoured only because `announce_self` is false; an ordinary node with no peers still falls back to the public bootstraps. |

### Strongly recommended

| key | bootstrap value | default | why |
|---|---|---|---|
| `announce_addr` | the reachable multiaddr | *(none)* | What identify reports to peers — the `ANNOUNCE_MADDRS` analogue. Declared, so it outranks AutoNAT and is confirmed at t=0 rather than after a probe round. Without it the node advertises only its container-internal address. |
| `port` | `8000` | `31337` | The listen port. The address built is `/ip4/0.0.0.0/tcp/<port>`. |
| `no_relay` | `false` | `false` | Keep the circuit relay hop service **on**: NATed nodes reserve circuits through the bootstraps. Set `true` only to deliberately withhold relay. |
| `ipv6` | `auto`, or `true` on a host with a delegated v6 prefix | `auto` | Whether to open `/ip6/::` listeners. `auto` runs IPv4-only when the host has no IPv6 loopback or refuses the bind; `true` makes either a startup error, which is what a dual-stack bootstrap wants — coming up v4-only and silently serving half the peers it advertises to is the failure mode. `false` opens no v6 listener and drops v6 addresses from the dial and announce sets. |
| `require_global_ips` | `true` | `true` | Reject IANA-reserved space (RFC2544, RFC5737, IPv6 unique-local and the v6 documentation prefixes) the way private ranges are rejected. Leave it on; a bootstrap serving a sealed network built on reserved space is the only reason to set `false`. |
| `max_connections` | sized to the host | `100` | The ceiling on established connections, and the only bound on how far the swarm's memory grows — idle connections are held for ten minutes. 100 suits a node that mostly dials out; a bootstrap mostly receives, and wants several hundred to low thousands. Budget roughly a megabyte per connection. Inbound is capped at 4/5 of this so the node can always still dial. |

### The kad protocol set is a build flag, not a key

There is deliberately no `kad_protocols` setting. Which Kademlia protocol
names a node serves decides whether the public IPFS DHT can absorb it —
measured at 300–600 foreign peers per bootstrap against a ~12-node real
routing table, with p2pd OOM-killed every 30–90 minutes — and every attempt to
express that as configuration turned out to be a way of getting it silently
wrong. So it is compiled in:

| build | serves |
|---|---|
| ordinary (`cargo build --release -p kwaainet`) | `/kwaai/kad/1.0.0` |
| `--features kad-multi-protocol` | `/kwaai/kad/1.0.0` **and** `/ipfs/kad/1.0.0` |

**A bootstrap needs the second one during the migration window**, and only
then. Peers that predate the kwaai name reach the network through the
bootstraps, so a bootstrap that serves the kwaai name alone cuts them off
until they upgrade. It is the one host where the IPFS-absorption exposure is
a deliberate, temporary trade. Once the fleet has moved, rebuild the
bootstraps without the feature — that is the end of the migration, and the
patched `libp2p-kad` goes with it.

The node logs its protocol set at startup (`kad protocol set`), including
whether this is a multi-protocol build, because nothing else distinguishes
the two binaries at runtime. Check it after deploying a bootstrap.

**Disable self-update on a dual-protocol bootstrap.** The release pipeline
builds no `kad-multi-protocol` artifact, and `contribute.auto_update`
defaults to `true` — so a hand-built dual bootstrap would replace itself
with the stock kwaai-only release at its next update check, silently ending
legacy bridging mid-migration. Set `contribute.auto_update: false` in its
config, or `KWAAINET_NO_AUTO_UPDATE=1` in the service environment (what the
production systemd units do): the version a bootstrap runs is the
deployment's decision, and the running binary must not overrule it.

### Independent keys worth setting

Nothing about being a bootstrap changes what these mean, and no code derives
them from the keys above — they are listed because the answer is usually
obvious for a machine deployed at a fixed address, not because a bootstrap
requires them.

| key | usual value | default | why |
|---|---|---|---|
| `enable_upnp` | `false` | `true` | A host at a known, already-reachable address has no gateway to ask, and SSDP multicast from a datacentre subnet is noise. Set it to whatever is true of the host; a bootstrap behind a home router may well want it left on. |

### Turn off what a bootstrap has no use for

It reaches no model download, shard spawn, map fetch or health poll. These are
skipped rather than run and discarded:

| key | bootstrap value | default |
|---|---|---|
| `health_monitoring.enabled` | `false` | `true` |
| `vpk_enabled` | `false` | `true` |
| `ollama_manage` | `false` | `true` |

`model` and `blocks` keep their defaults and are never read — they feed only the
announce record, which `announce_self = false` short-circuits first.

## Example

```yaml
native_p2p: true
announce_self: false
identity_key: /config/bootstrap_key1.bin
initial_peers: []
announce_addr: /dns4/bootstrap1/tcp/8000
port: 8000
no_relay: false
max_connections: 1024
vpk_enabled: false
ollama_manage: false
health_monitoring:
  enabled: false

# Independent of the above — set to whatever is true of this host.
enable_upnp: false
```

## Storage

In-memory, matching `run_dht`. A restarted bootstrap refills from the other
nodes' 300 s re-announce loop; nothing on disk needs preserving except the
identity key.

Both tiers are bounded by bytes as well as by entry count — 256 MiB primary,
64 MiB cache, and no single value above 1 MiB (`kwaai_hivemind_dht::server`).
That is the ceiling on what serving other peers' records can cost, and it is
a real ceiling rather than a nominal one: serving is not gated by record
validators, so the peer choosing what to store is whoever dialled us. Over
budget, the earliest-expiring records go first.

## What is not configurable: serving

Answering `rpc_ping` / `rpc_store` / `rpc_find` for other peers is not a config
key. Every native node does it, which is what `main` already did before this
branch — a bootstrap is not distinguished by serving, only by publishing
nothing and dialing nobody.

Worth stating plainly, because it is the same thing said two ways: there is no
record validation. A served store accepts any key from any peer that can reach
it — no signature is checked, because none is verified. The `RequestAuthInfo`
on the wire is a hivemind-compatible shape that nothing reads.

An earlier draft of this branch offered `dht_server: false` as an off-switch.
It was dropped because **a serving switch would be half a control**: turning
off the handlers is invisible to placement. With `decentralized_dht` on,
candidates come from the routing table with no filter on who serves, so a
non-serving node is still selected among the *k* nearest, its store fails, and
the loss surfaces only as a shortfall count — which names neither the peers
that refused nor the reason. The operator who flipped the switch and the
operator seeing thin replication are not connected. Hivemind avoids this: a
`client_mode` node sends an empty `node_info`, so peers never add it to their
routing tables and never route to it. We have no equivalent, and building one
is the prerequisite for re-proposing the key.

Note what is *not* the argument, since it is easy to assume: kad does **not**
fall back to client mode for an unreachable node. `NetworkConfig::dht_server`
is force-vs-auto, and every kwaainet node pins it `true`, so the auto-mode path
— client until an external address is confirmed, server after — is one no node
here takes. Serving kad is unconditional too, with no evidence-driven fallback,
which is why the dropped key would have been the only way to opt out of either.

Record validators are the control for unvalidated writes. A config key is not a
substitute for one, and shipping it as though it were would be worse than its
absence.
