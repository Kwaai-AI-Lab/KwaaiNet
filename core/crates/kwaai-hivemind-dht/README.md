# kwaai-hivemind-dht

Rust implementation of the Hivemind DHT protocol for KwaaiNet, compatible with the Petals distributed ML network.

## Overview

This crate provides a complete implementation of the Hivemind DHT protocol, which is used by Petals for distributed model block discovery. Unlike standard Kademlia DHT, Hivemind includes:

- **DHTValue wrapper** with expiration timestamps
- **Unified FIND RPC** that returns both values and routing information
- **Batch operations** for storing/retrieving multiple keys in one request
- **MessagePack serialization** for values
- **Protobuf wire format** for RPC messages

## Key Differences from Standard Kademlia

| Feature | Standard Kademlia | Hivemind DHT |
|---------|------------------|--------------|
| Value storage | Raw bytes | DHTValue with expiration + metadata |
| Expiration | TTL-based | Absolute timestamp with server-side validation |
| Operations | FIND_NODE + FIND_VALUE | Unified FIND (value + routing) |
| Batch support | No | Yes (multiple keys per request) |
| Caching | No | Two-tier (permanent vs cache) |
| Serialization | N/A | MessagePack for values, Protobuf for messages |

## Usage

### Creating a DHT Client

```rust
use kwaai_hivemind_dht::{HivemindDHT, DHTValue};
use libp2p::PeerId;

let peer_id = PeerId::random();
let mut dht_client = HivemindDHT::new(peer_id);
```

### Storing Values

```rust
// Single value
let key = b"Llama-3-1-8B-Instruct-hf.0".to_vec();
let value = DHTValue::with_ttl(server_info_bytes, 3600.0); // 1 hour TTL

let request_id = dht_client.store(target_peer, key, value);

// Batch store (more efficient)
let entries = vec![
    (b"model.0".to_vec(), DHTValue::with_ttl(info1, 3600.0)),
    (b"model.1".to_vec(), DHTValue::with_ttl(info2, 3600.0)),
    (b"model.2".to_vec(), DHTValue::with_ttl(info3, 3600.0)),
];

let request_id = dht_client.store_many(target_peer, entries);
```

### Retrieving Values

```rust
// Single value
let key = b"Llama-3-1-8B-Instruct-hf.0".to_vec();
let request_id = dht_client.get(peer, key);

// Batch get (unified FIND operation)
let keys = vec![
    b"model.0".to_vec(),
    b"model.1".to_vec(),
    b"model.2".to_vec(),
];

let request_id = dht_client.get_many(peer, keys);
```

### Handling Responses

```rust
use kwaai_hivemind_dht::codec::{DHTResponse, ResponseData};

match dht_client.handle_response(request_id, response)? {
    ResponseData::Store(results) => {
        for result in results {
            if result.stored {
                println!("Stored: {:?}", result.key);
            }
        }
    }
    ResponseData::Find(data) => {
        for result in data.results {
            if let Some(value) = result.value {
                if value.is_valid() {
                    println!("Found: {:?}", result.key);
                    // Deserialize value
                    let server_info: ServerInfo = value.deserialize()?;
                }
            }
        }
        // Use nearest_peers for routing
        println!("Nearest peers: {:?}", data.nearest_peers);
    }
}
```

### DHTValue Builder

```rust
use kwaai_hivemind_dht::value::DHTValueBuilder;

// From bytes
let value = DHTValueBuilder::new(bytes)
    .ttl_seconds(3600.0)
    .cached() // Mark as cache-tier (subject to LRU eviction)
    .build();

// From serializable type
let value = DHTValueBuilder::from_type(&server_info)?
    .expiration_time(1735689600.0) // Absolute timestamp
    .build();
```

## Protocol Details

### Wire Format

All RPC messages use the following format:
```
[8-byte length (big-endian)][1-byte marker][protobuf payload]
```

Protocol markers:
- `0x01` - Ping
- `0x02` - Store
- `0x03` - Find

### Protocol Version

The protocol uses libp2p StreamProtocol: `/hivemind/dht/1.0.0`

### Petals Integration

To announce in the Petals network:

```rust
// Block UIDs format: {dht_prefix}.{block_number}
let dht_prefix = "Llama-3-1-8B-Instruct-hf";

for block_num in 0..32 {
    let block_uid = format!("{}.{}", dht_prefix, block_num);
    let key = block_uid.as_bytes().to_vec();

    let value = DHTValue::serialize(&server_info, 3600.0)?;
    dht_client.store(bootstrap_peer, key, value);
}
```

## Features

- ✅ DHTValue with expiration and validation
- ✅ Unified FIND RPC (value retrieval + routing)
- ✅ Batch STORE operations
- ✅ MessagePack serialization
- ✅ Protobuf wire format
- ✅ libp2p integration
- ✅ Comprehensive tests
- ✅ Zero build-time dependencies (no protoc required)

## Architecture

```
kwaai-hivemind-dht/
├── src/
│   ├── lib.rs              # Public API
│   ├── value.rs            # DHTValue with expiration
│   ├── protocol.rs         # Protobuf messages (manual)
│   ├── codec.rs            # libp2p Codec implementation
│   ├── client.rs           # HivemindDHT client
│   └── error.rs            # Error types
└── proto/
    └── dht.proto           # Protocol specification (reference)
```

## License

MIT

## References

- [Hivemind DHT Documentation](https://dht.readthedocs.io/en/latest/)
- [Hivemind Python Implementation](https://github.com/learning-at-home/hivemind/tree/master/hivemind/dht)
- [Petals Health Monitor](https://github.com/petals-infra/health.petals.dev)
- [libp2p Request-Response](https://docs.rs/libp2p-request-response/)
