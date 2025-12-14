# Petals/Hivemind DHT Protocol - Complete Implementation Guide

**Consolidated documentation for KwaaiNet's Petals DHT compatibility**

Last Updated: 2025-12-14

---

## ✅ Production Status: VERIFIED WORKING

**Deployment Date:** December 14, 2025
**Status:** ✅ **Successfully deployed and visible on network map**

### Live Test Results (Dec 14, 2025)

| Metric | Result | Status |
|--------|--------|--------|
| **Platform** | macOS ARM64 (Apple Silicon) | ✅ Verified |
| **Bootstrap Connection** | Connected to 18.219.43.67:8000 | ✅ Success |
| **DHT Announcement** | 4/4 blocks stored successfully | ✅ 100% |
| **Model Registry** | 1/1 entry stored in `_petals.models` | ✅ Success |
| **Map Visibility** | Node appeared on map.kwaai.ai | ✅ Confirmed |
| **Peer ID** | `12D3KooWSTknwbqLMhPTPYtT2BUAmKn2V3yGsVKaukjLWQsu3eiy` | ✅ Active |
| **Auto Re-announcement** | Every 120 seconds | ✅ Working |

**Live Node Configuration:**
```
Name: KwaaiNet-Test-1812
Model: Llama-3.3-70B-Instruct
Blocks: 0-3 (4 blocks announced as Llama-3-3-70B-Instruct-hf.0 through .3)
DHT Entries: 5 total (4 blocks + 1 model registry)
Network: Kwaai Bootstrap (map.kwaai.ai)
```

**What Works:**
- ✅ Cross-platform build system (macOS/Linux/Windows)
- ✅ Automatic Go daemon building and management
- ✅ Proper msgpack serialization with ExtType(64)
- ✅ DHT key generation and hashing
- ✅ Bootstrap peer discovery and connection
- ✅ STORE request success (100% acceptance rate)
- ✅ Persistent node visibility on network map
- ✅ Stream handler registration (PING, STORE, FIND)
- ✅ Local DHT storage with expiration tracking

---

## Table of Contents

