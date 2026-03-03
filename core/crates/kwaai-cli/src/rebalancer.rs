//! Block rebalancing logic for `kwaainet shard serve --auto-rebalance`.
//!
//! `check_rebalance()` is a pure function that examines the current DHT chain
//! coverage and decides whether this node should move its blocks.  It returns
//! `Some((new_start, new_end))` when a move is warranted, `None` when the node
//! should stay put.
//!
//! # Decision algorithm
//!
//! 1. Build `coverage[0..total_blocks]` counting **other** peers per block
//!    (our own peer is excluded).
//! 2. If `min(coverage[our_start..our_end]) < min_redundancy` → stay put.
//!    (we are sole or insufficient coverage of our own range; moving would
//!    create a gap.)
//! 3. Find the first block `i` where `coverage[i] == 0` (uncovered gap).
//!    If none → network is fully covered, no move needed.
//! 4. Return `Some((i, min(i + target_blocks, total_blocks)))`.

use libp2p::PeerId;

use crate::shard_cmd::BlockServerEntry;

/// Decide whether this node should move its blocks to fill a gap.
///
/// Returns `Some((new_start, new_end))` if a rebalance is warranted,
/// or `None` if the node should keep serving its current range.
pub fn check_rebalance(
    chain: &[BlockServerEntry],
    our_peer_id: &PeerId,
    our_start: usize,
    our_end: usize,
    total_blocks: usize,
    target_blocks: usize,
    min_redundancy: usize,
) -> Option<(usize, usize)> {
    if total_blocks == 0 || our_start >= our_end {
        return None;
    }

    // Build per-block coverage count, excluding ourselves.
    let mut coverage = vec![0usize; total_blocks];
    for entry in chain {
        if &entry.peer_id == our_peer_id {
            continue;
        }
        let s = entry.start_block.min(total_blocks);
        let e = entry.end_block.min(total_blocks);
        for c in &mut coverage[s..e] {
            *c += 1;
        }
    }

    // Step 2 — stay put if our range is not sufficiently covered by others.
    let our_min_coverage = coverage[our_start.min(total_blocks)..our_end.min(total_blocks)]
        .iter()
        .copied()
        .min()
        .unwrap_or(0);
    if our_min_coverage < min_redundancy {
        return None;
    }

    // Step 3 — find the first uncovered block.
    let gap_start = coverage.iter().position(|&c| c == 0)?;

    // Step 4 — propose a new range starting at the gap.
    let gap_end = (gap_start + target_blocks).min(total_blocks);
    Some((gap_start, gap_end))
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a fake PeerId from a small integer (deterministic, no crypto).
    fn fake_peer(n: u8) -> PeerId {
        // A valid multihash-based PeerId: sha2-256 identity multihash prefix + 32 bytes.
        // Simplest approach: use the identity multihash (0x00 0x04 0x00…) which libp2p accepts.
        // We use the raw bytes representation with a deterministic public key.
        use libp2p::identity::Keypair;
        // Derive a deterministic-ish keypair by seeding a fixed bytes pattern.
        // Since libp2p doesn't expose seeded key generation in stable API,
        // we generate real keypairs and just need distinct PeerIds per test.
        // For tests we use PeerId::random() seeded differently — easiest: just generate N.
        let _ = n;
        // Actually use from_bytes on a crafted identity multihash.
        // identity multihash = varint(0x00) varint(len) data
        // PeerId from_bytes expects a multihash encoding. Use Ed25519 peer IDs via Keypair.
        Keypair::generate_ed25519().public().to_peer_id()
    }

    fn make_entry(peer: PeerId, start: usize, end: usize) -> BlockServerEntry {
        BlockServerEntry {
            peer_id: peer,
            start_block: start,
            end_block: end,
            public_name: format!("node-{}", start),
        }
    }

    /// Single node — no other coverage; moving would create a gap.
    #[test]
    fn no_rebalance_when_alone() {
        let our_peer = fake_peer(1);
        let chain = vec![make_entry(our_peer.clone(), 0, 8)];
        let result = check_rebalance(&chain, &our_peer, 0, 8, 32, 8, 2);
        assert_eq!(result, None, "Should not rebalance when alone");
    }

    /// All blocks covered ≥ min_redundancy by other nodes — but no gaps exist.
    #[test]
    fn no_rebalance_when_full_coverage() {
        let our_peer = fake_peer(1);
        let peer_b = fake_peer(2);
        let peer_c = fake_peer(3);
        // our range 0-8, covered by B and C as well
        // remaining blocks 8-32 also covered by B and C
        let chain = vec![
            make_entry(our_peer.clone(), 0, 8),
            make_entry(peer_b.clone(), 0, 32),
            make_entry(peer_c.clone(), 0, 32),
        ];
        // Our range (0-8) has min_coverage >= 2 (B and C each cover it).
        // No block has coverage == 0 (B and C cover everything).
        let result = check_rebalance(&chain, &our_peer, 0, 8, 32, 8, 2);
        assert_eq!(result, None, "No gap → no rebalance");
    }

    /// Our range is covered ≥ 2× by others, and there is a gap at block 8.
    #[test]
    fn rebalance_when_gap_and_redundant() {
        let our_peer = fake_peer(1);
        let peer_b = fake_peer(2);
        let peer_c = fake_peer(3);
        // B and C both cover blocks 0-8 (so our range has 2× other coverage).
        // Blocks 8-32 are uncovered — gap starts at 8.
        let chain = vec![
            make_entry(our_peer.clone(), 0, 8),
            make_entry(peer_b.clone(), 0, 8),
            make_entry(peer_c.clone(), 0, 8),
        ];
        let result = check_rebalance(&chain, &our_peer, 0, 8, 32, 8, 2);
        assert_eq!(result, Some((8, 16)), "Should move to fill gap at 8");
    }

    /// Gap exists but we are the sole coverage of our range — must not move.
    #[test]
    fn no_rebalance_when_gap_but_not_redundant() {
        let our_peer = fake_peer(1);
        let peer_b = fake_peer(2);
        // B covers 8-32 only. Our range 0-8 has zero other coverage.
        let chain = vec![
            make_entry(our_peer.clone(), 0, 8),
            make_entry(peer_b.clone(), 8, 32),
        ];
        let result = check_rebalance(&chain, &our_peer, 0, 8, 32, 8, 2);
        assert_eq!(result, None, "Only coverage of our range — must not move");
    }

    /// Multiple gaps — rebalancer picks the lowest (first uncovered) block.
    #[test]
    fn rebalance_picks_lowest_gap() {
        let our_peer = fake_peer(1);
        let peer_b = fake_peer(2);
        let peer_c = fake_peer(3);
        // Peers B and C both cover 0-8 and 16-24; gaps at 8-16 and 24-32.
        let chain = vec![
            make_entry(our_peer.clone(), 0, 8),
            make_entry(peer_b.clone(), 0, 8),
            make_entry(peer_b.clone(), 16, 24),
            make_entry(peer_c.clone(), 0, 8),
            make_entry(peer_c.clone(), 16, 24),
        ];
        let result = check_rebalance(&chain, &our_peer, 0, 8, 32, 8, 2);
        // Lowest gap is 8 (not 24).
        assert_eq!(result, Some((8, 16)), "Should pick the lowest gap first");
    }
}
