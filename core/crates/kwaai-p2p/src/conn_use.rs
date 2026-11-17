//! Which connection application traffic rides on, and whether it is busy.
//!
//! Shared by [`crate::unary`] and [`crate::raw_stream`]. Each connection's
//! handler owns a token; every queued request, live exchange and handed-out
//! stream holds a clone, so a connection is busy while the token has more than
//! one owner.

use std::collections::HashMap;
use std::sync::{Arc, Weak};

use libp2p::swarm::ConnectionId;

/// Held by anything still using a connection; see the module docs.
pub(crate) type InFlight = Arc<()>;

#[derive(Default)]
pub(crate) struct ConnUse {
    conns: HashMap<ConnectionId, (Weak<()>, bool)>,
}

impl ConnUse {
    /// Track a new connection and return its token, for its handler to own.
    pub(crate) fn open(&mut self, id: ConnectionId, relayed: bool) -> InFlight {
        let token = InFlight::default();
        self.conns.insert(id, (Arc::downgrade(&token), relayed));
        token
    }

    pub(crate) fn close(&mut self, id: ConnectionId) {
        self.conns.remove(&id);
    }

    /// Whether any request or stream is still using `id`.
    pub(crate) fn busy(&self, id: ConnectionId) -> bool {
        self.conns
            .get(&id)
            .is_some_and(|(token, _)| token.strong_count() > 1)
    }

    /// The connection new traffic should use: the first direct one, else the first.
    pub(crate) fn pick(&self, conns: &[ConnectionId]) -> ConnectionId {
        conns
            .iter()
            .copied()
            .find(|id| self.conns.get(id).is_some_and(|(_, relayed)| !relayed))
            .unwrap_or(conns[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_connection_is_busy_while_a_token_clone_lives() {
        let mut use_ = ConnUse::default();
        let id = ConnectionId::new_unchecked(1);
        let token = use_.open(id, false);
        assert!(!use_.busy(id));
        let held = Arc::clone(&token);
        assert!(use_.busy(id));
        drop(held);
        assert!(!use_.busy(id));
    }

    #[test]
    fn new_traffic_prefers_a_direct_connection_over_an_older_relayed_one() {
        let mut use_ = ConnUse::default();
        let relayed = ConnectionId::new_unchecked(1);
        let direct = ConnectionId::new_unchecked(2);
        use_.open(relayed, true);
        use_.open(direct, false);
        assert_eq!(use_.pick(&[relayed, direct]), direct);
        assert_eq!(use_.pick(&[relayed]), relayed);
    }
}
