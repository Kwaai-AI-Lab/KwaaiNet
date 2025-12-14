# Debugging: Why Rust Nodes Don't Appear on map.kwaai.ai

**Date:** 2025-12-11
**Analysis Based On:** OpenAI-Petal network visibility architecture study

---

## Executive Summary

**Problem:** KwaaiNet Rust nodes do not appear on map.kwaai.ai even when running.

**Root Cause:** The DHT announcement mechanism is stubbed out. The `DhtManager` only maintains a local cache and never actually publishes records to the Kademlia DHT.

**Evidence:**
- `core/crates/kwaai-p2p/src/dht.rs:30` - `// TODO: Actually put to Kademlia DHT via swarm`
- `core/crates/kwaai-p2p/src/dht.rs:44` - `// TODO: Actually provide via Kademlia`
- No code path exists that calls `kademlia.put_record()` or `kademlia.start_providing()`

**Solution:** A working implementation exists in `core/examples/petals_visible.rs` that demonstrates the correct approach.

---

## Background: How Nodes Appear on map.kwaai.ai

### 6-Step Process (from OpenAI-Petal analysis)

```
1. Node connects to bootstrap servers (TCP)
2. p2p daemon joins Kademlia DHT
3. DHT bootstrap completes
4. Node loads model blocks
5. ✅ Node ANNOUNCES blocks to DHT ← CRITICAL STEP
6. map.kwaai.ai queries DHT and discovers node
```

**The Problem:** Step 5 never happens in the core library implementation.

### Key Insight from Python Implementation

From `OpenAI-Petal/docs/NETWORK_VISIBILITY_ARCHITECTURE.md`:

> "TCP connections can succeed while DHT announcement fails, creating a zombie state where process indicators look healthy but network visibility is lost."

**This is exactly what's happening in the Rust implementation** - the Kademlia DHT is configured and connected, but no announcements are made.

---

## Code Analysis

### Current Architecture

```
KwaaiNetwork
    │
    ├─► swarm (libp2p::Swarm<KwaaiBehaviour>)
    │     └─► kademlia (kad::Behaviour) ← DHT configured but unused
    │
    └─► dht (Arc<RwLock<DhtManager>>) ← Local cache only
```

**The Problem:** `DhtManager` and Kademlia are completely decoupled.

### What's Missing

#### 1. DHT Record Publishing

**File:** `core/crates/kwaai-p2p/src/dht.rs`

```rust
pub async fn put(&mut self, key: &str, value: Vec<u8>) -> P2PResult<()> {
    debug!("DHT put: {} ({} bytes)", key, value.len());
    self.local_cache.insert(key.to_string(), value);
    // TODO: Actually put to Kademlia DHT via swarm  ← STUB
    Ok(())
}
```

**What it should do:**
```rust
// From petals_visible.rs:357-368
let record = Record {
    key: RecordKey::new(&module_uid),
    value: info_bytes.clone(),
    publisher: Some(*peer_id),
    expires: None,
};

swarm.behaviour_mut()
     .kademlia
     .put_record(record, kad::Quorum::One)?;
```

#### 2. Provider Announcement

**File:** `core/crates/kwaai-p2p/src/dht.rs`

```rust
pub async fn provide(&mut self, key: &str) -> P2PResult<()> {
    info!("DHT provide: {}", key);
    // TODO: Actually provide via Kademlia  ← STUB
    Ok(())
}
```

**What it should do:**
```rust
// From petals_visible.rs:371-374
let key = RecordKey::new(&module_uid);
swarm.behaviour_mut()
     .kademlia
     .start_providing(key)?;
```

#### 3. Block Announcement Loop

**Missing entirely** - No code announces blocks like Petals does.

