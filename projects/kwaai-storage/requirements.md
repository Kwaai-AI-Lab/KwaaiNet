# kwaai-storage — Requirements

## Purpose

Provide a privacy-preserving, multi-tenant vector knowledge store (VPK) bound to node identity,
supporting homomorphic-encrypted search so personal knowledge can be queried without exposing
plaintext to untrusted nodes.

## Functional requirements

1. **FR-S1** Run as a separate process bound to the node's `PeerId`.
2. **FR-S2** Support three roles: `bob` (personal), `eve` (encrypted inference), `both`.
3. **FR-S3** Provide homomorphic-encrypted vector search over multi-tenant vector tables.
4. **FR-S4** Advertise presence via DHT record `_kwaai.vpk.nodes` (subkey=peer_id, value={mode, endpoint, capacity_gb, tenant_count, vpk_version}).
5. **FR-S5** Expose `GET /api/health` returning `{status, version, peer_id, tenant_count, capacity_gb_available}`.
6. **FR-S6** Implement `kwaainet vpk enable/disable/status` commands.
7. **FR-S7** Check VPK health before DHT announcement; skip announce if health check fails.
8. **FR-S8** Multi-tenant isolation: tenants must not be able to read each other's vectors.

## Non-functional requirements

- **NFR-S1** Health check must complete in < 500ms.
- **NFR-S2** Vector search latency: < 100ms for 10k vectors at 1536 dimensions.
- **NFR-S3** Tenant isolation: a failing or compromised tenant must not affect others.
- **NFR-S4** Encrypted vectors must remain encrypted at rest and in transit.
- **NFR-S5** Targets: aarch64/x86_64 macOS, x86_64/aarch64 Linux, x86_64 Windows.

## Out of scope

- Cross-node shard placement — Phase 2, not current sprint.
- DHT-backed knowledge base resolution — Phase 3.
- Training data storage.
- The PHE service implementation (separate repo).

## Dependencies

- **kwaai-trust** — VPK binds to node `PeerId`; tenant access gated by trust credentials.
- **kwaai-network** — DHT announcement for `_kwaai.vpk.nodes`.
