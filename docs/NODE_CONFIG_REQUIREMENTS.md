# Node Configuration System - Requirements

**Created:** 2025-12-14
**Status:** Draft
**Purpose:** Define requirements for YAML-based node configuration system

---

## Overview

Create an **intelligent** configuration system for KwaaiNet nodes that:
1. **Auto-discovers** optimal settings through network analysis and hardware calibration
2. Provides **zero-configuration startup** - works out-of-the-box with smart defaults
3. Allows command-line overrides for common parameters
4. Matches the configuration patterns from OpenAI-Petal project
5. Supports future extensibility (resource limits, schedules, etc.)

### Design Philosophy: Zero-Touch Configuration

The configuration system follows a **"smart defaults, easy overrides"** approach:
- **First Run**: Node auto-discovers the most popular model, calibrates hardware, finds an available port, generates a unique name
- **Subsequent Runs**: Uses cached calibration and preferences unless explicitly overridden
- **Advanced Users**: Full YAML configuration available for fine-tuning
- **CLI Flexibility**: Common settings can be overridden via command-line arguments

---

## Functional Requirements

### FR-0: Intelligent Auto-Configuration (PRIORITY 1)

The system MUST provide intelligent defaults that "just work" without user configuration.

#### FR-0.1: Network Model Discovery
- **REQ-0.1.1**: Query DHT for `_petals.models` registry on first run
- **REQ-0.1.2**: Analyze providers per model to determine popularity
- **REQ-0.1.3**: Select most popular model (highest provider count) as default
- **REQ-0.1.4**: Cache model choice in `~/.kwaainet/discovery_cache.yaml`
- **REQ-0.1.5**: Refresh discovery cache every 24 hours or on explicit `--rediscover` flag
- **REQ-0.1.6**: Fallback to hardcoded default (`Llama-3.3-70B-Instruct`) if discovery fails
- **REQ-0.1.7**: Log discovery process: "Discovered 5 models, selected Llama-3.3-70B-Instruct (127 providers)"

**Priority**: CRITICAL - Enables zero-configuration onboarding

#### FR-0.2: Hardware Calibration System
- **REQ-0.2.1**: Implement calibration engine matching OpenAI-Petal architecture (see calibration.py)
- **REQ-0.2.2**: Detect hardware: total RAM, available RAM, GPU type (MPS/CUDA/CPU), GPU memory
- **REQ-0.2.3**: Use **quick estimation** mode by default (< 10 seconds):
  - Estimate memory per block: ~1GB for 70B models, ~500MB for 8B models
  - Calculate safe block count: `(available_memory * 0.75) / memory_per_block`
  - Apply safety margin (75% utilization for recommended, 90% for max)
- **REQ-0.2.4**: Provide three calibration presets:
  - **min**: 1-2 blocks (guaranteed stable, ~1-2GB)
  - **recommended**: 50-75% of max (balanced, default choice)
  - **max**: 85-90% of max (maximum contribution)
- **REQ-0.2.5**: Cache calibration profile in `~/.kwaainet/calibration.yaml`:
  ```yaml
  calibration:
    model: "Llama-3.3-70B-Instruct"
    total_blocks: 80
    hardware:
      total_memory: 34359738368  # 32GB
      available_memory: 25769803776  # 24GB
      gpu_type: "mps"
      architecture: "arm64"
    profiles:
      min: { blocks: 1, memory: 1073741824 }
      recommended: { blocks: 12, memory: 12884901888 }
      max: { blocks: 18, memory: 19327352832 }
    timestamp: "2025-12-14T10:30:00Z"
  ```
- **REQ-0.2.6**: Support `--calibrate [quick|full]` flag for explicit re-calibration
- **REQ-0.2.7**: Display calibration summary on first run:
  ```
  🔬 Hardware Calibration Complete:
     • RAM: 32GB (24GB available)
     • GPU: Apple M4 Pro (MPS)
     • Recommended blocks: 12 (~13GB, 75% utilization)
     • You can serve blocks 0-11 of Llama-3.3-70B-Instruct
  ```

**Priority**: HIGH - Prevents OOM errors and optimizes resource usage

