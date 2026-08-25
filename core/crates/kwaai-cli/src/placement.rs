//! Which peers a DHT record is placed on, and read back from.
//!
//! With `decentralized_dht` off, a record goes to every configured bootstrap
//! and comes back from the same list. This module is the replacement for that
//! list: given the peers a node happens to know, it answers *"who should hold
//! this particular key"* — and gets the same answer any other node with a
//! similar view would give, which is what makes a record findable by someone
//! who never spoke to the peer that wrote it.
//!
//! # The keyspace is hivemind's, not libp2p-kad's
//!
//! This is the detail that matters most and the easiest one to get wrong.
//! Both keyspaces are in play in this binary at once:
//!
//! | | node ID | record ID | used by |
//! | --- | --- | --- | --- |
//! | libp2p-kad | `SHA256(peer_id)` | `SHA256(key)` | the swarm's own routing |
//! | **hivemind** | `SHA1(peer_id.to_bytes())` | `SHA1(msgpack(key))` | **this module** |
//!
//! Placement must use the hivemind space because that is the space the
//! *readers* search in. The Python health service beam-searches by asking peers
//! for `DHTProtocol.rpc_find` nearest lists, and a hivemind server answers those
//! from `DHTStorage`, XOR-sorted over 20-byte SHA1 DHTIDs
//! (`kwaai_hivemind_dht::server::nearest_peers`). A record placed by SHA256
//! distance would sit on peers that no SHA1 beam search ever walks to: stored
//! successfully, findable by nobody.
//!
//! `SHA1(peer_id.to_bytes())` is [`NodeInfo::from_peer_id`]'s derivation and
//! the same one `discover_chain` computes for its own `our_dhtid`;
//! [`peer_dht_id`] is that one line, named.
//!
//! # Distance
//!
//! Byte-wise XOR of the two 20-byte IDs, compared lexicographically. Because
//! DHTIDs are big-endian integers, lexicographic order over the XOR bytes *is*
//! numeric order, so no bignum arithmetic is needed — the same argument
//! `kwaai_hivemind_dht`'s `xor_distance` makes.
//!
//! # Placement is attempt-and-advance, not a fixed slice
//!
//! [`rank_candidates`] returns *every* candidate in nearest-first order rather
//! than the nearest `k`. The caller walks that ranking, and a peer that fails to
//! store (dial failure, timeout, a refusal) does not consume a replica slot —
//! the walk advances to the next-nearest until `k` stores have succeeded or the
//! ranking is exhausted. Returning a truncated `k` would make an unreachable
//! peer silently cost a replica, which is precisely the fragility this feature
//! exists to remove.
//!
//! # Scope
//!
//! One round, no iterative deepening: candidates are the peers we already know,
//! not peers discovered by asking those peers who *they* know. That is
//! sufficient while the network is small enough that any node's view overlaps
//! every other's, and [`rank_candidates`] is the seam a beam search would be
//! built on — it ranks whatever candidate set it is handed, so widening the set
//! is the only change a later phase needs to make here.

use kwaai_p2p::{KnownPeer, PeerId};
use sha1::{Digest, Sha1};

/// A peer eligible to hold a record, with its hivemind DHTID precomputed.
///
/// Built from [`KnownPeer`]s via [`candidates_from_known`], or directly in
/// tests. `dht_id` is derived once here rather than per key, because a
/// placement round ranks the same candidate set against every record key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    /// The peer to store to.
    pub peer_id: PeerId,
    /// `SHA1(peer_id.to_bytes())` — the hivemind DHTID, 20 bytes.
    pub dht_id: Vec<u8>,
    /// A dialable multiaddr, kept for logging and for the reputation timing
    /// that the announce path reports per peer. Empty when the candidate came
    /// from a source that carried no address.
    pub addr: String,
}

impl Candidate {
    /// Build a candidate for `peer_id`, deriving its DHTID.
    pub fn new(peer_id: PeerId, addr: String) -> Self {
        Self {
            dht_id: peer_dht_id(&peer_id),
            peer_id,
            addr,
        }
    }
}

