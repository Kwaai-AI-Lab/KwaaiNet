# Running a bootstrap seed

A bootstrap seed is not a separate program or a mode. It is an ordinary
KwaaiNet node whose configuration says "serve the DHT, announce nothing,
dial nobody". Anything a seed does, a node can do — the difference is
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

Without these, the process is a normal node, not a seed.

| key | seed value | default | why |
|-----|-----------|---------|-----|
| `native_p2p` | `true` | `false` | The seed exists only on the native Rust stack. |
| `announce_self` | `false` | `true` | Publishes none of its own records — no blocks, no `_petals.models`, no `_kwaai.inference.nodes`, no VPK entry — and writes no `state = -1` tombstone on exit. It stores and serves what *other* peers publish, while appearing on the map as nothing at all. |
| `dht_server` | `true` | `false` | Kademlia server mode from t=0, before any external address is confirmed. Without it the seed waits on a reachability probe it does not need. |
| `identity_key` | path to the key file | *(none)* | **Load, never generate.** A seed's peer ID is pinned into every node's bootstrap multiaddr, so a fresh identity silently orphans the network. Setting this selects the load-don't-generate path: a missing file is an error, not a new identity. |
| `initial_peers` | `[]` | public seeds | A seed dials nobody. The empty list is honoured only because `announce_self` is false; a *node* with no peers still falls back to the public bootstraps. |

### Strongly recommended

| key | seed value | default | why |
|-----|-----------|---------|-----|
| `announce_addr` | the reachable multiaddr | *(none)* | What identify reports to peers — the `ANNOUNCE_MADDRS` analogue. Declared, so it outranks AutoNAT and is confirmed at t=0 rather than after a probe round. Without it the seed advertises only its container-internal address. |
| `port` | `8000` | `31337` | The listen port. The address built is `/ip4/0.0.0.0/tcp/<port>`. |
| `no_relay` | `false` | `false` | Keep the circuit relay hop service **on**: NATed nodes reserve circuits through the bootstraps. Set `true` only to deliberately withhold relay. |

### Independent keys worth setting

Nothing about being a seed changes what these mean, and no code derives them
from the keys above — they are listed because the answer is usually obvious for
a machine deployed at a fixed address, not because a seed requires them.

| key | usual seed value | default | why |
|-----|-----------------|---------|-----|
| `enable_upnp` | `false` | `true` | A host at a known, already-reachable address has no gateway to ask, and SSDP multicast from a datacentre subnet is noise. Set it to whatever is true of the host; a seed behind a home router may well want it left on. |

### Turn off what a seed has no use for

A seed reaches no model download, shard spawn, map fetch or health poll. These
are skipped rather than run and discarded:

| key | seed value | default |
|-----|-----------|---------|
| `health_monitoring.enabled` | `false` | `true` |
| `vpk_enabled` | `false` | `true` |
| `ollama_manage` | `false` | `true` |

`model` and `blocks` keep their defaults and are never read — they feed only the
announce record, which `announce_self = false` short-circuits first.

## Example

```yaml
native_p2p: true
announce_self: false
dht_server: true
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

In-memory, matching `run_dht`. A restarted seed refills from the nodes'
300 s re-announce loop; nothing on disk needs preserving except the identity key.