#### FR-0.3: Auto-Incrementing Port Selection
- **REQ-0.3.1**: Default starting port: **8080**
- **REQ-0.3.2**: Check port availability using OS socket binding test
- **REQ-0.3.3**: If port in use, auto-increment: 8081, 8082, 8083... up to 8100
- **REQ-0.3.4**: Fail with clear error after 20 attempts: "Could not find available port in range 8080-8100"
- **REQ-0.3.5**: Log selected port: "Using port 8082 (8080-8081 were in use)"
- **REQ-0.3.6**: Save selected port to config for subsequent runs
- **REQ-0.3.7**: Support `--port <N>` override (disables auto-increment, fails if unavailable)

**Priority**: MEDIUM - Enables running multiple nodes on same machine

#### FR-0.4: Intelligent Node Naming
- **REQ-0.4.1**: Generate node name using pattern: `{username}-Rust-Node-{suffix}`
- **REQ-0.4.2**: Detect system username via `whoami` (Unix) or `$USERNAME` (Windows)
- **REQ-0.4.3**: Suffix options (in order of preference):
  1. **Hostname-based**: `{username}-Rust-{hostname}` (e.g., `rezarassool-Rust-MacBookPro`)
  2. **Port-based**: `{username}-Rust-{port}` (e.g., `rezarassool-Rust-8080`)
  3. **Random-based**: `{username}-Rust-{random_adjective}` (e.g., `rezarassool-Rust-Swift`)
- **REQ-0.4.4**: Provide adjective wordlist for random names: `["Swift", "Brave", "Keen", "Bold", "Bright", "Quick", "Wise", "Fast", "Strong", "Noble"]`
- **REQ-0.4.5**: Ensure name uniqueness by checking DHT for existing names (query `_petals.servers.{name}`)
- **REQ-0.4.6**: If name collision detected, append numeric suffix: `rezarassool-Rust-MacBookPro-2`
- **REQ-0.4.7**: Save generated name to config for subsequent runs
- **REQ-0.4.8**: Support `--name <custom>` override
- **REQ-0.4.9**: Validate name constraints: alphanumeric + hyphens, 3-64 characters, no leading/trailing hyphens

**Priority**: MEDIUM - Provides recognizable identity on network map

#### FR-0.5: Model Compatibility & Block Range Auto-Assignment
- **REQ-0.5.1**: Query DHT for model's total block count from `_petals.models.{model_name}`
- **REQ-0.5.2**: Analyze existing providers to find **under-served block ranges**:
  - Query all providers for the selected model
  - Build coverage map: `[block_0: 45 providers, block_1: 43 providers, ..., block_79: 2 providers]`
  - Identify blocks with lowest provider count (highest demand)
- **REQ-0.5.3**: Assign contiguous block range starting from least-served block:
  - If calibration recommends 12 blocks and blocks 68-79 are under-served
  - Assign: `start_block=68, end_block=80` (exclusive end)
- **REQ-0.5.4**: Fallback to start from block 0 if network data unavailable
- **REQ-0.5.5**: Support `--start-block` and `--end-block` CLI overrides
- **REQ-0.5.6**: Validate block range: `end_block - start_block <= calibrated_max_blocks`
- **REQ-0.5.7**: Log block assignment: "Assigned blocks 68-79 (12 blocks, least served on network)"

**Priority**: HIGH - Optimizes network coverage and reduces redundancy

#### FR-0.6: Adaptive Performance Defaults
- **REQ-0.6.1**: Estimate throughput based on hardware:
  - **GPU (CUDA/MPS)**: 100-200 tokens/sec for 70B models, 200-400 for 8B models
  - **CPU only**: 10-30 tokens/sec for 70B models, 30-60 for 8B models
- **REQ-0.6.2**: Set cache capacity based on available memory:
  - Reserve 20% of assigned memory for KV cache
  - Calculate: `cache_capacity = (block_memory * 0.2) / bytes_per_token`
- **REQ-0.6.3**: Auto-detect optimal data type (dtype):
  - **MPS (macOS)**: Use `float16` (best compatibility)
  - **CUDA with Ampere+**: Use `bfloat16` (better precision)
  - **CUDA older**: Use `float16`
  - **CPU**: Use `float32` (required for accuracy)
- **REQ-0.6.4**: Support `--dtype` CLI override

**Priority**: LOW - Nice-to-have optimizations