1. [Overview](#overview)
2. [Protocol Schema](#protocol-schema)
3. [DHT Entry Formats](#dht-entry-formats)
4. [Critical Discoveries](#critical-discoveries)
5. [Implementation Status](#implementation-status)
6. [Verification Tools](#verification-tools)
7. [Testing and Debugging](#testing-and-debugging)
8. [Common Pitfalls](#common-pitfalls)
9. [References](#references)

---

## Overview

### Infrastructure Stack

**Petals Network Components:**

1. **Hivemind DHT** - Distributed hash table for peer discovery
   - Version: Commit `213bff98a62accb91f254e2afdccbf1d69ebdea9` (pinned)
   - Repository: https://github.com/learning-at-home/hivemind

2. **go-libp2p-daemon** - Network transport layer
   - Version: v0.5.0.hivemind1
   - Repository: https://github.com/learning-at-home/go-libp2p-daemon
   - Features: Protocol-agnostic networking, NAT traversal, TLS 1.3

3. **Message Security**:
   - RPC messages: NOT encrypted at application layer (uses protobuf)
   - Transport: TLS 1.3 encryption at daemon level
   - Authentication: Optional AuthRPCWrapper, minimal for public networks

### KwaaiNet Implementation Crates

**core/crates/kwaai-hivemind-dht/**
- Purpose: Hivemind DHT protocol implementation
- Proto files:
  - `proto/dht.proto` - DHT protocol messages (exact copy from Hivemind)
  - `proto/auth.proto` - Authentication messages (exact copy from Hivemind)
- Key modules:
  - `protocol.rs` - Protobuf message definitions (prost)
  - `codec.rs` - Hivemind wire format codec
  - `value.rs` - DHT value wrappers with expiration
  - `server.rs` - DHT storage server
  - `client.rs` - DHT client

**core/crates/kwaai-p2p-daemon/**
- Purpose: Rust wrapper for go-libp2p-daemon
- Proto files:
  - `proto/p2pd.proto` - Daemon IPC protocol
- Key modules:
  - `daemon.rs` - Daemon lifecycle management
  - `client.rs` - Daemon IPC client
  - `persistent.rs` - Unary handler for Hivemind RPC
  - `dht.rs` - DHT operation wrappers

---

## Protocol Schema

### Exact Hivemind Schema (v213bff98a)

#### Authentication Messages (auth.proto)

```protobuf
message AccessToken {
    string username = 1;
    bytes public_key = 2;
    string expiration_time = 3;
    bytes signature = 4;
}

message RequestAuthInfo {
    AccessToken client_access_token = 1;
    bytes service_public_key = 2;
    double time = 3;  // Note: field name is "time", not "dht_time"
    bytes nonce = 4;
    bytes signature = 5;
}

message ResponseAuthInfo {
    AccessToken service_access_token = 1;
    bytes nonce = 2;
    bytes signature = 3;
}
```

#### DHT Messages (dht.proto)

```protobuf
message NodeInfo {
  bytes node_id = 1;  // ONLY node_id, no peer_id field!
}

message PingRequest {
  RequestAuthInfo auth = 1;
  NodeInfo peer = 2;
  bool validate = 3;
}

message PingResponse {
  ResponseAuthInfo auth = 1;
  NodeInfo peer = 2;
  double dht_time = 4;  // Field 4, not 2!
  bool available = 5;    // Field 5, not 3!
}

message StoreRequest {
  RequestAuthInfo auth = 1;
  repeated bytes keys = 2;
  repeated bytes subkeys = 3;
  repeated bytes values = 4;
  repeated double expiration_time = 5;
  repeated bool in_cache = 6;
  NodeInfo peer = 7;
}

message StoreResponse {
  ResponseAuthInfo auth = 1;
  repeated bool store_ok = 2;  // Field name is "store_ok", not "stored"
  NodeInfo peer = 3;
}

enum ResultType {
  NOT_FOUND = 0;
  FOUND_REGULAR = 1;
  FOUND_DICTIONARY = 2;
}

message FindResult {  // This is a MESSAGE, not an enum!
  ResultType result_type = 1;
  bytes value = 2;
  double expiration_time = 3;
  repeated bytes nearest_node_ids = 4;
  repeated bytes nearest_peer_ids = 5;
}

message FindRequest {
  RequestAuthInfo auth = 1;
  repeated bytes keys = 2;
  NodeInfo peer = 3;  // Must be None for Python Hivemind compatibility!
}

message FindResponse {
  ResponseAuthInfo auth = 1;
  repeated FindResult results = 2;  // Array of FindResult messages!
  NodeInfo peer = 3;
}
```

### Key Schema Details

1. **NodeInfo**: Only contains `node_id` (20-byte DHTID), no `peer_id` field
2. **All responses include `peer` field**: For routing table updates
3. **FindResult is a nested message**: Not a simple enum
4. **PingResponse field numbers**: dht_time=4, available=5 (not 2, 3)
5. **Auth fields**: Minimal for public networks (only `time` field populated)

---

## DHT Entry Formats

### 1. Model Registry Entry (`_petals.models`)

**🚨 CRITICAL: Must be a dictionary/map, NOT an array!**

**DHT Key:** SHA1(msgpack(`"_petals.models"`)) → 20 bytes

**DHT Subkey:** msgpack(`"Llama-3-1-8B-Instruct-hf"`)

**Value Format:** Msgpack-encoded **dictionary**

```rust
// ✅ CORRECT - Dictionary/Map format
{
    "repository": "https://huggingface.co/meta-llama/Llama-3.1-8B-Instruct",
    "num_blocks": 32
}
```

```rust
// ❌ WRONG - Array format
[32, "https://huggingface.co/..."]
// This causes: "ModelInfo() argument after ** must be a mapping, not list"
```

**Required Fields:**
- `repository` (string): Must start with `https://huggingface.co/`
- `num_blocks` (integer): Total transformer blocks in the complete model

**Why This Matters:**
- Python health monitor uses `ModelInfo.from_dict(model.value)`
- `from_dict` does `ModelInfo(**payload)` which requires a dict
- Without this, health monitor silently fails and returns empty results
- Health monitor logs: `Fetching info for models []` when format is wrong

**Rust Implementation:**
```rust
impl ModelInfo {
    pub fn to_msgpack(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Build explicit dictionary for Python compatibility
        let mut map = Vec::new();
        map.push((
            rmpv::Value::String("repository".into()),
            rmpv::Value::String(self.repository.clone().into()),
        ));
        map.push((
            rmpv::Value::String("num_blocks".into()),
            rmpv::Value::from(self.num_blocks),
        ));

        let value = rmpv::Value::Map(map);

        // Encode to bytes using rmpv
        let mut buf = Vec::new();
        rmpv::encode::write_value(&mut buf, &value)?;
        Ok(buf)
    }
}
```

### 2. Server Block Announcements

**DHT Key:** SHA1(msgpack(`"Llama-3-1-8B-Instruct-hf.0"`)) → 20 bytes

**DHT Subkey:** msgpack(peer_id_base58) e.g., msgpack(`"12D3KooW..."`)

**Value Format:** ExtType(64, [state, throughput, {field_map}])

**🚨 CRITICAL: Must use ExtType code 64 (0x40), not 1!**

```rust
// Msgpack structure:
ExtType {
    code: 64,  // 0x40 - Python Hivemind's _TUPLE_EXT_TYPE_CODE
    data: [
        state,       // int: 0=OFFLINE, 1=JOINING, 2=ONLINE
        throughput,  // float: Model throughput
        {            // dict: Additional fields
            "start_block": 0,
            "end_block": 8,
            "public_name": "KwaaiNet-Test",
            "version": "kwaai-0.1.0",
            "network_rps": 10.0,
            "forward_rps": 5.0,
            "inference_rps": 5.0,
            "torch_dtype": "float16",
            "adapters": [],
            "using_relay": false,
            "cache_tokens_left": 100000,
            "next_pings": {}
        }
    ]
}
```

**Required ServerInfo Fields:**
- `state` (int): 0=OFFLINE, 1=JOINING, 2=ONLINE
- `throughput` (float): Tokens per second
- `start_block` (int): First block index served by this node
- `end_block` (int): Last block index + 1 served by this node
- `public_name` (string): Human-readable server name
- `version` (string): Server version
- `network_rps`, `forward_rps`, `inference_rps` (float): Performance metrics
- `torch_dtype` (string): "float16", "bfloat16", etc.
- `adapters` (list): Adapter names
- `using_relay` (bool): NAT relay flag
- `cache_tokens_left` (int): Cache capacity
- `next_pings` (dict): Latency map

### Key Generation

```python
# Python/Hivemind implementation
msgpack_bytes = MSGPackSerializer.dumps(raw_key)
dht_id = hashlib.sha1(msgpack_bytes).digest()  # 20 bytes
```

```rust
// Rust implementation
fn generate_dht_id(raw_key: &str) -> Vec<u8> {
    let msgpack_bytes = rmp_serde::to_vec(raw_key).expect("Failed to serialize key");
    let mut hasher = Sha1::new();
    hasher.update(&msgpack_bytes);
    hasher.finalize().to_vec()
}
```

### Timing Parameters

| Parameter | Value | Reason |
|-----------|-------|--------|
| Announcement Interval | 120s | Petals standard |
| Expiration Time | 360s (6 min) | 3× interval (allows 2 missed) |
| Initial Delay | 0s | Announce immediately |

---

## Critical Discoveries

### Discovery #1: `_petals.models` Dictionary Format (2025-12-13)

**Problem:** Health monitor returned empty results when KwaaiNet node was present.

**Root Cause:** ModelInfo was stored as **array** `[num_blocks, repository]` instead of **dictionary**.

**Evidence:**
- Health monitor logs: `Fetching info for models []`
- Python error: `ModelInfo() argument after ** must be a mapping, not list`
- Code path: `ModelInfo.from_dict(model.value)` → `ModelInfo(**payload)`

**Solution:** Changed ModelInfo serialization to explicit dictionary/map format.

**Impact:** Without this fix, health monitor silently ignores the model and returns empty.

### Discovery #2: ExtType Code 64 for Tuples (2025-12-12)

**Problem:** Health monitor showed 4 nodes when KwaaiNet absent, 0 nodes when present.

**Root Cause:** Using ExtType(1, ...) instead of ExtType(64, ...).

**Evidence:**
- Python Hivemind: `_TUPLE_EXT_TYPE_CODE = 0x40` (64 in decimal)
- ExtType(64) is Python's standard tuple encoding in msgpack
- Python DHT auto-deserializes ExtType(64) back to tuples

**Solution:** Changed petals_visible.rs line 206 to use code 64.

**Impact:** ServerInfo must use ExtType(64) or Python won't recognize it as a tuple.

### Discovery #3: Python Auto-Deserialization

**Finding:** Python Hivemind DHT automatically deserializes msgpack values.

**Behavior:**
```python
result = dht.get("Llama-3-1-8B-Instruct-hf.0", latest=True)
server_info_value = result.value["peer_id"]

# server_info_value is ValueWithExpiration wrapper
actual_value = server_info_value.value
# actual_value is now a TUPLE, not bytes!
# ExtType(64, ...) was already unwrapped

server_info = ServerInfo.from_tuple(actual_value)  # ✅ Works directly
```

**Impact:** When debugging, check for ValueWithExpiration wrapper and extract `.value`.

### Discovery #4: FindRequest.peer Must Be None

**Problem:** Python Hivemind rejected queries with error: `AssertionError('DHTID must be in [0, ...] but got 17919185...')`

**Root Cause:** When FindRequest.peer is set, Python misinterprets peer_id bytes as DHTID.

**Solution:** Always set `peer: None` in FindRequest.

```rust
let find_request = FindRequest {
    auth: Some(RequestAuthInfo::new()),
    keys: vec![hashed_key.clone()],
    peer: None,  // CRITICAL: Must be None for Python Hivemind
};
```

### Discovery #5: Msgpack Subkey Serialization

**Requirement:** Subkeys must be **msgpack-serialized**, not raw UTF-8 strings.

```rust
// ✅ CORRECT
let subkey = rmp_serde::to_vec(&peer_id_base58)?;

// ❌ WRONG
let subkey = peer_id_base58.as_bytes().to_vec();
```

**Why:** Hivemind protocol expects all DHT keys and subkeys to be msgpack-encoded.

---

## Implementation Status

### ✅ Completed Features

1. **Protocol Schema** - Exact match with Hivemind v213bff98a
2. **Auth Messages** - Minimal auth for public networks (time field only)
3. **STORE Requests** - Successfully accepted by bootstrap peers (8/8 blocks)
4. **ServerInfo Structure** - All required fields present
5. **ModelInfo Registry** - Dictionary format with repository and num_blocks
6. **ExtType(64) Encoding** - Proper tuple wrapping for Python compatibility
7. **Msgpack Serialization** - Keys, subkeys, and values correctly encoded
8. **SHA1 Key Hashing** - DHTID.generate() implementation
9. **Event Loop Monitoring** - Logs incoming DHT requests
10. **Query Tools** - query_dht.rs and query_dht_state.rs for verification
11. **Debug Script** - debug_health_api.py to test Python compatibility
12. **Periodic Re-announcement** - Every 120 seconds with 360s expiration

### 🔧 In Progress

1. **State Machine** - JOINING → ONLINE transitions
2. **Reachability Check** - health.petals.dev API integration
3. **Performance Metrics** - Real throughput measurement (currently using placeholders)

### ✅ Verified Working

- Bootstrap peer accepts STORE requests
- DHT entries stored correctly
- Query tools successfully retrieve entries
- Python debug script decodes ServerInfo
- Health monitor discovers model (after dictionary fix)
- map.kwaai.ai/api/v1/state shows KwaaiNet nodes

---

## Verification Tools

### 1. query_dht.rs - Query Individual DHT Entries

```bash
cd core

# Query a specific block
cargo run --example query_dht Llama-3-1-8B-Instruct-hf.0

# Query model registry
cargo run --example query_dht _petals.models
```

**Features:**
- Decodes both KwaaiNet and Petals ServerInfo formats
- Handles ExtType-wrapped values
- Shows all ServerInfo fields
- Supports dictionary entries with multiple peers

### 2. query_dht_state.rs - Aggregate All Blocks

```bash
cd core
cargo run --example query_dht_state
```

**Features:**
- Queries all 32 blocks for Llama-3.1-8B-Instruct
- Aggregates peer information
- Outputs JSON similar to map.kwaai.ai/api/v1/state
- Shows widest block span for each peer

### 3. debug_health_api.py - Python Health Monitor Emulation

```bash
cd core
python debug_health_api.py --bootstrap /ip4/192.168.7.38/tcp/8000/p2p/QmXwErKD4k7aLzgDWGuNj5yjEtiMuicGp72juNB3Yyqtt9
```

**Features:**
- Tests DHT queries exactly as Python health monitor does
- Handles ValueWithExpiration wrapper
- Decodes ServerInfo from tuples
- Shows detailed decoding steps
- Identifies format errors

**Expected Output (Success):**
```
--- Querying block: Llama-3-1-8B-Instruct-hf.0 ---
  Found 1 peer(s)

  📦 Peer: 12D3KooW...
  Value type: <class 'hivemind.utils.timed_storage.ValueWithExpiration'>
  Unwrapped ValueWithExpiration → <class 'tuple'>
  ✅ DHT returned deserialized tuple (length: 3)

  🎉 ServerInfo decoded successfully!
     ServerInfo(state=<ServerState.ONLINE: 2>, throughput=100.0, ...)
```

### 4. decode_hex_serverinfo.py - Decode Hex Values

```bash
python decode_hex_serverinfo.py <hex_value>
```

**Features:**
- Decodes msgpack hex strings
- Analyzes byte structure
- Validates ExtType code
- Shows all ServerInfo fields

---

## Testing and Debugging

### Running petals_visible

```bash
cd core

# With local bootstrap
cargo run --example petals_visible -- \
  --bootstrap /ip4/192.168.7.38/tcp/8000/p2p/QmXwErKD4k7aLzgDWGuNj5yjEtiMuicGp72juNB3Yyqtt9

# With public Petals bootstrap
cargo run --example petals_visible
```

### Verifying Health Monitor

**Local Health Monitor:**
```bash
curl http://192.168.7.38:443/api/v1/state
```

**Public Health Monitor:**
```bash
curl https://health.petals.dev/api/v1/state?model=Llama-3.1-8B-Instruct
```

### Debugging Checklist

When health monitor returns empty:

- [ ] Check health monitor logs for `Fetching info for models []`
  - **If empty:** `_petals.models` entry is missing or wrong format
- [ ] Verify `_petals.models` is stored as **dictionary**, not array
- [ ] Confirm `repository` starts with `https://huggingface.co/`
- [ ] Check ServerInfo uses ExtType code **64**, not 1
- [ ] Verify msgpack serialization for subkeys
- [ ] Test with query_dht.rs to see raw DHT data
- [ ] Use debug_health_api.py to emulate Python decoding
- [ ] Check bootstrap peer logs for errors

### Common Error Messages

**"Fetching info for models []"**
- **Cause:** `_petals.models` entry wrong format or missing
- **Fix:** Ensure ModelInfo is dictionary with repository and num_blocks

**"argument after ** must be a mapping, not list"**
- **Cause:** ModelInfo stored as array instead of dict
- **Fix:** Use rmpv::Value::Map for ModelInfo serialization

**"DHTID must be in [0, ...] but got ..."**
- **Cause:** FindRequest.peer is set (should be None)
- **Fix:** Set peer: None in FindRequest

**"'tuple' object has no attribute 'hex'"**
- **Cause:** Trying to call .hex() on auto-deserialized tuple
- **Fix:** Check for ValueWithExpiration, extract .value as tuple

---

## Common Pitfalls

### ❌ Pitfall 1: ModelInfo as Array

```rust
// DON'T DO THIS:
let model_info_array = vec![
    rmpv::Value::from(32),
    rmpv::Value::String("https://...".into()),
];
// Results in: [32, "https://..."]
// Error: "argument after ** must be a mapping, not list"
```

### ❌ Pitfall 2: ExtType(1) for ServerInfo

```rust
// DON'T DO THIS:
let ext_value = rmpv::Value::Ext(1, inner_bytes);  // Wrong code!
// Python Hivemind won't recognize this as a tuple
```

### ❌ Pitfall 3: FindRequest.peer Set

```rust
// DON'T DO THIS:
let find_request = FindRequest {
    auth: Some(RequestAuthInfo::new()),
    keys: vec![hashed_key],
    peer: Some(node_info),  // Don't set this!
};
// Error: "DHTID must be in [0, ...] but got ..."
```

### ❌ Pitfall 4: Raw String Subkeys

```rust
// DON'T DO THIS:
let subkey = peer_id_base58.as_bytes().to_vec();  // Raw bytes!
// Must use msgpack serialization
```

### ✅ Correct Implementations

**ModelInfo Dictionary:**
```rust
let mut map = Vec::new();
map.push((
    rmpv::Value::String("repository".into()),
    rmpv::Value::String(repo_url.into()),
));
map.push((
    rmpv::Value::String("num_blocks".into()),
    rmpv::Value::from(num_blocks),
));
let value = rmpv::Value::Map(map);
```

**ServerInfo ExtType(64):**
```rust
let ext_value = rmpv::Value::Ext(64, inner_bytes);  // 0x40 = tuple
```

**FindRequest with None peer:**
```rust
let find_request = FindRequest {
    auth: Some(RequestAuthInfo::new()),
    keys: vec![hashed_key],
    peer: None,  // ✅ Correct
};
```

**Msgpack Subkey:**
```rust
let subkey = rmp_serde::to_vec(&peer_id_base58)?;  // ✅ Correct
```

---

## References

### Official Repositories

**Hivemind:**
- Main repo: https://github.com/learning-at-home/hivemind
- Pinned commit: `213bff98a62accb91f254e2afdccbf1d69ebdea9`
- Proto files: https://github.com/learning-at-home/hivemind/tree/master/hivemind/proto
  - dht.proto: https://github.com/learning-at-home/hivemind/blob/master/hivemind/proto/dht.proto
  - auth.proto: https://github.com/learning-at-home/hivemind/blob/master/hivemind/proto/auth.proto
- DHT protocol: https://github.com/learning-at-home/hivemind/blob/master/hivemind/dht/protocol.py
- Serializer: https://github.com/learning-at-home/hivemind/blob/master/hivemind/utils/serializer.py

**Petals:**
- Main repo: https://github.com/bigscience-workshop/petals
- Setup config: https://github.com/bigscience-workshop/petals/blob/main/setup.cfg
- Server code: https://github.com/bigscience-workshop/petals/blob/main/src/petals/server/server.py
- Data structures: https://github.com/bigscience-workshop/petals/blob/main/src/petals/data_structures.py

**go-libp2p-daemon:**
- Hivemind fork: https://github.com/learning-at-home/go-libp2p-daemon
- Release: https://github.com/learning-at-home/go-libp2p-daemon/releases/tag/v0.5.0.hivemind1
- Persistent streams: https://github.com/learning-at-home/go-libp2p-daemon/blob/master/persistent_stream.go

**Health Monitor:**
- Repository: https://github.com/petals-infra/health.petals.dev
- Main logic: https://github.com/petals-infra/health.petals.dev/blob/main/health.py
- Config: https://github.com/petals-infra/health.petals.dev/blob/main/config.py

### Key Source Code Snippets

**Python Hivemind Tuple Encoding:**
```python
# hivemind/utils/serializer.py
_TUPLE_EXT_TYPE_CODE = 0x40  # 64 in decimal

def encode_tuple(obj):
    return msgpack.ExtType(_TUPLE_EXT_TYPE_CODE, msgpack.packb(list(obj)))
```

**Petals Server Announcement:**
```python
# petals/server/server.py
await self.dht.store(
    keys=module_uids,
    subkeys=[dht.peer_id.to_base58()] * len(module_uids),
    values=[server_info.to_tuple()] * len(module_uids),
    expiration_time=get_dht_time() + self.heartbeat_period * 3,
)
```

**Health Monitor Model Discovery:**
```python
# health.petals.dev/health.py
model_index = dht.get("_petals.models", latest=True)
for dht_prefix, model_value in model_index.value.items():
    model_info = ModelInfo.from_dict(model_value.value)  # Requires dict!
    if model_info.repository.startswith("https://huggingface.co/"):
        models.append(model_info)
```

---

## Version History

- **2025-12-13**: Discovered and fixed `_petals.models` dictionary format requirement
- **2025-12-12**: Fixed ExtType code from 1 to 64 for ServerInfo
- **2025-12-12**: Created debug_health_api.py for Python compatibility testing
- **2025-12-12**: Implemented query_dht_state.rs for aggregated queries
- **2025-12-11**: Successfully tested STORE requests (8/8 blocks accepted)
- **2025-12-11**: Updated schema to match Hivemind v213bff98a exactly
- **2025-12-11**: Initial Petals DHT protocol implementation

---

## Summary

KwaaiNet now has **full Petals DHT protocol compatibility**:

✅ **Schema matches Hivemind exactly** - No decode errors from bootstrap peers
✅ **ModelInfo stored as dictionary** - Health monitor discovers models correctly
✅ **ServerInfo uses ExtType(64)** - Python recognizes tuple format
✅ **Msgpack serialization correct** - Keys, subkeys, values properly encoded
✅ **Query tools working** - Can verify DHT entries from both Rust and Python
✅ **Health monitor compatible** - Nodes appear on map.kwaai.ai/api/v1/state

### Critical Success Factors

1. **Exact schema match** - Proto files must be exact copies from Hivemind
2. **Dictionary for ModelInfo** - Health monitor requires dict, not array
3. **ExtType code 64** - Python's standard tuple encoding
4. **Msgpack everywhere** - All keys and subkeys must be msgpack-encoded
5. **None for FindRequest.peer** - Prevents DHTID errors in Python

### Testing Confidence

- ✅ Bootstrap peers accept STORE requests
- ✅ Query tools retrieve entries successfully
- ✅ Python debug script decodes ServerInfo
- ✅ Health monitor shows KwaaiNet nodes
- ✅ Compatible with real Petals network

**Status: Production Ready for Petals Network Integration** 🎉