**What's needed** (from petals_visible.rs:331-398):
```rust
fn announce_to_dht(
    swarm: &mut Swarm<PetalsBehaviour>,
    model_name: &str,
    peer_id: &PeerId,
    server_info: &ServerInfo,
) {
    // Serialize server info to MessagePack
    let info_bytes = server_info.to_msgpack()?;

    // Announce each block: {model_name}.{block_idx}
    for block_idx in server_info.start_block..server_info.end_block {
        let module_uid = format!("{}.{}", model_name, block_idx);

        // Put DHT record
        let record = Record {
            key: RecordKey::new(&module_uid),
            value: info_bytes.clone(),
            publisher: Some(*peer_id),
            expires: None,
        };
        swarm.behaviour_mut().kademlia.put_record(record, kad::Quorum::One)?;

        // Start providing
        swarm.behaviour_mut().kademlia.start_providing(RecordKey::new(&module_uid))?;
    }

    // Also announce model metadata
    let model_metadata_key = format!("_petals.models.{}", model_name);
    // ... put model metadata record
}
```

#### 4. Heartbeat Re-announcement

**Missing** - DHT records expire after ~10-15 minutes if not refreshed.

**What's needed** (from petals_visible.rs:177-196):
```rust
// Re-announce every 4 minutes to keep records alive
let mut heartbeat_interval = tokio::time::interval(Duration::from_secs(240));

loop {
    tokio::select! {
        _ = heartbeat_interval.tick() => {
            if bootstrap_done {
                announce_to_dht(&mut swarm, &model_name, &local_peer_id, &server_info);
            }
        }
        // ... handle swarm events
    }
}
```

#### 5. RPC Handler for Health Monitor

**Partially implemented** - The RPC codec exists but isn't integrated into `KwaaiNetwork`.

**File:** `core/crates/kwaai-p2p/src/rpc.rs` ✅ (Good)
**File:** `core/crates/kwaai-p2p/src/network.rs` ❌ (Not using RPC)

**What's needed:**
- Add `rpc: request_response::Behaviour<HivemindCodec>` to `KwaaiBehaviour`
- Handle `SwarmEvent::Behaviour::Rpc` events
- Respond to health monitor `rpc_info` requests

---

## Comparison: Working Example vs Library

### Working Example (petals_visible.rs)

```rust
#[derive(NetworkBehaviour)]
struct PetalsBehaviour {
    kademlia: kad::Behaviour<MemoryStore>,
    identify: identify::Behaviour,
    rpc: request_response::Behaviour<HivemindCodec>,  ← RPC handler
}

// After DHT bootstrap completes:
announce_to_dht(&mut swarm, &model_name, &local_peer_id, &server_info);  ← Announces!

// Periodic heartbeat:
tokio::time::interval(Duration::from_secs(240))  ← Keeps records alive

// RPC event handling:
SwarmEvent::Behaviour(PetalsBehaviourEvent::Rpc(
    request_response::Event::Message { peer, message },
)) => {
    let response = rpc_handler.handle_request(request);  ← Responds to health monitor
    swarm.behaviour_mut().rpc.send_response(channel, response)?;
}
```

### Library (network.rs)

```rust
#[derive(SwarmBehaviour)]
pub struct KwaaiBehaviour {
    pub kademlia: kad::Behaviour<MemoryStore>,
    pub identify: identify::Behaviour,
    pub kwaai: KwaaiProtocol,  ← Custom protocol, not Hivemind RPC
}

// DHT operations delegated to DhtManager:
async fn put(&mut self, key: &str, value: Vec<u8>) -> P2PResult<()> {
    let mut dht = self.dht.write().await;
    dht.put(key, value).await  ← Goes to stub!
}

// No announcement logic
// No heartbeat
// No RPC handling
```

**Conclusion:** The library has the infrastructure (Kademlia, Identify) but not the application logic (announcing, heartbeat, RPC).

---

## Critical Missing Components

### 1. DHT Announcement Logic ❌

**Status:** Not implemented
**Location:** Should be in `network.rs` or a new `announcer.rs` module
**Dependencies:**
- Access to `swarm.behaviour_mut().kademlia`
- ServerInfo (block range, public_name)
- Model name