### FR-1: Configuration File Format
- **REQ-1.1**: Use YAML format for human-readable configuration
- **REQ-1.2**: Default config file location: `./node_config.yaml`
- **REQ-1.3**: Support custom config file path via `--config` flag
- **REQ-1.4**: Provide example config with comprehensive comments

### FR-2: Configuration Categories
Configuration must support the following categories:

#### Node Identity
- Node name (human-readable)
- Node version
- Listen port

#### Model Configuration
- Model name/repository
- Block range (start_block, end_block)
- Data type (float16, bfloat16, float32)
- Quantization type

#### Performance Settings
- Throughput (tokens/second)
- Cache capacity (tokens)
- RPS metrics (network, forward, inference)

#### Network Settings
- Bootstrap peers (list of multiaddrs)
- DHT settings (enable, replication factor)
- NAT traversal settings
- Relay settings
- Connection/request timeouts
- Max connections

#### DHT Configuration
- Re-announcement interval
- Expiration time
- Bootstrap timeout

#### Resource Limits
- Max memory (GB)
- Max GPU memory (GB)
- Max CPU usage (%)
- Max disk space (GB)
- Max models

#### Logging
- Log level (error, warn, info, debug, trace)
- Log file path
- Log rotation settings

#### Health Checks
- Check interval
- Timeout
- Failure threshold

#### State Management
- Initial state (OFFLINE, JOINING, ONLINE)
- Using relay flag
- Adapters list

### FR-3: Command-Line Override Priority
Command-line arguments must override YAML config:

**Priority Order:** CLI args > YAML config > Hardcoded defaults

**Supported CLI Overrides:**
- `--config <path>` - Custom config file path
- `--name <string>` - Node name
- `--model <string>` - Model name
- `--start-block <int>` - Starting block index
- `--end-block <int>` - Ending block index (exclusive)
- `--port <int>` - Listen port
- `--dtype <string>` - Data type
- `--version <string>` - Node version
- `--bootstrap <multiaddr>` - Additional bootstrap peer (can repeat)

### FR-4: Configuration Validation
- **REQ-4.1**: Validate all numeric ranges (ports, block indices, timeouts)
- **REQ-4.2**: Validate multiaddr format for bootstrap peers
- **REQ-4.3**: Validate enum values (dtype, log level, state)
- **REQ-4.4**: Provide clear error messages for invalid config
- **REQ-4.5**: Fail fast on critical validation errors
- **REQ-4.6**: Warn on non-critical validation issues

### FR-5: Default Behavior & Configuration Priority
- **REQ-5.1**: Configuration priority (highest to lowest):
  1. **CLI arguments** (`--model`, `--port`, etc.)
  2. **YAML config file** (`node_config.yaml`)
  3. **Cached auto-discovery** (`~/.kwaainet/discovery_cache.yaml`, `~/.kwaainet/calibration.yaml`)
  4. **Intelligent defaults** (network discovery, hardware calibration)
  5. **Hardcoded fallbacks** (Llama-3.3-70B-Instruct, port 8080, etc.)
- **REQ-5.2**: First-run behavior (no config file, no cache):
  1. Display: "🚀 First-time setup - auto-configuring your node..."
  2. Run network model discovery (3-5 seconds)
  3. Run hardware calibration quick mode (5-10 seconds)
  4. Generate node name and find available port
  5. Display configuration summary with save prompt
  6. Create `node_config.yaml` if user confirms
- **REQ-5.3**: Subsequent runs behavior (config exists):
  1. Load `node_config.yaml`
  2. Check cache freshness (discovery > 24h triggers refresh warning)
  3. Apply CLI overrides if any
  4. Start node with merged configuration
- **REQ-5.4**: Config file optional: Node works without `node_config.yaml` (uses cached or intelligent defaults)
- **REQ-5.5**: Explicit `--config <path>` must exist (fail if not found)
- **REQ-5.6**: Log configuration source for each setting:
  ```
  📋 Configuration loaded:
     • Model: Llama-3.3-70B-Instruct (auto-discovered)
     • Blocks: 0-12 (calibrated, recommended preset)
     • Port: 8082 (auto-selected, 8080-8081 in use)
     • Name: rezarassool-Rust-MacBookPro (auto-generated)
     • Data type: float16 (auto-detected for MPS)
  ```

---

## Non-Functional Requirements

