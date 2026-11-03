//! How one token step travels the pinned path. Each strategy lives in its
//! own file; discovery, path pinning and circuits stay in `shard_cmd`.

pub mod forward_chain;
mod hop;
mod star;

use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use kwaai_p2p_daemon::P2PClient;
use libp2p::PeerId;

use self::forward_chain::ChainOriginator;
use crate::block_rpc::{InferenceRequest, InferenceResponse, PayloadType};
use crate::config::KwaaiNetConfig;
use crate::display::print_warning;
use crate::reputation::ReputationStore;
use crate::shard_cmd::{BlockServerEntry, ProgressSink, TokenContext};

#[cfg(test)]
pub use hop::{classify_hop_failure, hop_candidates};
pub use hop::{ChainPositionError, HopTiming};
pub use star::forward_through_chain;

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum RouteMode {
    /// Every hop returns to the originator.
    Star,
    /// Hop to hop; the last returns to the originator.
    Chain,
}

impl RouteMode {
    /// A typed `--route` wins; otherwise `shard.forward_chain.originate`.
    pub fn resolve(flag: Option<RouteMode>, cfg: &KwaaiNetConfig) -> Self {
        flag.unwrap_or(if cfg.shard.forward_chain.originate {
            Self::Chain
        } else {
            Self::Star
        })
    }
}

/// A run's routing strategy.
pub enum Router {
    Star,
    Chain(ChainOriginator),
}

impl Router {
    /// A chain the node cannot set up degrades to star with a warning rather
    /// than failing the run.
    pub async fn open(mode: RouteMode, session_id: u64, our_peer_id: &PeerId) -> Self {
        match mode {
            RouteMode::Star => Self::Star,
            RouteMode::Chain => match ChainOriginator::open(session_id, our_peer_id).await {
                Ok(o) => Self::Chain(o),
                Err(e) => {
                    print_warning(&format!("forward chain unavailable ({e:#}) — using star"));
                    Self::Star
                }
            },
        }
    }

    /// One line for `--stats`.
    pub fn summary(&self) -> String {
        match self {
            Self::Star => "star".into(),
            Self::Chain(o) => match &o.fallback {
                None => format!("forward chain ({} decode steps)", o.steps),
                Some(why) => format!("forward chain for {} steps, then star: {why}", o.steps),
            },
        }
    }

    /// Run one token step. Same contract as [`forward_through_chain`]; a
    /// forward chain carries decode steps only.
    #[allow(clippy::too_many_arguments)]
    pub async fn forward(
        &mut self,
        client: &mut P2PClient,
        chain: &[BlockServerEntry],
        total_blocks: usize,
        session_id: u64,
        seq_pos: u32,
        first_request: InferenceRequest,
        our_peer_id: Option<&PeerId>,
        failed_peers: &mut HashSet<PeerId>,
        unreached: &HashSet<PeerId>,
        sink: Option<&dyn ProgressSink>,
        reputation: Option<Arc<Mutex<ReputationStore>>>,
        token_ctx: TokenContext,
    ) -> Result<InferenceResponse> {
        let decode = first_request.payload_type == PayloadType::TokenIds && seq_pos > 0;
        match self {
            Self::Chain(o) if o.fallback.is_none() && decode && chain.len() > 1 => {
                o.step(
                    client,
                    chain,
                    total_blocks,
                    session_id,
                    seq_pos,
                    first_request,
                    our_peer_id,
                    failed_peers,
                    unreached,
                    sink,
                    reputation,
                    token_ctx,
                )
                .await
            }
            _ => {
                forward_through_chain(
                    client,
                    chain,
                    total_blocks,
                    session_id,
                    seq_pos,
                    first_request,
                    our_peer_id,
                    failed_peers,
                    unreached,
                    sink,
                    reputation,
                    token_ctx,
                )
                .await
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn originating_is_off_unless_asked() {
        let mut cfg = KwaaiNetConfig::default();
        assert!(cfg.shard.forward_chain.serve, "serving defaults on");
        assert_eq!(RouteMode::resolve(None, &cfg), RouteMode::Star);
        assert_eq!(
            RouteMode::resolve(Some(RouteMode::Chain), &cfg),
            RouteMode::Chain
        );
        cfg.shard.forward_chain.originate = true;
        assert_eq!(RouteMode::resolve(None, &cfg), RouteMode::Chain);
        assert_eq!(
            RouteMode::resolve(Some(RouteMode::Star), &cfg),
            RouteMode::Star
        );
    }

    #[test]
    fn the_config_block_reads_and_stays_out_of_a_default_file() {
        let cfg: KwaaiNetConfig =
            serde_yaml::from_str("shard:\n  forward_chain:\n    originate: true\n").unwrap();
        assert!(cfg.shard.forward_chain.originate && cfg.shard.forward_chain.serve);
        let y = serde_yaml::to_string(&KwaaiNetConfig::default()).unwrap();
        assert!(!y.contains("forward_chain"), "{y}");
    }
}