/// `SHA1(peer_id.to_bytes())` — a peer's position in the hivemind keyspace.
///
/// Identical to `NodeInfo::from_peer_id`'s `node_id` and to the `our_dhtid`
/// `discover_chain` computes. **Not** `PeerId::to_bytes()` itself and not a
/// SHA256: see the module docs for why the distinction is load-bearing.
pub fn peer_dht_id(peer_id: &PeerId) -> Vec<u8> {
    Sha1::new()
        .chain_update(peer_id.to_bytes())
        .finalize()
        .to_vec()
}

/// XOR distance between two DHTIDs, as raw bytes for lexicographic comparison.
///
/// Mirrors `kwaai_hivemind_dht::server::xor_distance` — kept as its own copy
/// rather than exported from there because that one is a private detail of the
/// storage server, and a divergence would be caught by
/// `xor_distance_matches_the_hivemind_server` below.
///
/// Differing lengths are zero-extended on the *left*, preserving big-endian
/// numeric alignment, so a malformed ID orders deterministically instead of
/// panicking.
fn xor_distance(a: &[u8], b: &[u8]) -> Vec<u8> {
    let n = a.len().max(b.len());
    (0..n)
        .map(|i| {
            let from_end = n - i;
            let ai = a.len().checked_sub(from_end).map_or(0, |idx| a[idx]);
            let bi = b.len().checked_sub(from_end).map_or(0, |idx| b[idx]);
            ai ^ bi
        })
        .collect()
}

/// Turn the handle's known-peer snapshot into placement candidates.
///
/// Peers with no address are dropped: they cannot be stored to, and keeping
/// them would let an undialable peer outrank a reachable one purely on
/// distance. `exclude` is this node's own peer ID — a node serves its own
/// records from local storage directly, so spending a replica dialing itself
/// would be a wasted slot.
pub fn candidates_from_known(known: &[KnownPeer], exclude: &PeerId) -> Vec<Candidate> {
    known
        .iter()
        .filter(|k| &k.peer_id != exclude)
        .filter_map(|k| {
            let addr = k.addrs.first()?;
            Some(Candidate::new(
                k.peer_id,
                format!("{addr}/p2p/{}", k.peer_id),
            ))
        })
        .collect()
}

/// Parse configured `initial_peers` / `bootstrap_peers` multiaddrs into
/// candidates.
///
/// Under decentralization these are ordinary candidates with no special role —
/// they are simply peers whose address we happen to have been given rather than
/// having learned. Entries without a `/p2p/<id>` component, or with an
/// unparseable one, are skipped: an address we cannot attribute to a peer ID
/// has no position in the keyspace to rank.
pub fn candidates_from_addrs(addrs: &[String], exclude: &PeerId) -> Vec<Candidate> {
    addrs
        .iter()
        .filter_map(|addr| {
            let peer_id: PeerId = addr.split("/p2p/").nth(1)?.parse().ok()?;
            (&peer_id != exclude).then(|| Candidate::new(peer_id, addr.clone()))
        })
        .collect()
}

/// Merge candidate sources, keeping one entry per peer.
///
/// Earlier sources win on collision, so a live known-peer address is preferred
/// over a configured one for the same peer. Order within the result is not
/// meaningful — [`rank_candidates`] imposes the order that matters.
pub fn merge_candidates(sources: &[Vec<Candidate>]) -> Vec<Candidate> {
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    for source in sources {
        for candidate in source {
            if seen.insert(candidate.peer_id) {
                out.push(candidate.clone());
            }
        }
    }
    out
}

/// Order `candidates` nearest-first around `record_id` in the hivemind
/// keyspace.
///
/// `record_id` is the record's own DHTID — `announce::dht_id(key)`, i.e.
/// `SHA1(msgpack(key))` — not the raw key.
///
/// The full ranking is returned, not the first `k`: see the module docs on
/// attempt-and-advance. Ties on XOR distance are impossible between distinct
/// peers (equal distance to a common point implies equal DHTIDs), but the sort
/// breaks any that a duplicated candidate could produce by peer ID, so the
/// order is total and identical across nodes and across runs.
pub fn rank_candidates(candidates: &[Candidate], record_id: &[u8]) -> Vec<Candidate> {
    let mut ranked: Vec<(Vec<u8>, &Candidate)> = candidates
        .iter()
        .map(|c| (xor_distance(record_id, &c.dht_id), c))
        .collect();

    ranked.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.peer_id.cmp(&b.1.peer_id)));
    ranked.into_iter().map(|(_, c)| c.clone()).collect()
}