### NFR-1: Performance
- **REQ-1.1**: Config loading performance:
  - **Cached config load**: < 100ms (reading YAML file + parsing)
  - **First-run auto-configuration**: < 20 seconds total:
    - Network discovery: 3-5 seconds (DHT queries)
    - Hardware calibration (quick mode): 5-10 seconds (estimation only)
    - Port scanning: 100-500ms (up to 20 ports)
    - Name generation: < 10ms
  - **Full calibration** (explicit `--calibrate full`): 2-5 minutes (actual model loading tests)
- **REQ-1.2**: Config parsing errors must not crash the daemon
- **REQ-1.3**: Network discovery failures must not block startup (use fallback defaults)
- **REQ-1.4**: Display progress indicators for operations > 1 second:
  ```
  🔍 Discovering network models... [████████░░] 80% (4/5 models)
  🔬 Calibrating hardware...     [██████████] 100% Done!
  ```

### NFR-2: Usability
- **REQ-2.1**: Config file must be self-documenting with inline comments
- **REQ-2.2**: Error messages must indicate which config field is invalid
- **REQ-2.3**: Support both absolute and relative paths
- **REQ-2.4**: Expand environment variables in paths (e.g., `~/.kwaainet/`)

### NFR-3: Maintainability
- **REQ-3.1**: Use Rust's type system for validation (serde validation)
- **REQ-3.2**: Configuration struct must be serializable/deserializable
- **REQ-3.3**: Support config merging (CLI + YAML + defaults)
- **REQ-3.4**: Separate config loading from business logic

### NFR-4: Compatibility
- **REQ-4.1**: Config structure should align with OpenAI-Petal patterns
- **REQ-4.2**: Must support future additions without breaking changes
- **REQ-4.3**: Use semantic versioning for config schema

---

## Technical Requirements

### TR-1: Dependencies
Required Rust crates for configuration system:
- `serde` - Serialization framework
- `serde_yaml` - YAML parsing and serialization
- `clap` - Command-line argument parsing (already in use)
- `validator` (optional) - Advanced validation rules

Additional crates for intelligent defaults (FR-0):
- `sysinfo` - Hardware detection (RAM, CPU cores, architecture)
- `tokio::net::TcpListener` - Port availability testing (already available)
- `hostname` - System hostname detection
- `rand` - Random adjective selection for node names
- `chrono` - Timestamp management for cache freshness (already in use)
- `sha1` or `blake3` - DHT key hashing (already in use via kwaai-hivemind-dht)

### TR-2: Code Structure
```
core/
├── crates/
│   └── kwaai-config/         # New crate for config management
│       ├── Cargo.toml
│       ├── src/
│       │   ├── lib.rs        # Public API
│       │   ├── node.rs       # NodeConfig struct
│       │   ├── loader.rs     # Config loading logic
│       │   ├── validator.rs  # Validation rules
│       │   ├── merge.rs      # CLI/YAML/default merging
│       │   │
│       │   ├── discovery/    # Intelligent defaults (FR-0)
│       │   │   ├── mod.rs    # Discovery module exports
│       │   │   ├── model.rs  # Network model discovery (FR-0.1)
│       │   │   ├── calibration.rs  # Hardware calibration (FR-0.2)
│       │   │   ├── port.rs   # Port auto-selection (FR-0.3)
│       │   │   ├── naming.rs # Node name generation (FR-0.4)
│       │   │   └── blocks.rs # Block range assignment (FR-0.5)
│       │   │
│       │   ├── cache/        # Configuration caching
│       │   │   ├── mod.rs
│       │   │   ├── discovery.rs  # ~/.kwaainet/discovery_cache.yaml
│       │   │   └── calibration.rs  # ~/.kwaainet/calibration.yaml
│       │   │
│       │   └── defaults/     # Fallback defaults
│       │       ├── mod.rs
│       │       └── hardcoded.rs  # Hardcoded fallback values
│       │
│       └── tests/
│           ├── config_tests.rs
│           ├── discovery_tests.rs
│           ├── calibration_tests.rs
│           └── integration_tests.rs
└── examples/
    └── petals_visible.rs     # Updated to use kwaai-config
```

### TR-3: Configuration Schema

