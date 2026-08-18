# Circuit Pinning for `kwaainet shard api`

## Context

`kwaainet shard run` already supports `--name-filter <substr>` and `--circuit <id>` to restrict distributed inference to specific nodes. `kwaainet shard api` (the OpenAI-compatible HTTP server) has no equivalent — it rediscovers the chain fresh at startup with no filtering, so benchmark runs can't guarantee which nodes answer across requests.

This adds three flags to `shard api` mirroring what `shard run` already has:
- `--name-filter <substr>` — filter discovered chain to nodes whose `public_name` contains the substring
- `--circuit <id>` — load a pre-formed circuit (from `shard circuit create`) and use its chain snapshot instead of DHT discovery
- `--peer <peer_id>` — filter to a single explicit peer by base58 PeerId

The implementation is purely additive (no existing behaviour changes), and the filtering happens at startup before `AppState` is built — so all downstream inference logic (`run_inference`, `build_pinned_path`, peer-failure retry) works unchanged.

---

## Files to Modify

### 1. `core/crates/kwaai-cli/src/cli.rs` — `ShardApiArgs` struct (~line 887)

Add three fields after the existing `gguf_path` field:

```rust
/// Only use block servers whose public_name contains this string.
/// Matches the same filter available on `shard run`.
/// Example: --name-filter metro-linux
#[arg(long, value_name = "SUBSTR")]
pub name_filter: Option<String>,

/// Use a pre-formed circuit instead of discovering the chain fresh.
/// Create one with: kwaainet shard circuit create [--name-filter SUBSTR]
/// The server uses the circuit's chain snapshot; DHT discovery is skipped.
#[arg(long, value_name = "ID")]
pub circuit: Option<String>,

/// Pin all requests to a single peer (base58 PeerId).
/// The peer must cover the full block range or inference will fail.
#[arg(long, value_name = "PEER_ID")]
pub peer: Option<String>,
```

---

### 2. `core/crates/kwaai-cli/src/shard_api.rs` — `run()` function

The discovery section is around lines 790–806. Currently it always calls `discover_chain()`. Replace that section with conditional logic:

**Step A — chain acquisition** (replace existing `discover_chain` call):
```rust
let chain = if let Some(ref circuit_id) = args.circuit {
    // Load circuit's chain snapshot from disk (same mechanism as shard run, lines 1032-1043 of shard_cmd.rs)
    let circuit = crate::shard_cmd::load_circuit_by_id(circuit_id)?;
    let entries: Vec<BlockServerEntry> = circuit.chain.iter()
        .filter_map(|e| e.to_entry())
        .collect();
    println!("  Circuit:      {} ({} nodes, skipping DHT)", circuit.id, entries.len());
    entries
} else {
    discover_chain(&mut client, &our_peer_id, &dht_prefix, total_blocks, &bootstrap_peers).await
};
```

**Step B — apply `--name-filter`** (after chain acquisition):
```rust
let chain = if let Some(ref f) = args.name_filter {
    let filtered: Vec<_> = chain.into_iter()
        .filter(|e| e.public_name.contains(f.as_str()))
        .collect();
    if filtered.is_empty() {
        return Err(anyhow::anyhow!("--name-filter '{}' matched no block servers", f));
    }
    println!("  Name filter:  '{}' → {} nodes", f, filtered.len());
    filtered
} else {
    chain
};
```

**Step C — apply `--peer`** (after name filter):
```rust
let chain = if let Some(ref peer_str) = args.peer {
    let filtered: Vec<_> = chain.into_iter()
        .filter(|e| e.peer_id.to_base58() == peer_str.as_str())
        .collect();
    if filtered.is_empty() {
        return Err(anyhow::anyhow!("--peer '{}' not found in discovered chain", peer_str));
    }
    println!("  Peer pin:     {}", peer_str);
    filtered
} else {
    chain
};
```

The filtered `chain` then flows into `AppState` construction (lines 851–863) — no changes needed there.

**No changes to `AppState` struct or `run_inference()`** — the existing `build_pinned_path(&state.chain, ...)` automatically works on whatever filtered chain was stored.

---

## Reused functions / patterns

| Function | File | Used for |
|---|---|---|
| `load_circuit_by_id(id)` | `shard_cmd.rs:2421` | Load circuit from `~/.kwaainet/run/circuits.json` |
| `SerializableEntry::to_entry()` | `shard_cmd.rs:2355` | Convert JSON circuit entries back to `BlockServerEntry` |
| `build_pinned_path()` | `shard_cmd.rs:2292` | Already called per-request in `run_inference` — unchanged |
| `discover_chain()` | `shard_cmd.rs:1931` | Called when `--circuit` is not set — unchanged |

---

## Behaviour matrix

| Flags | Chain source | Path selection |
|---|---|---|
| (none) | Fresh DHT discovery | `build_pinned_path` per request |
| `--name-filter metro` | DHT, filtered by name | `build_pinned_path` from filtered pool |
| `--peer 12D3...` | DHT, filtered to one peer | `build_pinned_path` (single-peer pool) |
| `--circuit abc123` | Circuit's saved chain snapshot | `build_pinned_path` from circuit pool |
| `--circuit abc123 --name-filter metro` | Circuit chain, further filtered | `build_pinned_path` from doubly-filtered pool |

On peer failure, the existing retry loop in `run_inference` calls `build_pinned_path` again with the failed peer excluded — this works correctly with all filter combinations since it operates on `state.chain`, whatever was stored at startup.

---

## Verification

```bash
# 1. Build
cd core && cargo build -p kwaainet --release
cp target/release/kwaainet ~/.cargo/bin/kwaainet
codesign -s - --force ~/.cargo/bin/kwaainet

# 2. Start api with name filter (should only use metro-linux nodes)
kwaainet shard api --port 8080 --name-filter metro-linux

# 3. In another terminal — confirm which node answered (check api stdout or shard serve logs)
curl -s http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model":"llama3","messages":[{"role":"user","content":"hi"}],"max_tokens":5}' | jq .

# 4. Test --circuit workflow
kwaainet shard circuit create --name-filter metro-linux   # note the circuit ID in output
kwaainet shard api --port 8080 --circuit <ID>             # should skip DHT, show "Circuit: <ID>"

# 5. Test --peer
kwaainet shard chain                                       # find a known peer_id
kwaainet shard api --port 8080 --peer <peer_id_base58>

# 6. Test error case: filter that matches nothing
kwaainet shard api --port 8080 --name-filter nonexistent  # should exit with clear error message

# 7. Run cargo test
cd core && cargo test -p kwaai-cli 2>&1 | tail -20
```