**Reference:** `petals_visible.rs:331-398`

### 2. Heartbeat Timer ❌

**Status:** Not implemented
**Location:** Should be in the swarm event loop
**Purpose:** Re-announce every 4 minutes to prevent DHT record expiration

**Reference:** `petals_visible.rs:177-196`

### 3. RPC Integration ❌

**Status:** Partially implemented (codec exists, not integrated)
**Location:** `network.rs` needs to add RPC to `KwaaiBehaviour`

**What exists:**
- ✅ `HivemindCodec` (rpc.rs)
- ✅ `RpcHandler` (rpc.rs)
- ✅ `ServerInfo` (hivemind.rs)

**What's missing:**
- ❌ RPC added to `KwaaiBehaviour`
- ❌ RPC event handling in swarm loop
- ❌ `RpcHandler` instantiation and integration

**Reference:** `petals_visible.rs:123, 281-300`

### 4. Bootstrap Sequence ❌

**Status:** Bootstrap() is called, but no follow-up announcement
**Location:** `network.rs:201-228` calls `kademlia.bootstrap()` but doesn't wait for completion

**What's needed:**
```rust
SwarmEvent::Behaviour(KwaaiBehaviourEvent::Kademlia(
    kad::Event::OutboundQueryProgressed {
        result: kad::QueryResult::Bootstrap(Ok(stats)),
        ..
    },
)) => {
    // Bootstrap complete - NOW announce!
    announce_to_dht(...);
}
```

**Reference:** `petals_visible.rs:227-248`

### 5. DHT Key Format ❌

**Status:** Unknown if implemented correctly
**Required format:** `{model_name}.{block_index}` (e.g., "Llama-3.3-70B-Instruct.0")

**Also needed:**
- Model metadata key: `_petals.models.{model_name}`

**Reference:** `petals_visible.rs:352-389`

---

## How map.kwaai.ai Discovers Nodes

### Query Pattern

Based on the OpenAI-Petal health monitor analysis:

1. **map.kwaai.ai queries DHT** for known model keys:
   ```
   _petals.models.Llama-3.3-70B-Instruct
   _petals.models.Llama-3.1-8B-Instruct
   ... (all tracked models)
   ```

2. **Finds providers** for each model via Kademlia `GET_PROVIDERS`

3. **Queries each node** via Hivemind RPC `/hivemind/0.0.0/rpc`:
   ```
   Request: ExpertUID { uid: "" }  // General server info
   Response: ExpertInfo { serialized_info: msgpack(ServerInfo) }
   ```

4. **Parses ServerInfo** to extract:
   - `public_name` (displayed on map)
   - `state` ("online", "joining", "offline")
   - `start_block` / `end_block`
   - `throughput`, `inference_rps`

5. **Aggregates into /api/v1/state** response

### Why Rust Nodes Are Invisible

**Missing Step 1:** No DHT records published → map.kwaai.ai finds nothing

Even if we published records:
- **Missing Step 3:** No RPC handler → map.kwaai.ai can't query ServerInfo

**Result:** Node never appears in `model_reports[].server_rows[]` array

---

## Testing Strategy

### Phase 1: Test DHT Announcement (Working Example)

```bash
cd /Users/rezarassool/Source/KwaaiNet/core

# Build and run the working example
cargo build --release --example petals_visible

# Run with a unique name
./target/release/examples/petals_visible \
  --name "rust-test-$(date +%s)" \
  --model "Llama-3.3-70B-Instruct" \
  --port 31337

# Expected output:
# [CONNECTED] to Petals network via QmXXX...
# [DHT] Bootstrap complete! 5 peers in routing table
# [ANNOUNCE] Announcing node to DHT...
#   [DHT] Announced module: Llama-3.3-70B-Instruct.0
#   [DHT] Announced module: Llama-3.3-70B-Instruct.1
#   ...
#   [DHT] Announced model metadata: _petals.models.Llama-3.3-70B-Instruct
# [STATUS] Node is now announcing itself to the Petals DHT.
```