**Rust Struct Hierarchy:**
```rust
pub struct NodeConfig {
    pub node: NodeIdentity,
    pub model: ModelConfig,
    pub performance: PerformanceConfig,
    pub network: NetworkConfig,
    pub dht: DhtConfig,
    pub resources: ResourceLimits,
    pub logging: LoggingConfig,
    pub health: HealthConfig,
    pub state: StateConfig,
}
```

### TR-4: Error Handling
- Use `Result<NodeConfig, ConfigError>` for all config operations
- Define custom `ConfigError` enum with specific error types:
  - `FileNotFound`
  - `ParseError`
  - `ValidationError`
  - `MergeError`

---

## Testing Requirements

### Unit Tests
- Parse valid YAML config
- Parse invalid YAML (malformed)
- Validate field ranges
- Test CLI override precedence
- Test default fallbacks

### Integration Tests
- Load config and start node
- Verify all settings are applied
- Test missing config file handling
- Test custom config path

---

## Documentation Requirements

### User Documentation
- [ ] Create `CONFIG.md` with full reference
- [ ] Add examples for common use cases
- [ ] Document all configuration fields
- [ ] Provide migration guide from CLI-only setup

### Developer Documentation
- [ ] API documentation (rustdoc)
- [ ] Config loading flow diagram
- [ ] Validation rule documentation

---

## Migration Strategy

### Phase 1: Foundation - YAML Config & Basic Defaults (Week 1-2)
**Goal**: Basic YAML configuration with hardcoded fallbacks

**Tasks**:
1. Create `kwaai-config` crate with basic structure
2. Implement `NodeConfig` struct hierarchy
3. Add serde YAML serialization/deserialization
4. Implement config loader with CLI override support
5. Create example `node_config.yaml` with comprehensive comments
6. Update `petals_visible` example to use config system
7. Add basic validation (port ranges, block indices, etc.)

**Deliverables**:
- ✅ YAML config file loading
- ✅ CLI argument overrides
- ✅ Hardcoded fallback defaults
- ✅ Config validation
- ⏱️ Estimated: 3-5 days

---

### Phase 2: Intelligent Defaults - Discovery & Calibration (Week 3-4)
**Goal**: Zero-configuration first-run experience with intelligent defaults (FR-0)

**Sub-Phase 2.1: Hardware Calibration (FR-0.2)**
1. Implement hardware detection using `sysinfo` crate
2. Create calibration engine with quick estimation mode
3. Implement calibration profile caching (`~/.kwaainet/calibration.yaml`)
4. Add `--calibrate` CLI command
5. Display calibration summary on first run

**Sub-Phase 2.2: Port & Naming (FR-0.3, FR-0.4)**
1. Implement port availability testing (tokio TcpListener)
2. Add auto-increment port selection (8080-8100)
3. Implement node name generation with hostname/port/random fallback
4. Add adjective wordlist for random names
5. Cache generated name in config

**Sub-Phase 2.3: Network Discovery (FR-0.1, FR-0.5)**
1. Implement DHT query for `_petals.models` registry
2. Build model popularity analyzer (provider count)
3. Implement block coverage map analyzer
4. Auto-assign least-served block ranges
5. Cache discovery results (`~/.kwaainet/discovery_cache.yaml`)
6. Add cache freshness checking (24h expiration)

**Deliverables**:
- ✅ Hardware calibration (quick mode)
- ✅ Auto-port selection
- ✅ Smart node naming
- ✅ Network model discovery
- ✅ Block range optimization
- ✅ Configuration caching
- ⏱️ Estimated: 7-10 days

---

### Phase 3: Advanced Calibration & Optimization (Week 5-6)
**Goal**: Full calibration mode and adaptive performance tuning

**Tasks**:
1. Implement full calibration mode (actual model loading tests)
2. Add binary search for optimal block count
3. Implement performance benchmarking (tokens/sec)
4. Add adaptive data type selection (FR-0.6)
5. Implement cache capacity calculation
6. Add monitoring and re-calibration triggers

**Deliverables**:
- ✅ Full calibration mode (`--calibrate full`)
- ✅ Performance benchmarking
- ✅ Adaptive settings
- ⏱️ Estimated: 5-7 days

---

### Phase 4: Production Polish & Advanced Features (Week 7+)
**Goal**: Production-ready with advanced management features

