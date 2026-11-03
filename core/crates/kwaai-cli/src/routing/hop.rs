//! What every routing strategy shares: choosing a hop and classifying its failure.

use std::collections::HashSet;

use libp2p::PeerId;

use crate::shard_cmd::{BlockServerEntry, HopFailureKind};

/// Per-hop timing record: (peer display name, start_block, end_block, elapsed_ms).
pub type HopTiming = (String, usize, usize, f64);

/// Marker in the error a hop timeout produces, so the failure classifier can
/// tell it apart from an error the peer itself reported.
pub(crate) const HOP_TIMEOUT_MARKER: &str = "hop timeout after";

/// Classify a hop failure from its error text.
///
/// Split out of the dispatch loop so the policy — which failures blacklist a
/// peer for the rest of the run and which leave it eligible — is testable
/// without a live peer.
pub fn classify_hop_failure(err: &str) -> HopFailureKind {
    if err.contains("protocols not supported") {
        HopFailureKind::NoHandler
    } else if err.contains(HOP_TIMEOUT_MARKER) {
        HopFailureKind::Timeout
    } else if err.contains("stream reset")
        || err.contains("early eof")
        || err.contains("connection closed")
    {
        HopFailureKind::Transient
    } else {
        HopFailureKind::Other
    }
}

/// The entries of `chain` that may serve the hop at `pos`, most preferred
/// first. Only entries starting exactly at `pos` qualify — one starting
/// earlier would re-apply layers, since a request carries no block range.
/// Handed a pinned path that is the planned entry alone; a wider chain ranks
/// its alternatives reached-first, then by span, trust, and self-dispatch.
pub fn hop_candidates<'a>(
    chain: &'a [BlockServerEntry],
    pos: usize,
    failed_peers: &HashSet<PeerId>,
    unreached: &HashSet<PeerId>,
    our_peer_id: Option<&PeerId>,
) -> Vec<&'a BlockServerEntry> {
    let mut candidates: Vec<&BlockServerEntry> = chain
        .iter()
        .filter(|e| e.start_block == pos && e.end_block > pos)
        .filter(|e| !failed_peers.contains(&e.peer_id))
        .collect();
    candidates.sort_by(|a, b| {
        unreached
            .contains(&a.peer_id)
            .cmp(&unreached.contains(&b.peer_id))
            .then_with(|| b.end_block.cmp(&a.end_block))
            .then_with(|| {
                b.trust_score
                    .partial_cmp(&a.trust_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .then_with(|| {
                let a_self = our_peer_id == Some(&a.peer_id);
                let b_self = our_peer_id == Some(&b.peer_id);
                b_self.cmp(&a_self)
            })
    });
    candidates
}

/// A [`forward_through_chain`] failure at a block position no candidate could
/// serve. Displays as its message, so callers formatting the error see no change.
#[derive(Debug)]
pub struct ChainPositionError {
    pub block: usize,
    pub(crate) message: String,
}

impl std::fmt::Display for ChainPositionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ChainPositionError {}
