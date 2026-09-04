# kwaai-storage — Roadmap

## Shipped

- VPK process binding to node `PeerId`
- Multi-tenant vector tables (plaintext f32 vectors; see Planned for encryption)
- Three roles: `bob` (personal), `eve` (encrypted inference), `both`
- DHT advertising: `_kwaai.vpk.nodes` record with TTL=360s
- `kwaainet vpk enable/disable/status` commands
- `VpkInfo` struct + `vpk` field in `DHTServerInfo` wire format
- `check_vpk_health()` gating DHT announcement

## In progress

- **`vpk discover`** — query DHT for available VPK nodes; stub implemented, DHT integration pending
- PHE repo changes (separate repo): `peer_id`/`mode` in config, `tenant_id` column in DB, health endpoint additions

## Planned

- **Encrypted vectors at rest on the host** — integrate the PHE crate's ROME
  (Random Orthogonal Matrix Encryption) so Bob scrambles vectors before upload and Eve
  stores them without knowing whether they are scrambled. ROME is partially homomorphic:
  `E(v) = Q·pad(v)` with `Q` orthogonal preserves inner products exactly, so cosine
  ranking is unchanged and **the host needs no code change** — a padded vector is just a
  tenant declaring dimension `m`.

  **Threat model, from the published analysis** (Kwaai, SIAM 2025, §6 "Sacking ROME"):
  ROME falls to an actor who sees both the plaintext and the encrypted form of enough
  queries — `Q` is then recoverable by linear algebra. The VPK boundary exists to exclude
  exactly that actor: the host sees ciphertext only, never plaintext queries, and does not
  hold the embedder. State that assumption explicitly wherever this ships, because the
  guarantee rests on it. Note also that the scheme is deterministic, so equal vectors
  produce equal ciphertexts, and inner-product preservation means the host can see the
  corpus geometry even though it cannot read a vector.

- **`vpk shard`** — Phase 2: cross-node shard placement with trust-weighted, geography-aware, capacity-aware policies
- **`vpk resolve`** — Phase 3: DHT-backed knowledge base resolution for fully distributed personal AI memory
- DHT record schemas for discovering and aggregating VPK shards
- Benchmark and tune encrypted vector search pipeline on realistic workloads

## Research / future

- PHE performance optimisation for large-scale deployments (> 1M vectors)
- Homomorphic encryption tooling improvements
- Cross-node redundancy policies (n-of-m shard availability guarantees)
