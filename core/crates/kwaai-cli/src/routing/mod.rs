//! How one token step travels the pinned path. Each strategy lives in its
//! own file; discovery, path pinning and circuits stay in `shard_cmd`.

mod hop;
mod star;

#[cfg(test)]
pub use hop::{classify_hop_failure, hop_candidates};
pub use hop::{ChainPositionError, HopTiming};
pub use star::forward_through_chain;