**Tasks**:
1. Resource schedules (time-based limits)
2. Multiple model support
3. Advanced health checks
4. Hot-reload configuration (SIGHUP support)
5. Web UI for config editing
6. Config migration tools

**Deliverables**:
- ✅ Hot-reload support
- ✅ Web UI integration
- ✅ Multi-model configuration
- ⏱️ Estimated: Ongoing

---

## Open Questions

1. **Q**: Should we support JSON format in addition to YAML?
   - **A**: TBD - Start with YAML only, JSON can be added later

2. **Q**: Should config changes require node restart?
   - **A**: Yes for Phase 1, hot-reload in Phase 2

3. **Q**: How to handle config schema versioning?
   - **A**: TBD - Include schema_version field in YAML

4. **Q**: Should we validate bootstrap peer connectivity during config load?
   - **A**: No - validation should be fast, connectivity check is slow

5. **Q**: Support multiple config files (base + override)?
   - **A**: Not in Phase 1, single file only

---

## Success Criteria

### Phase 1 Success Criteria (YAML Config Foundation)
The basic configuration system is complete when:
- ✅ User can run node with zero CLI args (uses hardcoded defaults)
- ✅ User can create `node_config.yaml` with all settings
- ✅ CLI args properly override YAML settings
- ✅ Invalid config provides clear error messages
- ✅ All OpenAI-Petal config patterns are supported
- ✅ Existing functionality is not broken

### Phase 2 Success Criteria (Intelligent Defaults - PRIORITY)
The intelligent auto-configuration is complete when:
- ✅ **First-Run Experience**: User runs `cargo run --example petals_visible` with ZERO arguments and:
  1. Node discovers most popular model from network (3-5 sec)
  2. Node calibrates hardware and selects optimal block count (5-10 sec)
  3. Node finds available port (8080 or next available)
  4. Node generates unique name (`rezarassool-Rust-MacBookPro`)
  5. Node displays configuration summary
  6. Node asks to save config and starts successfully
  7. Total first-run time: < 20 seconds
- ✅ **Subsequent Runs**: Node starts in < 2 seconds using cached config
- ✅ **Network Discovery**: Correctly identifies most popular model from DHT
- ✅ **Hardware Calibration**:
  - Quick mode estimates blocks within ±20% of optimal
  - Full mode accurately determines max safe blocks
  - No OOM crashes during calibration
- ✅ **Port Selection**: Successfully finds available port on machines with 8080-8085 occupied
- ✅ **Node Naming**: Generated names are unique, recognizable, and valid
- ✅ **Block Range Optimization**: Preferentially assigns under-served block ranges
- ✅ **Cache Management**: Discovery cache expires after 24 hours and refreshes automatically
- ✅ **Fallback Behavior**: System gracefully handles:
  - Network discovery failures → uses hardcoded default model
  - Calibration failures → uses conservative 1 block minimum
  - Port exhaustion → clear error message
  - DHT unavailable → proceeds with fallback defaults
- ✅ **User Experience**:
  - Progress indicators for operations > 1 second
  - Clear logging of configuration sources
  - Helpful error messages with actionable suggestions
- ✅ **Documentation**: README includes first-run experience demo

### Phase 3 Success Criteria (Advanced Features)
The advanced calibration is complete when:
- ✅ Full calibration mode works on MPS, CUDA, and CPU backends
- ✅ Performance benchmarking completes in < 5 minutes
- ✅ Adaptive settings improve performance by 10-20%

### Overall Success Metrics
- **⚡ Speed**: First run < 20s, subsequent runs < 2s
- **📊 Accuracy**: Calibration within ±20% of optimal (quick) or ±5% (full)
- **🛡️ Reliability**: Zero crashes during auto-configuration
- **👥 Usability**: 90%+ of new users successfully start node without reading docs
- **🔄 Compatibility**: Works on macOS (ARM/Intel), Linux (CUDA/CPU), Windows

---

## First-Run User Experience Example

### Scenario: User runs node for the first time with zero configuration

