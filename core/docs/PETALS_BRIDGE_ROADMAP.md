# Petals Bridge Roadmap

Strategy for KwaaiNet nodes to participate in both networks (Option C).

## Current Status

✅ **Transport Layer Compatible**
- TCP/noise/yamux working
- RSA key support enabled
- Successfully connected to Petals bootstrap

✅ **DHT Compatible**
- Both use `/ipfs/kad/1.0.0`
- Can share peer discovery

✅ **Compression Compatible**
- We have BLOCKWISE_8BIT (matches Hivemind)
- We have Top-K sparse (similar to their QUANTILE_8BIT)

## Hivemind/Petals Protocol Analysis

### Message Format
```
┌─────────────────────────────────────┐
│ Length (8 bytes, big-endian)        │
├─────────────────────────────────────┤
│ Marker (1 byte): 0x00=msg, 0x01=err │
├─────────────────────────────────────┤
│ Protobuf payload                    │
└─────────────────────────────────────┘
```

### Averaging Handlers (hivemind)
| Handler | Purpose |
|---------|---------|
| `rpc_join_group` | Join an averaging group |
| `rpc_aggregate_part` | Exchange tensor parts during all-reduce |
| `rpc_download_state` | Download optimizer state from peers |

### Expert Serving Handlers (Petals)
| Handler | Purpose |
|---------|---------|
| `rpc_inference` | Single inference step with KV cache |
| `rpc_forward` | Forward pass through transformer blocks |
| `rpc_forward_stream` | Streaming forward pass |
| `rpc_backward` | Backward pass for gradients |
| `rpc_backward_stream` | Streaming backward pass |
| `rpc_push` | Push activations between servers |
| `rpc_info` | Get server metadata and load |

### Protobuf Messages
```protobuf
message Tensor {
  bytes buffer = 1;
  repeated uint32 size = 2;
  bool requires_grad = 3;
  string dtype = 4;
  CompressionType compression = 5;
  int32 chunks = 6;
}

enum CompressionType {
  NONE = 0;
  MEANSTD_16BIT = 1;
  FLOAT16 = 2;
  QUANTILE_8BIT = 3;
  UNIFORM_8BIT = 4;
  BLOCKWISE_8BIT = 5;  // We have this!
}

message ExpertRequest {
  string uid = 1;
  repeated Tensor tensors = 2;
  bytes metadata = 3;
}

message ExpertResponse {
  repeated Tensor tensors = 1;
  bytes metadata = 2;
}
```

## Implementation Phases

### Phase 1: Protocol Foundation (Week 1-2)
- [ ] Add `hivemind.proto` to kwaai-distributed
- [ ] Implement Hivemind message framing (8-byte length + marker)
- [ ] Create `HivemindCodec` for libp2p request-response
- [ ] Map our `QuantizedTensor` ↔ Hivemind `Tensor`

### Phase 2: Averaging Compatibility (Week 3-4)
- [ ] Implement `rpc_join_group` handler
- [ ] Implement `rpc_aggregate_part` handler
- [ ] Implement `rpc_download_state` handler
- [ ] Test averaging with Hivemind Python client

### Phase 3: Expert Serving (Week 5-6)
- [ ] Implement `rpc_info` handler
- [ ] Implement `rpc_forward` handler
- [ ] Implement `rpc_inference` handler
- [ ] Register as Petals server in DHT

### Phase 4: Full Integration (Week 7-8)
- [ ] Implement streaming handlers
- [ ] Add backward pass support
- [ ] Performance optimization
- [ ] Multi-model support

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      KwaaiNet Node                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────────┐          ┌─────────────────┐          │
│  │  KwaaiNet       │          │  Petals/Hivemind │          │
│  │  Protocols      │          │  Protocols       │          │
│  │                 │          │                  │          │
│  │ /kwaai/tensor/* │          │ rpc_forward      │          │
│  │ /kwaai/avg/*    │          │ rpc_inference    │          │
│  │                 │          │ rpc_join_group   │          │
│  └────────┬────────┘          └────────┬─────────┘          │
│           │                            │                    │
│           └──────────┬─────────────────┘                    │
│                      │                                      │
│           ┌──────────▼──────────┐                          │
│           │  Candle Inference   │                          │
│           │  Engine             │                          │
│           └─────────────────────┘                          │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│                     libp2p Transport                        │
│              (TCP/noise/yamux + Kademlia DHT)               │
└─────────────────────────────────────────────────────────────┘
```

## Benefits

1. **Instant Network Access**: Join existing Petals network with thousands of nodes
2. **Shared DHT**: Discover both KwaaiNet and Petals peers
3. **Incremental Migration**: Petals users can try KwaaiNet features
4. **Fallback**: If one protocol fails, try the other
5. **Rust Performance**: Faster inference than Python nodes

## Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| Protocol drift | Pin to specific Hivemind version, monitor changes |
| Performance overhead | Lazy protocol initialization |
| Complexity | Clean abstraction layer between protocols |
| Testing | Comprehensive integration tests with Python clients |

## Quick Win: Bootstrap Integration

Even before full protocol support, we can:

```rust
// Use Petals bootstrap servers for faster discovery
const PETALS_BOOTSTRAP: &[&str] = &[
    "/ip4/159.89.214.152/tcp/31337/p2p/QmedTaZXmULqwspJXz44SsPZyTNKxhnnFvYRajfH7MGhCY",
];

// Add to our default bootstrap list
config.bootstrap_peers.extend(PETALS_BOOTSTRAP);
```

This gives us access to their DHT immediately.

## Next Steps

1. **Immediate**: Add Petals bootstrap to default config
2. **This week**: Set up protobuf compilation for Hivemind messages
3. **Next sprint**: Implement `rpc_info` as proof of concept
