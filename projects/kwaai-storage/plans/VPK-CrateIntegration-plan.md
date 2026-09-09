# VPK crate integration — ROME as a client-side firewall

**Status:** draft for review · **Depends on:** PHE repo (private) · **Relates to:** v1 G0·R1

## The shape of it

Bob scrambles vectors before they leave the machine. The host indexes and searches them
without knowing whether they are scrambled. Nothing in `kwaai-storage` learns about
encryption, because the transform happens entirely on the client side of the wire.

```
  Bob (trusted)                                   Eve (untrusted host)
  ┌───────────────────────────────┐               ┌──────────────────────┐
  │ text ──► embedder ──► v (n)   │               │                      │
  │                        │      │               │  kwaai-storage       │
  │                    cloak.seal │   ṽ (m)       │  HNSW + cosine       │
  │                        ▼      │ ────────────► │  over Vec<f32>       │
  │                       ṽ (m)   │               │  dimension = m       │
  │                               │ ◄──────────── │                      │
  │ doc_id + score ◄──────────────│  (id, score)  │  never decrypts,     │
  │ (score == plaintext cosine)   │               │  never can           │
  └───────────────────────────────┘               └──────────────────────┘
```

## Why the host needs no changes

Three facts already hold in `core/crates/kwaai-storage`:

1. `vector_dimension` is **per-tenant and configurable** (`api.rs:112`, default 384).
2. Vectors are opaque `Vec<f32>`; the store never interprets a coordinate.
3. Search is cosine — exact below `BRUTE_FORCE_THRESHOLD`, HNSW above (`db.rs:131`).

ROME's defining property is `⟨Q·pad(v₁), Q·pad(v₂)⟩ = ⟨v₁,v₂⟩` for orthogonal `Q`, so
cosine is preserved exactly. A sealed vector is therefore just a tenant that declared
dimension `m`. **No branch, no flag, no new field on the host.** The obliviousness is
structural rather than enforced, which is the strongest form of it.

## The boundary

A new crate `kwaai-vpk` sits between the embedder and the storage client. It depends on
`phe`; `kwaai-storage` does not, and must not.

```rust
/// Transforms embeddings into what the host stores.
///
/// The host cannot distinguish a sealed vector from a clear one, because both arrive as
/// `Vec<f32>` of the tenant's declared dimension. That is the point: obliviousness is a
/// property of the data path, not a promise the host keeps.
pub trait VectorCloak: Send + Sync {
    /// Dimension the host will see. `Clear` returns n; `Rome` returns m.
    fn host_dimension(&self, plaintext_dim: usize) -> usize;

    /// Documents on the way out, and queries on the way in — the same transform for both,
    /// which is what makes the returned scores comparable.
    fn seal(&self, v: &[f32]) -> Result<Vec<f32>, CloakError>;
}

/// Identity. Existing tenants keep working, through the same call path.
pub struct Clear;

/// ROME. Holds a seed, not a matrix — see "The key is a seed".
pub struct Rome { /* seed, n, m, lazily-derived Q */ }
```

Every caller seals unconditionally. A plaintext tenant is a `Clear` cloak, not a special
case, so there is no code path where someone forgets to encrypt.

**No `open`.** Search returns `(doc_id, score)`, and scores are already plaintext-exact.
Bob maps ids to documents he already holds. Nothing ever needs decrypting, so `Q` is a
sealing key only. Add `open` only if a read-the-vector-back endpoint appears.

## The key is a seed, not a matrix

`Q` is large — at n=384, m=768 the composed matrix is 768×384 f64 ≈ 2.4 MB. Storing that
as the key makes backup, rotation and transport awkward.

Store a **32-byte seed** and derive `Q` deterministically: seeded CSPRNG → Gaussian matrix
→ QR/Householder orthogonalisation. The key becomes the same size as any other key.

This only works if regeneration is byte-stable, so **pin the PRNG algorithm, the
orthogonalisation routine, and the traversal order**, and add a test that a fixed seed
produces a fixed `Q` checksum. If that drifts between versions, every corpus sealed by the
old build becomes unsearchable — a silent, total failure. Treat the derivation as a wire
format.

### Where the seed lives

`~/.kwaainet/vpk/<tenant_id>.seed`, mode 0600.

**Do not derive it from the node identity key.** Tempting — no new secret to back up — but
it couples corpus recoverability to identity rotation: rotate the node key and every sealed
corpus becomes unsearchable. Keep it an independent secret with its own backup story.

The credential store at `~/.kwaainet/credentials/` is plaintext JSON today, which the trust
crate's own docs already contradict. Seeds land in the same bucket; fix both together or
accept both, but do not pretend one is protected.