**Verification:**
```bash
# Wait 2-3 minutes for map.kwaai.ai to scrape DHT
curl -s https://map.kwaai.ai/api/v1/state | \
  jq '.model_reports[] | select(.short_name == "Llama-3.3-70B-Instruct") | .server_rows[] | select(.span.server_info.public_name | contains("rust-test"))'
```

**If this works:** ✅ Proves the example is correct
**If this fails:** ❌ Need to debug DHT key format or bootstrap servers

### Phase 2: Integrate into Library

**Goal:** Make `KwaaiNetwork::announce_blocks()` work

**Steps:**
1. Refactor `DhtManager` to take `&mut Swarm` or use channels
2. Implement actual Kademlia operations
3. Add announcement logic to network startup
4. Add heartbeat timer
5. Integrate RPC handler

**Test:**
```rust
let mut network = KwaaiNetwork::new(config).await?;
network.start().await?;
network.bootstrap(bootstrap_peers).await?;

// After bootstrap completes:
let server_info = ServerInfo::new("test-node").with_span(0, 8);
network.announce_blocks("Llama-3.3-70B-Instruct", server_info).await?;

// Verify on map.kwaai.ai after 2-3 minutes
```

### Phase 3: Verify Health Monitor Queries

**Test RPC handler:**
```bash
# Run node
cargo run --release --example petals_visible --name "rpc-test"

# From another terminal, query the node directly
# (Requires libp2p-based RPC client tool)
# For now, rely on map.kwaai.ai health monitor to query us

# Check logs for:
# [RPC] Received request from QmXXX...
# [RPC] Responding with server info
```

**Verification:**
- Node appears on map with correct `public_name`
- `state` shows "online"
- `blocks` shows correct range

---

## Implementation Checklist

### Minimal Viable Fix (Make One Node Visible)

- [ ] **Copy `petals_visible.rs` approach into `KwaaiNetwork`**
  - [ ] Add `announce_blocks()` method
  - [ ] Take `&mut Swarm` or refactor to access Kademlia
  - [ ] Implement DHT record publishing
  - [ ] Implement provider announcement

- [ ] **Add RPC to KwaaiBehaviour**
  - [ ] Add `rpc: request_response::Behaviour<HivemindCodec>` field
  - [ ] Handle RPC events in swarm loop
  - [ ] Instantiate `RpcHandler` with ServerInfo

- [ ] **Add heartbeat timer**
  - [ ] Create `tokio::time::Interval` (240s)
  - [ ] Re-announce on each tick

- [ ] **Test with `petals_visible` example**
  - [ ] Verify node appears on map.kwaai.ai
  - [ ] Verify health monitor can query RPC
  - [ ] Verify node stays visible for >10 minutes (heartbeat working)

### Production-Ready Implementation

- [ ] **Error handling**
  - [ ] Handle DHT operation failures gracefully
  - [ ] Retry announcement if bootstrap incomplete
  - [ ] Log warnings if map.kwaai.ai unreachable

- [ ] **Configuration**
  - [ ] Make heartbeat interval configurable
  - [ ] Make DHT replication factor configurable
  - [ ] Support multiple models per node

- [ ] **Monitoring**
  - [ ] Expose metrics (announcements sent, RPC requests handled)
  - [ ] Health check endpoint
  - [ ] Graceful shutdown (announce "offline" state)

- [ ] **Testing**
  - [ ] Unit tests for DHT key generation
  - [ ] Integration tests with mock DHT
  - [ ] E2E test with real Petals network

---

## Key Differences: Rust vs Python Implementation

