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

/// Choose the best block range for a new or rebalancing node to serve.
///
/// - If any blocks are uncovered (coverage == 0 from *other* peers), picks
///   the first such gap.
/// - If the network is fully covered by others, picks the window of
///   `target_blocks` consecutive blocks with the lowest total coverage
///   (join as redundant — still useful for resilience).
///
/// Excludes our own peer ID so stale DHT entries for our old range (TTL up
/// to 360 s) do not falsely mark blocks as covered after a rebalance restart.
///
/// Never returns an error — always returns a valid `(start, end)` range.
pub fn pick_gap_from_chain(
    chain: &[BlockServerEntry],
    our_peer_id: &PeerId,
    total_blocks: usize,
    target_blocks: usize,
) -> (usize, usize) {
    if total_blocks == 0 {
        return (0, 0);
    }
    let target = target_blocks.min(total_blocks);

    // Count per-block coverage excluding ourselves.
    let mut coverage = vec![0usize; total_blocks];
    for e in chain {
        if &e.peer_id == our_peer_id {
            continue;
        }
        let s = e.start_block.min(total_blocks);
        let end = e.end_block.min(total_blocks);
        for c in &mut coverage[s..end] {
            *c += 1;
        }
    }

    let min_cov = coverage.iter().copied().min().unwrap_or(0);

    let start = if min_cov == 0 {
        // Genuine gap: first uncovered block.
        coverage.iter().position(|&c| c == 0).unwrap_or(0)
    } else {
        // Fully covered: minimum-coverage window of `target` blocks.
        let n_windows = total_blocks.saturating_sub(target) + 1;
        (0..n_windows)
            .min_by_key(|&i| coverage[i..i + target].iter().sum::<usize>())
            .unwrap_or(0)
    };

    let end = (start + target).min(total_blocks);
    (start, end)
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a deterministic PeerId from a small integer seed.
    /// `fake_peer(n)` always returns the same PeerId for the same `n`,
    /// and `fake_peer(a) != fake_peer(b)` for `a != b`.
    fn fake_peer(n: u8) -> PeerId {
        let mut seed = [0u8; 32];
        seed[0] = n;
        let sk = libp2p::identity::ed25519::SecretKey::try_from_bytes(&mut seed)
            .expect("valid 32-byte seed");
        let kp = libp2p::identity::Keypair::from(libp2p::identity::ed25519::Keypair::from(sk));
        kp.public().to_peer_id()
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

    // ── pick_gap_from_chain tests ───────────────────────────────────────────

    /// Empty chain — first gap is at block 0.
    #[test]
    fn gap_from_chain_never_panics_on_empty_chain() {
        let our_peer = fake_peer(1);
        let result = pick_gap_from_chain(&[], &our_peer, 32, 8);
        assert_eq!(result, (0, 8), "Empty chain → start at block 0");
    }

    /// Chain covers [0,16); gap starts at 16.
    #[test]
    fn gap_from_chain_finds_genuine_gap() {
        let our_peer = fake_peer(1);
        let peer_b = fake_peer(2);
        let chain = vec![make_entry(peer_b.clone(), 0, 16)];
        let result = pick_gap_from_chain(&chain, &our_peer, 32, 8);
        assert_eq!(result, (16, 24), "Should pick the first uncovered block");
    }

    /// Network fully covered — join the least-covered window.
    #[test]
    fn gap_from_chain_joins_least_covered_when_full() {
        let our_peer = fake_peer(1);
        let peer_b = fake_peer(2);
        let peer_c = fake_peer(3);
        // B covers [0,32), C covers [0,8) and [16,32).
        // Coverage: [0,8)=2, [8,16)=1, [16,32)=2.
        // Least-covered 8-block window is [8,16) with sum=8.
        let chain = vec![
            make_entry(peer_b.clone(), 0, 32),
            make_entry(peer_c.clone(), 0, 8),
            make_entry(peer_c.clone(), 16, 32),
        ];
        let result = pick_gap_from_chain(&chain, &our_peer, 32, 8);
        assert_eq!(result, (8, 16), "Should join least-covered window");
    }

    /// Our stale DHT entry must not count as coverage when choosing a gap.
    #[test]
    fn gap_from_chain_excludes_self() {
        let our_peer = fake_peer(1);
        // Our peer (stale) covers [0,32); no other peers.
        // Without self-exclusion the network looks fully covered.
        // With self-exclusion coverage is all zeros → picks (0, 8).
        let chain = vec![make_entry(our_peer.clone(), 0, 32)];
        let result = pick_gap_from_chain(&chain, &our_peer, 32, 8);
        assert_eq!(
            result,
            (0, 8),
            "Stale self entry must not count as coverage"
        );
    }
}