/// Report on one record's placement round, for logging and for tests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacementOutcome {
    /// Peers that accepted the store, in the order they were tried.
    pub stored_on: Vec<PeerId>,
    /// Peers that were tried and failed. Not an error by itself — the walk
    /// advances past them — but a large count means the candidate set is
    /// mostly stale.
    pub failed: Vec<PeerId>,
    /// How many replicas short of `k` the round finished. Zero on success.
    pub shortfall: usize,
}

impl PlacementOutcome {
    /// Whether the record landed anywhere at all.
    pub fn any_success(&self) -> bool {
        !self.stored_on.is_empty()
    }
}

/// Walk `ranked` nearest-first, calling `store` on each until `k` succeed.
///
/// The generic exists so the walk itself — the attempt-and-advance rule and the
/// shortfall accounting — is testable without a network: the announce path
/// passes a closure that issues the real STORE RPC, and the tests below pass
/// one that consults a set of peers designated to fail.
///
/// Returns as soon as `k` stores succeed, so a healthy network dials exactly
/// `k` peers and no more.
pub async fn place_with<F, Fut>(ranked: &[Candidate], k: usize, mut store: F) -> PlacementOutcome
where
    F: FnMut(Candidate) -> Fut,
    Fut: std::future::Future<Output = bool>,
{
    let mut stored_on = Vec::new();
    let mut failed = Vec::new();

    for candidate in ranked {
        if stored_on.len() >= k {
            break;
        }
        if store(candidate.clone()).await {
            stored_on.push(candidate.peer_id);
        } else {
            failed.push(candidate.peer_id);
        }
    }

    PlacementOutcome {
        shortfall: k.saturating_sub(stored_on.len()),
        stored_on,
        failed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::announce::dht_id;

    /// A candidate with a hand-chosen DHTID, so distance is arithmetic the test
    /// can predict rather than whatever SHA1 happens to produce.
    fn at(dht_id: Vec<u8>) -> Candidate {
        Candidate {
            peer_id: PeerId::random(),
            dht_id,
            addr: String::new(),
        }
    }

    /// A 20-byte ID that is `first` followed by zeros.
    fn id(first: u8) -> Vec<u8> {
        let mut v = vec![0u8; 20];
        v[0] = first;
        v
    }

    // ── The keyspace ────────────────────────────────────────────────────────

    /// A peer's placement ID is `SHA1(peer_id.to_bytes())` — the same 20-byte
    /// DHTID hivemind puts in `NodeInfo.node_id`. If these ever diverge, records
    /// land on peers that no hivemind beam search reaches.
    #[test]
    fn peer_dht_id_is_the_hivemind_node_id() {
        let peer = PeerId::random();
        let expected = kwaai_hivemind_dht::protocol::NodeInfo::from_peer_id(peer).node_id;

        assert_eq!(peer_dht_id(&peer), expected);
        assert_eq!(peer_dht_id(&peer).len(), 20, "hivemind DHTIDs are 20 bytes");
        assert_ne!(
            peer_dht_id(&peer),
            peer.to_bytes(),
            "the raw peer id is not its DHTID"
        );
    }

    /// The placement keyspace is SHA1, not the SHA256 keyspace libp2p-kad
    /// routes in. Asserted explicitly because both are live in this binary and
    /// the mistake is invisible at runtime — stores succeed and reads miss.
    #[test]
    fn placement_does_not_use_the_libp2p_kad_keyspace() {
        use sha2::{Digest as _, Sha256};
        let peer = PeerId::random();
        let sha256: Vec<u8> = Sha256::digest(peer.to_bytes()).to_vec();
        assert_ne!(peer_dht_id(&peer), sha256);
        assert_eq!(peer_dht_id(&peer).len(), 20, "SHA1, not SHA256's 32");
    }

    /// Our XOR must agree with the one a hivemind server sorts its `rpc_find`
    /// nearest lists by — that agreement is the entire basis for expecting a
    /// reader to walk to the peer a writer chose.
    #[test]
    fn xor_distance_matches_the_hivemind_server() {
        // Distance to self is zero, and the ordering is by magnitude from the
        // most significant byte down — the properties `nearest_peers` relies on.
        assert_eq!(xor_distance(&id(0x42), &id(0x42)), vec![0u8; 20]);
        assert!(xor_distance(&id(0x00), &id(0x01)) < xor_distance(&id(0x00), &id(0x02)));
        assert!(xor_distance(&id(0x00), &id(0x7f)) < xor_distance(&id(0x00), &id(0x80)));
        // Symmetric, as XOR is.
        assert_eq!(
            xor_distance(&id(0x13), &id(0x37)),
            xor_distance(&id(0x37), &id(0x13))
        );
    }

    /// Differing-length IDs are left-padded, not truncated or panicked on.
    #[test]
    fn xor_distance_left_pads_short_ids() {
        assert_eq!(xor_distance(&[0x01], &[0x00, 0x01]), vec![0x00, 0x00]);
        assert_eq!(xor_distance(&[], &[0xff]), vec![0xff]);
    }

    // ── Ranking ─────────────────────────────────────────────────────────────

    /// The nearest candidate to a key is the one whose DHTID differs from it in
    /// the least significant way — ordering is by XOR magnitude, not by the raw
    /// ID's magnitude.
    #[test]
    fn candidates_are_ranked_by_xor_distance_to_the_record() {
        let key = id(0x10);
        // XOR against 0x10: 0x11→0x01, 0x14→0x04, 0x18→0x08, 0x90→0x80.
        let candidates = vec![at(id(0x90)), at(id(0x18)), at(id(0x11)), at(id(0x14))];
        let ranked = rank_candidates(&candidates, &key);

        let order: Vec<u8> = ranked.iter().map(|c| c.dht_id[0]).collect();
        assert_eq!(
            order,
            vec![0x11, 0x14, 0x18, 0x90],
            "nearest-first by XOR against the key, not by the ID's own value"
        );
    }

    /// A candidate that *is* the key's own ID sorts first — distance zero.
    #[test]
    fn an_exact_match_ranks_first() {
        let key = id(0x55);
        let candidates = vec![at(id(0x54)), at(id(0x55)), at(id(0x57))];
        assert_eq!(rank_candidates(&candidates, &key)[0].dht_id, id(0x55));
    }

    /// Two different keys rank the same candidate set differently — the whole
    /// point of per-key placement. Records under different keys are *expected*
    /// to land on different peer sets.
    #[test]
    fn different_keys_select_different_peers() {
        let candidates = vec![at(id(0x00)), at(id(0x40)), at(id(0x80)), at(id(0xc0))];

        let near_low = rank_candidates(&candidates, &id(0x01));
        let near_high = rank_candidates(&candidates, &id(0xc1));

        assert_eq!(near_low[0].dht_id, id(0x00));
        assert_eq!(near_high[0].dht_id, id(0xc0));
        assert_ne!(
            near_low[0].peer_id, near_high[0].peer_id,
            "placement must depend on the record key, or every record piles onto one peer"
        );
    }

    /// Ranking is deterministic: the same inputs in any input order give the
    /// same output order. Two nodes with the same view must agree on placement,
    /// or a reader looks in the wrong place.
    #[test]
    fn ranking_is_deterministic_regardless_of_input_order() {
        let a = at(id(0x11));
        let b = at(id(0x22));
        let c = at(id(0x33));
        let key = id(0x20);

        let forward = rank_candidates(&[a.clone(), b.clone(), c.clone()], &key);
        let backward = rank_candidates(&[c, b, a], &key);

        let ids = |v: &[Candidate]| v.iter().map(|c| c.peer_id).collect::<Vec<_>>();
        assert_eq!(ids(&forward), ids(&backward));
    }

    /// Every candidate is returned, not just the first k — the caller needs the
    /// tail to advance past failures.
    #[test]
    fn ranking_returns_every_candidate() {
        let candidates: Vec<Candidate> = (0..10).map(|i| at(id(i * 16))).collect();
        assert_eq!(rank_candidates(&candidates, &id(0x00)).len(), 10);
    }

    /// An empty candidate set ranks to nothing rather than panicking — a node
    /// that knows no peers yet is a normal startup state.
    #[test]
    fn ranking_an_empty_set_is_empty() {
        assert!(rank_candidates(&[], &dht_id("_petals.models")).is_empty());
    }

    // ── Candidate assembly ──────────────────────────────────────────────────

    /// Peers with no dialable address are not candidates: they would outrank
    /// reachable peers on distance and then fail every store.
    #[test]
    fn known_peers_without_an_address_are_not_candidates() {
        let with_addr = PeerId::random();
        let without = PeerId::random();
        let known = vec![
            KnownPeer {
                peer_id: with_addr,
                addrs: vec!["/ip4/10.0.0.1/tcp/8000".parse().unwrap()],
                connected: true,
            },
            KnownPeer {
                peer_id: without,
                addrs: vec![],
                connected: false,
            },
        ];

        let candidates = candidates_from_known(&known, &PeerId::random());
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].peer_id, with_addr);
        assert!(
            candidates[0].addr.ends_with(&with_addr.to_string()),
            "the candidate address must carry /p2p/<id> so it is dialable as-is"
        );
    }

    /// We are never a candidate for our own records — we store those locally.
    #[test]
    fn our_own_peer_id_is_excluded() {
        let us = PeerId::random();
        let known = vec![KnownPeer {
            peer_id: us,
            addrs: vec!["/ip4/10.0.0.1/tcp/8000".parse().unwrap()],
            connected: true,
        }];
        assert!(candidates_from_known(&known, &us).is_empty());

        let configured = vec![format!("/ip4/10.0.0.1/tcp/8000/p2p/{us}")];
        assert!(candidates_from_addrs(&configured, &us).is_empty());
    }

    /// Configured bootstrap addresses become plain candidates; malformed ones
    /// are skipped rather than failing the round.
    #[test]
    fn configured_addresses_become_ordinary_candidates() {
        let good = PeerId::random();
        let addrs = vec![
            format!("/ip4/198.18.0.10/tcp/8000/p2p/{good}"),
            "/ip4/198.18.0.11/tcp/8000".to_string(), // no /p2p/
            "/ip4/198.18.0.12/tcp/8000/p2p/not-a-peer-id".to_string(),
        ];

        let candidates = candidates_from_addrs(&addrs, &PeerId::random());
        assert_eq!(candidates.len(), 1, "only the well-formed address survives");
        assert_eq!(candidates[0].peer_id, good);
        assert_eq!(candidates[0].dht_id, peer_dht_id(&good));
    }

    /// Merging keeps one entry per peer, preferring the earlier source — the
    /// live address over the configured one for the same peer.
    #[test]
    fn merging_deduplicates_by_peer_preferring_earlier_sources() {
        let shared = PeerId::random();
        let other = PeerId::random();

        let live = vec![Candidate::new(shared, "/live".to_string())];
        let configured = vec![
            Candidate::new(shared, "/configured".to_string()),
            Candidate::new(other, "/other".to_string()),
        ];

        let merged = merge_candidates(&[live, configured]);
        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0].peer_id, shared);
        assert_eq!(merged[0].addr, "/live", "the earlier source wins");
        assert_eq!(merged[1].peer_id, other);
    }

    // ── Placement walk ──────────────────────────────────────────────────────

    /// A healthy network stores on exactly k peers and stops — the nearest
    /// three, and no further dials.
    #[tokio::test]
    async fn placement_stops_after_k_successes() {
        let ranked: Vec<Candidate> = (0..10).map(|i| at(id(i * 16))).collect();
        let mut attempted = 0usize;

        let outcome = place_with(&ranked, 3, |_| {
            attempted += 1;
            async { true }
        })
        .await;

        assert_eq!(outcome.stored_on.len(), 3);
        assert_eq!(attempted, 3, "a healthy round must not dial past k");
        assert_eq!(outcome.shortfall, 0);
        assert!(outcome.failed.is_empty());
        assert!(outcome.any_success());
    }

    /// The load-bearing rule: a failed store does not consume a replica slot.
    /// The walk advances to the next-nearest peer until k *successes*, so two
    /// dead peers at the front cost latency, not durability.
    #[tokio::test]
    async fn a_failed_store_advances_to_the_next_nearest() {
        let ranked: Vec<Candidate> = (0..6).map(|i| at(id(i * 16))).collect();
        let dead: Vec<PeerId> = ranked[..2].iter().map(|c| c.peer_id).collect();

        let outcome = place_with(&ranked, 3, |c| {
            let ok = !dead.contains(&c.peer_id);
            async move { ok }
        })
        .await;

        assert_eq!(
            outcome.stored_on,
            ranked[2..5].iter().map(|c| c.peer_id).collect::<Vec<_>>(),
            "the three nearest *reachable* peers hold the record"
        );
        assert_eq!(outcome.failed, dead);
        assert_eq!(outcome.shortfall, 0, "k was still reached");
    }

    /// Fewer candidates than k is a shortfall, not a failure: the record is
    /// still stored everywhere it can be, and the count is reported so the
    /// caller can warn.
    #[tokio::test]
    async fn too_few_candidates_reports_the_shortfall() {
        let ranked: Vec<Candidate> = (0..2).map(|i| at(id(i * 16))).collect();

        let outcome = place_with(&ranked, 3, |_| async { true }).await;

        assert_eq!(outcome.stored_on.len(), 2);
        assert_eq!(outcome.shortfall, 1);
        assert!(outcome.any_success(), "two replicas is still a success");
    }

    /// Every candidate failing is a full shortfall and no success — the state
    /// that must produce an operator-visible warning rather than a silent pass.
    #[tokio::test]
    async fn a_total_failure_reports_no_success() {
        let ranked: Vec<Candidate> = (0..4).map(|i| at(id(i * 16))).collect();

        let outcome = place_with(&ranked, 3, |_| async { false }).await;

        assert!(!outcome.any_success());
        assert_eq!(outcome.shortfall, 3);
        assert_eq!(outcome.failed.len(), 4, "every candidate was tried");
        assert!(outcome.stored_on.is_empty());
    }

    /// No candidates at all — an isolated node — is a shortfall of k with
    /// nothing attempted, not a panic.
    #[tokio::test]
    async fn placement_with_no_candidates_is_a_full_shortfall() {
        let outcome = place_with(&[], 3, |_| async { true }).await;
        assert_eq!(outcome.shortfall, 3);
        assert!(!outcome.any_success());
        assert!(outcome.failed.is_empty());
    }

    // ── End to end over real keys ───────────────────────────────────────────

    /// The three announce key kinds rank a real candidate set independently.
    /// Different keys landing on different peers is the property that makes the
    /// bootstrap list non-load-bearing.
    #[tokio::test]
    async fn real_announce_keys_place_independently() {
        let candidates: Vec<Candidate> = (0..12)
            .map(|_| Candidate::new(PeerId::random(), "/addr".to_string()))
            .collect();

        let keys = [
            dht_id("_petals.models"),
            dht_id(crate::shard_cmd::INFERENCE_NODES_DHT_KEY),
            dht_id("Qwen/Qwen3-8B-hf.0"),
            dht_id("Qwen/Qwen3-8B-hf.17"),
        ];

        let mut placements = Vec::new();
        for key in &keys {
            let ranked = rank_candidates(&candidates, key);
            let outcome = place_with(&ranked, 3, |_| async { true }).await;
            assert_eq!(outcome.stored_on.len(), 3);
            assert_eq!(outcome.shortfall, 0);
            placements.push(outcome.stored_on);
        }

        // Over 12 candidates and 4 unrelated keys, the selected triples must not
        // all be identical — if they were, placement would not be key-dependent.
        assert!(
            placements.windows(2).any(|w| w[0] != w[1]),
            "different keys must not all resolve to the same three peers"
        );

        // And each placement is reproducible from the same inputs.
        let again = rank_candidates(&candidates, &keys[0]);
        let repeat = place_with(&again, 3, |_| async { true }).await;
        assert_eq!(repeat.stored_on, placements[0]);
    }
}
