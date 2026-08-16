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
It was dropped, for two reasons worth recording so it is not re-proposed:

- **Kademlia does not need it.** The only case for declining to serve kad
  queries is a node too NATed to answer them, and kad's auto-mode reaches that
  state on its own — client until the reachability machine confirms an external
  address, server after. A key would let an operator assert what the swarm
  already observes, and get it wrong.
- **A serving switch would be half a control.** Turning off the handlers is
  invisible to placement: with `decentralized_dht` on, candidates are drawn
  from the routing table with no filter on who serves, so a non-serving node is
  still selected among the *k* nearest, its store fails, and replication
  silently degrades. Hivemind avoids this — a `client_mode` node sends an empty
  `node_info` so peers never route to it — and we have no equivalent.

Record validators are the control for unvalidated writes. A config key is not a
substitute for one, and shipping it as though it were would be worse than its
absence.