```bash
$ cargo run --release --example petals_visible

🚀 KwaaiNet Node - First-Time Setup

No configuration found. Let's auto-configure your node...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Step 1/4: Discovering Network Models
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🔍 Querying DHT for available models...
   • Found: Llama-3.3-70B-Instruct (127 providers)
   • Found: Llama-3.1-8B-Instruct (89 providers)
   • Found: Qwen2.5-72B-Instruct (45 providers)
   • Found: DeepSeek-V3 (23 providers)

✅ Selected: Llama-3.3-70B-Instruct (most popular, 127 providers)
   Cached to: ~/.kwaainet/discovery_cache.yaml (expires in 24h)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Step 2/4: Calibrating Hardware
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🔬 Detecting system capabilities...
   • Platform: macOS 14.6 (ARM64)
   • RAM: 32 GB total, 24 GB available
   • GPU: Apple M4 Pro (MPS backend)
   • Model size: Llama-3.3-70B-Instruct has 80 blocks

⚡ Running quick calibration (estimating memory requirements)...
   • Estimated: ~1.2 GB per block for 70B model
   • Safe capacity: 18 GB (75% of available memory)
   • Maximum blocks: 15 blocks

📊 Calibration Profiles:
   • Conservative (min): 1 block (~1.2 GB) - Guaranteed stable
   • Balanced (recommended): 12 blocks (~14.4 GB) - Optimal ← SELECTED
   • Aggressive (max): 15 blocks (~18 GB) - Maximum contribution

✅ Calibration complete! Using recommended preset (12 blocks)
   Cached to: ~/.kwaainet/calibration.yaml

💡 Tip: Run with --calibrate full for precise memory testing

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Step 3/4: Analyzing Network Coverage
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 Building block coverage map for Llama-3.3-70B-Instruct...
   • Blocks 0-20: 89-127 providers (well-served)
   • Blocks 21-60: 45-88 providers (moderate)
   • Blocks 61-79: 2-44 providers (under-served) ⚠️

🎯 Assigning optimal block range...
   • Target: 12 contiguous blocks
   • Selected: Blocks 68-79 (least served)
   • Current providers: 2-15 per block

✅ Your node will significantly improve network coverage!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Step 4/4: Finalizing Configuration
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🏷️  Node name: rezarassool-Rust-MacBookPro (auto-generated)
🔌 Port: 8080 (available)
💾 Data type: float16 (optimal for MPS)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Configuration Summary
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Model:         Llama-3.3-70B-Instruct (auto-discovered)
Blocks:        68-79 (12 blocks, ~14.4 GB) (calibrated)
Port:          8080 (auto-selected)
Node Name:     rezarassool-Rust-MacBookPro (auto-generated)
Data Type:     float16 (auto-detected)
Bootstrap:     bootstrap-1.kwaai.ai, bootstrap-2.kwaai.ai

💾 Save this configuration to node_config.yaml? [Y/n]: y

✅ Configuration saved to: ./node_config.yaml
   You can edit this file or use CLI flags to override settings.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Starting Node
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🚀 Launching KwaaiNet daemon...
🔗 Connecting to DHT network...
✅ Connected to bootstrap-1.kwaai.ai
📢 Announcing 12 blocks to DHT...
   • Llama-3.3-70B-Instruct-hf.68 ✓
   • Llama-3.3-70B-Instruct-hf.69 ✓
   • Llama-3.3-70B-Instruct-hf.70 ✓
   ... (9 more)
   • Llama-3.3-70B-Instruct-hf.79 ✓

✅ Announced 12 blocks to DHT
✅ Announced model info to _petals.models registry
📊 DHT Storage: 13 total entries, 13 valid

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Node Running
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[STATUS] Your node is live!
         Check map: https://map.kwaai.ai

         Press Ctrl+C to stop
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

**Total time: 16 seconds**

### Subsequent Run (with saved config):

```bash
$ cargo run --release --example petals_visible

📋 Loading configuration from: ./node_config.yaml

🚀 Launching KwaaiNet daemon...
🔗 Connecting to DHT network...
✅ Connected to bootstrap-1.kwaai.ai
📢 Announcing 12 blocks to DHT...
✅ Announced 12 blocks to DHT

[STATUS] Node is running!
         Check map: https://map.kwaai.ai
```

**Total time: 1.8 seconds**

---

## References

- OpenAI-Petal example_config.yaml: `../OpenAI-Petal/NodeManager/example_config.yaml`
- OpenAI-Petal config.py: `../OpenAI-Petal/config.py`
- Petals constants: `petals.constants.PUBLIC_INITIAL_PEERS`
