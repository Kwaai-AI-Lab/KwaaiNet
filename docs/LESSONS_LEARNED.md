# Lessons Learned: KwaaiNet Node Operations

## Incident: Map API Overload (2025-12-11)

### What Happened

The map.kwaai.ai service was overloaded and crashed due to excessive API requests from multiple simultaneously running node instances.

### Root Cause

1. **Multiple Node Instances**: 13+ Petals server instances were running simultaneously on the same machine
2. **Auto-Restart Service**: A launchd service (`ai.kwaai.kwaainet`) was automatically restarting failed processes
3. **Port Conflict**: All instances were attempting to use port 8080
4. **DHT Announcement Spam**: Each node was announcing itself to the DHT every 4 minutes (heartbeat)
5. **Health Monitor Overload**: The map.kwaai.ai health monitor attempted to query all discovered nodes repeatedly

**Result**: `13+ nodes × periodic announcements × health monitor queries = API overload and service crash`

### Technical Details

- **Launchd Service**: `ai.kwaai.kwaainet` (PPID = 1)
- **Process Tree**:
  ```
  launchd (PID 1)
    └── kwaainet.runner
        └── petals.cli.run_server (×13+)
            └── p2pd (DHT daemon)
  ```
- **DHT Behavior**: Each node announces blocks to DHT with keys like `Llama-3.3-70B-Instruct.{0-7}`
- **Heartbeat Interval**: 240 seconds (4 minutes)

### How to Prevent This

#### 1. Run Only ONE Node at a Time

```bash
# Before starting a new node, verify no others are running
ps aux | grep -E "(kwaai|petals|p2pd)" | grep -v grep

# If any are found, stop them first
killall -9 python
launchctl remove ai.kwaai.kwaainet 2>/dev/null
```

#### 2. Use Unique Ports for Each Node

```bash
# Good: Each test uses a different port
cargo run --example petals_visible -- --port 8081 --name "Test-Node-1"

# Bad: Multiple nodes on same port
cargo run --example petals_visible -- --port 8080 --name "Node-1" &
cargo run --example petals_visible -- --port 8080 --name "Node-2" &  # CONFLICT!
```

#### 3. Wait Between Node Restarts

DHT records take time to expire. When stopping a node and starting a new one:

```bash
# Stop the node
# Wait 5+ minutes for DHT records to expire
# Start the new node
```

#### 4. Monitor Before Checking the Map

Let the node run for at least 5-10 minutes before checking map.kwaai.ai:

```bash
# Start node
cargo run --example petals_visible -- --port 8081 --name "My-Node"

# Monitor logs for these indicators:
# - [CONNECTED] to Petals network
# - [DHT] Bootstrap complete
# - [ANNOUNCE] Announcing node to DHT
# - [DHT] Announced module: [model].[block]
# - [DHT] Now providing: [module_uid]

# Wait 5-10 minutes, then check map.kwaai.ai
```

#### 5. Check for Auto-Restart Services

```bash
# macOS: Check for launchd services
launchctl list | grep -i kwaai

# If found, remove it during testing
launchctl remove ai.kwaai.kwaainet

# Linux: Check for systemd services
systemctl list-units | grep -i kwaai
```

### Best Practices for Node Testing

1. **Single Node Rule**: Always run exactly ONE node per machine during testing
2. **Unique Identifiers**: Use unique names and ports for each node
3. **Gradual Scaling**: Test with 1 node, then 2-3, before scaling to production
4. **Monitor Resources**: Watch CPU, memory, and network usage
5. **Clean Shutdown**: Always stop nodes gracefully before starting new ones
6. **Rate Limiting Awareness**: Be mindful that discovery services have rate limits

### Commands Reference

#### Start a Single Node (Rust)
```bash
cd /Users/rezarassool/Source/KwaaiNet/core
cargo run --release --example petals_visible -- \
  --port 8081 \
  --name "My-Node-Name" \
  --model "Llama-3.3-70B-Instruct"
```

#### Check Running Nodes
```bash
ps aux | grep -E "(kwaai|petals|p2pd)" | grep -v grep
```

#### Stop All Nodes
```bash
# Kill all Python-based nodes
killall -9 python

# Remove auto-restart service
launchctl remove ai.kwaai.kwaainet 2>/dev/null

# Verify all stopped
ps aux | grep -E "(kwaai|petals|p2pd)" | grep -v grep
```

#### Verify Clean State
```bash
# No processes running
ps aux | grep -E "(kwaai|petals|p2pd)" | grep -v grep | wc -l
# Should output: 0

# No launchd service
launchctl list | grep -i kwaai
# Should output: (empty)
```

### Recovery Steps After API Overload

1. **Stop all nodes immediately** (see commands above)
2. **Wait 10-15 minutes** for DHT records to expire
3. **Contact service operator** if the map remains unresponsive
4. **Test with a single node** once service recovers
5. **Monitor carefully** for the first 30 minutes

### Related Documentation

- [petals_visible.rs](/core/examples/petals_visible.rs) - Node announcement example
- [ARCHITECTURE.md](/ARCHITECTURE.md) - System architecture
- [P2P Configuration](/core/crates/kwaai-p2p/src/config.rs) - Bootstrap and DHT settings

---

**Date**: 2025-12-11
**Status**: Production Incident - Resolved
**Impact**: map.kwaai.ai temporarily unavailable
**Resolution**: All nodes stopped, launchd service removed