**Losing a seed is recoverable, and worth saying so.** Bob still holds the source
documents. A lost seed costs a re-embed and re-upload, not the corpus. That is a materially
lower stake than losing an encryption key for the only copy, and it should be stated
plainly so nobody over-engineers the key ceremony.

## Paths

**Ingest.** embed → `cloak.seal(v)` → `upsert(tenant, doc_id, ṽ)`. The tenant was created
with `vector_dimension = cloak.host_dimension(n)`.

**Query.** embed → `cloak.seal(q)` → `search(tenant, q̃, top_k)` → `[(doc_id, score)]`.
Scores are the plaintext cosine values, so existing thresholds and rerankers need no
recalibration.

**Capacity.** `m ≈ 1.5n` to `2n`, so a sealed tenant costs 1.5–2× the vector bytes of a
clear one. `tenant.rs` enforces capacity limits; they are denominated in the host dimension
and therefore already correct — but the *user-visible* quota now buys fewer documents. Say
so in the CLI.

## The risk that needs measuring, not assuming

**The paper's "100% ranking preservation" is a statement in exact arithmetic. The
integration is f32.**

PHE computes in `f64` (`Array1<f64>`); `kwaai-storage` stores `f32`. Sealing in f64 and
narrowing to f32 perturbs each coordinate by ~1e-7 relative. For near-tied neighbours —
common in a dense corpus — that can reorder results. Exact preservation is a property of
the mathematics, not of the pipeline.

This is measurable and must be measured before the feature is described as lossless. PHE
already ships the instruments in `src/vpk/metrics.rs`: `recall_at_k`, `ndcg_at_k`,
`kendall_tau`, `exact_match_at_k`.

**Acceptance test:** seal a real corpus (D6 is the obvious candidate — 14 KBs, existing eval
harness), run the same query set against sealed and clear tenants, and require
`recall@10 == 1.0` and `kendall_tau ≥ 0.99`. If f32 costs more than that, the options are to
store f64 on the host (a real storage change, and a change Eve can see) or to accept and
document a recall figure. Decide with numbers.

## Coexistence and migration

Sealed and clear tenants live on one host with no flag distinguishing them, which is the
firewall property working as intended. Existing tenants keep dimension `n` and a `Clear`
cloak; no migration is required to adopt this.

**Rotating `Q` means re-sealing every vector** — a full re-embed and re-upload of the
tenant. There is no in-place rekey, because the host cannot transform what it cannot read.
Size the tenant with that in mind.

## Threat model, stated plainly

From the published analysis (Kwaai, SIAM 2025, §6 *"Sacking ROME"*): the scheme falls to an
actor who observes both the plaintext and the encrypted form of enough queries, at which
point `Q` is recoverable by linear algebra — `QEAAᵀ = ÃAᵀ`.

The VPK boundary exists to exclude that actor. The guarantee therefore rests on two
assumptions, both of which should be written wherever this ships:

1. **The host never sees plaintext queries.** Structural — sealing happens before the wire.
2. **The host does not hold the embedder.** Not structural. This is an assumption about
   deployment, and it is the weaker of the two.

Two further properties follow from the mathematics and should be stated rather than
discovered: the transform is **deterministic**, so identical vectors produce identical
ciphertexts and the host can see duplicates; and preserving inner products means the host
can see the **corpus geometry** — clusters, norms, every pairwise similarity — without
reading any vector.

**Do not describe this as IND-CPA or semantically secure in KwaaiNet documentation.**
IND-CPA is strictly stronger than resistance to known-plaintext key recovery, and §6 of the
public paper is the refutation. The honest sentence is: *the host stores vectors it cannot
read, learns their geometry but not their content, and never sees the text or the query.*
That is a real privacy property and it survives scrutiny.

## Sequence

1. `kwaai-vpk` crate with `VectorCloak`, `Clear`, and the seed→`Q` derivation, plus the
   fixed-seed checksum test. No storage changes.
2. Wire `phe` in as a dependency and implement `Rome` behind the trait. Private repo, so
   decide git-dependency-with-auth versus vendoring first.
3. Measure f32 ranking loss on D6. Publish the numbers in this plan.
4. Wire the client paths and add `--seal` (or a tenant property) to the CLI.
5. Only then update the storage roadmap's Planned entry to Shipped — with the threat model
   next to it.

## Do not

- Put ROME behind `kwaai-storage`. The host would then know, and the property is lost.
- Derive the seed from the node identity key.
- Ship step 5 before step 3 produces numbers.