| Aspect | Python (Petals/Hivemind) | Rust (KwaaiNet) |
|--------|-------------------------|-----------------|
| **DHT Library** | Hivemind (custom) | libp2p Kademlia |
| **Announcement** | Automatic (Petals handles it) | Manual (must call `put_record`) |
| **RPC Protocol** | Built-in to Hivemind | Must implement with request-response |
| **Heartbeat** | Automatic (Petals reannounces) | Manual (must run timer) |
| **Bootstrap** | Automatic (Hivemind connects) | Manual (must wait for event) |
| **p2pd daemon** | Separate process (Go) | Embedded (libp2p-rs) |

**Implication:** The Rust implementation requires more explicit management of the announcement lifecycle.

---

## Debugging Tools

### 1. Check DHT Records (from another Petals node)

```python
# Python REPL on a working Python node
from hivemind import DHT

dht = DHT(initial_peers=["/ip4/18.219.43.67/tcp/8000/p2p/QmQhRuhe..."])
await dht.get("Llama-3.3-70B-Instruct.0")  # Should return ServerInfo bytes
```

### 2. Monitor Network Traffic

```bash
# Capture DHT queries on port 8000
sudo tcpdump -i any -nn 'tcp port 8000' -X

# Look for Kademlia FIND_NODE, FIND_VALUE, STORE messages
```

### 3. Check map.kwaai.ai API

```bash
# See all visible nodes
curl -s https://map.kwaai.ai/api/v1/state | jq

# Check specific model
curl -s https://map.kwaai.ai/api/v1/state | \
  jq '.model_reports[] | select(.short_name == "Llama-3.3-70B-Instruct")'

# Check bootstrap health
curl -s https://map.kwaai.ai/api/v1/state | jq '.bootstrap_states'
```

### 4. Verify libp2p Kademlia Behavior

```bash
# Enable libp2p debug logs
RUST_LOG=libp2p_kad=debug,libp2p_swarm=debug cargo run --example petals_visible

# Look for:
# - "Routing table updated"
# - "Storing record"
# - "Started providing"
```

---

## Expected Timeline

### Immediate (1-2 hours)
- ✅ Run `petals_visible` example
- ✅ Verify it appears on map.kwaai.ai
- ✅ Understand working implementation

### Short-term (1 day)
- 🔄 Refactor `DhtManager` to access Kademlia
- 🔄 Implement `announce_blocks()` method
- 🔄 Add RPC to `KwaaiBehaviour`
- 🔄 Test with minimal example

### Medium-term (1 week)
- 🔄 Add heartbeat timer
- 🔄 Error handling and retries
- 🔄 Integration tests
- 🔄 Documentation

---

## Related Files

### Working Example
- `core/examples/petals_visible.rs` ← **THE REFERENCE IMPLEMENTATION**

### Library Files (Need Fixing)
- `core/crates/kwaai-p2p/src/dht.rs` ← Stubbed DHT operations
- `core/crates/kwaai-p2p/src/network.rs` ← Missing announcement logic
- `core/crates/kwaai-p2p/src/hivemind.rs` ← ServerInfo structure ✅
- `core/crates/kwaai-p2p/src/rpc.rs` ← RPC handler ✅

### Reference Documentation
- `../OpenAI-Petal/docs/NETWORK_VISIBILITY_ARCHITECTURE.md` ← How Python version works
- `../OpenAI-Petal/ROOT_CAUSE_ZOMBIE_STATE_AFTER_SWARM_REBALANCE.md` ← Why announcements matter

---

## Conclusion

**The Rust node doesn't appear on the map because the DHT announcement step is not implemented in the core library.**

The `petals_visible.rs` example shows this CAN work in Rust - it just needs to be integrated into `KwaaiNetwork`.

**Next Steps:**
1. Run `petals_visible` example to prove the concept works
2. Refactor library to expose Kademlia operations
3. Integrate announcement logic into `KwaaiNetwork`
4. Add heartbeat and RPC handling
5. Test end-to-end on map.kwaai.ai

**Key Learning from OpenAI-Petal:** TCP connections and DHT membership are NOT sufficient for visibility. The node must actively announce its blocks and respond to RPC queries. Without this, it's in a "zombie state" - technically connected but functionally invisible.
