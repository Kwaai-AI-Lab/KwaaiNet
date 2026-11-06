//! Periodic INFO summaries of relay and dial activity.
//!
//! Per-event detail stays at DEBUG/TRACE, where a busy relay or a churning
//! client would flood the log. One line per window is enough to see a relay
//! refusing circuits, or a client failing every hole punch, at the default level.

use std::{
    collections::{BTreeMap, HashMap},
    net::IpAddr,
    time::Duration,
};

use libp2p::{swarm::DialError, PeerId};

use crate::addresses::is_circuit;

/// How often the summaries are logged.
pub const SUMMARY_INTERVAL: Duration = Duration::from_secs(5 * 60);

/// Sources named per summary; the rest are only counted.
const TOP_SOURCES: usize = 3;

/// The relay hop server's activity since the last summary.
#[derive(Debug, Default)]
pub struct RelayServerStats {
    reservations_accepted: u64,
    reservations_renewed: u64,
    reservations_denied: BTreeMap<String, u64>,
    reservations_ended: u64,
    circuits_accepted: u64,
    circuits_closed: u64,
    circuits_denied: BTreeMap<String, u64>,
    denied_sources: HashMap<String, u64>,
}

impl RelayServerStats {
    pub fn reservation_accepted(&mut self, renewed: bool) {
        if renewed {
            self.reservations_renewed += 1;
        } else {
            self.reservations_accepted += 1;
        }
    }

    pub fn reservation_denied(&mut self, status: impl std::fmt::Debug) {
        *self
            .reservations_denied
            .entry(format!("{status:?}"))
            .or_default() += 1;
    }

    pub fn reservation_ended(&mut self) {
        self.reservations_ended += 1;
    }

    pub fn circuit_accepted(&mut self) {
        self.circuits_accepted += 1;
    }

    pub fn circuit_closed(&mut self) {
        self.circuits_closed += 1;
    }

    /// `src_ip` names the source when its connection is known, else the peer does.
    pub fn circuit_denied(
        &mut self,
        status: impl std::fmt::Debug,
        src: PeerId,
        src_ip: Option<IpAddr>,
    ) {
        *self
            .circuits_denied
            .entry(format!("{status:?}"))
            .or_default() += 1;
        let who = src_ip.map_or_else(|| src.to_string(), |ip| ip.to_string());
        *self.denied_sources.entry(who).or_default() += 1;
    }

    /// The summary line's fields, resetting the window.
    pub fn take(&mut self, reservations_held: usize) -> String {
        let s = std::mem::take(self);
        format!(
            "reservations held={reservations_held} new={} renewed={} ended={} denied={} | \
             circuits opened={} closed={} denied={}{}",
            s.reservations_accepted,
            s.reservations_renewed,
            s.reservations_ended,
            fmt_counts(&s.reservations_denied),
            s.circuits_accepted,
            s.circuits_closed,
            fmt_counts(&s.circuits_denied),
            fmt_top_sources(s.denied_sources),
        )
    }
}

/// Why one address of an outbound dial failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DialFailure {
    RelayRateLimited,
    RelayNoReservation,
    RelayRefused,
    Timeout,
    Refused,
    WrongPeer,
    NoAddresses,
    DeniedLocally,
    Other,
}

impl DialFailure {
    fn label(self) -> &'static str {
        match self {
            Self::RelayRateLimited => "relay-rate-limited",
            Self::RelayNoReservation => "relay-no-reservation",
            Self::RelayRefused => "relay-refused",
            Self::Timeout => "timeout",
            Self::Refused => "refused",
            Self::WrongPeer => "wrong-peer",
            Self::NoAddresses => "no-addresses",
            Self::DeniedLocally => "denied-locally",
            Self::Other => "other",
        }
    }
}

/// Classifies a failed dial per attempted address. Our own aborts and
/// "already dialing" conditions are not failures and yield nothing.
pub fn classify(error: &DialError) -> Vec<(bool, DialFailure)> {
    match error {
        DialError::Transport(attempts) => attempts
            .iter()
            .map(|(addr, e)| (is_circuit(addr), classify_text(&error_chain(e))))
            .collect(),
        DialError::WrongPeerId { address, .. } => {
            vec![(is_circuit(address), DialFailure::WrongPeer)]
        }
        DialError::NoAddresses => vec![(false, DialFailure::NoAddresses)],
        DialError::Denied { .. } => vec![(false, DialFailure::DeniedLocally)],
        DialError::LocalPeerId { .. } => vec![(false, DialFailure::Other)],
        DialError::Aborted | DialError::DialPeerConditionFalse(_) => Vec::new(),
    }
}

fn classify_text(text: &str) -> DialFailure {
    let t = text.to_ascii_lowercase();
    if t.contains("resource limit exceeded") {
        DialFailure::RelayRateLimited
    } else if t.contains("no reservation") {
        DialFailure::RelayNoReservation
    } else if t.contains("denied permission") || t.contains("permission denied") {
        DialFailure::RelayRefused
    } else if t.contains("timeout") || t.contains("timed out") {
        DialFailure::Timeout
    } else if t.contains("refused") {
        DialFailure::Refused
    } else {
        DialFailure::Other
    }
}

fn error_chain(e: &(dyn std::error::Error + 'static)) -> String {
    let mut out = e.to_string();
    let mut source = e.source();
    while let Some(s) = source {
        out.push_str(": ");
        out.push_str(&s.to_string());
        source = s.source();
    }
    out
}

/// Outbound dial and hole-punch outcomes since the last summary.
#[derive(Debug, Default)]
pub struct DialStats {
    dials_failed: u64,
    direct: BTreeMap<DialFailure, u64>,
    circuit: BTreeMap<DialFailure, u64>,
    punches_ok: u64,
    punches_failed: u64,
}

impl DialStats {
    pub fn dial_failed(&mut self, error: &DialError) {
        let attempts = classify(error);
        if attempts.is_empty() {
            return;
        }
        self.dials_failed += 1;
        for (circuit, why) in attempts {
            let bucket = if circuit {
                &mut self.circuit
            } else {
                &mut self.direct
            };
            *bucket.entry(why).or_default() += 1;
        }
    }

    pub fn punch(&mut self, ok: bool) {
        if ok {
            self.punches_ok += 1;
        } else {
            self.punches_failed += 1;
        }
    }

    /// The summary line's fields, resetting the window.
    pub fn take(&mut self) -> String {
        let s = std::mem::take(self);
        let fmt = |m: &BTreeMap<DialFailure, u64>| {
            fmt_counts(&m.iter().map(|(k, v)| (k.label().to_string(), *v)).collect())
        };
        format!(
            "dials failed={} circuit-addrs={} direct-addrs={} | hole punches ok={} failed={}",
            s.dials_failed,
            fmt(&s.circuit),
            fmt(&s.direct),
            s.punches_ok,
            s.punches_failed,
        )
    }
}

fn fmt_counts(counts: &BTreeMap<String, u64>) -> String {
    let total: u64 = counts.values().sum();
    if total == 0 {
        return "0".into();
    }
    let parts: Vec<String> = counts.iter().map(|(k, v)| format!("{k}:{v}")).collect();
    format!("{total} ({})", parts.join(" "))
}

fn fmt_top_sources(sources: HashMap<String, u64>) -> String {
    if sources.is_empty() {
        return String::new();
    }
    let mut top: Vec<(String, u64)> = sources.into_iter().collect();
    top.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let shown: Vec<String> = top
        .iter()
        .take(TOP_SOURCES)
        .map(|(who, n)| format!("{who}:{n}"))
        .collect();
    format!(" top-denied-sources=[{}]", shown.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p::core::transport::TransportError;
    use libp2p::relay::StatusCode;
    use libp2p::Multiaddr;

    fn transport(addr: &str, msg: &str) -> (Multiaddr, TransportError<std::io::Error>) {
        (
            addr.parse().unwrap(),
            TransportError::Other(std::io::Error::other(msg.to_string())),
        )
    }

    #[test]
    fn classifies_each_address_and_separates_circuits() {
        let circuit = "/ip4/198.51.100.1/tcp/8000/p2p/QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc/p2p-circuit";
        let err = DialError::Transport(vec![
            transport(
                circuit,
                "Failed to connect to destination.: Remote reported resource limit exceeded.",
            ),
            transport(circuit, "Relay has no reservation for destination."),
            transport("/ip4/192.0.2.1/tcp/8080", "Timeout has been reached"),
        ]);
        let mut stats = DialStats::default();
        stats.dial_failed(&err);
        stats.punch(false);
        let line = stats.take();
        assert!(line.contains("dials failed=1"), "{line}");
        assert!(
            line.contains("circuit-addrs=2 (relay-no-reservation:1 relay-rate-limited:1)"),
            "{line}"
        );
        assert!(line.contains("direct-addrs=1 (timeout:1)"), "{line}");
        assert!(line.contains("hole punches ok=0 failed=1"), "{line}");
        assert!(
            stats.take().contains("dials failed=0"),
            "take resets the window"
        );
    }

    #[test]
    fn our_own_aborts_are_not_failures() {
        let mut stats = DialStats::default();
        stats.dial_failed(&DialError::Aborted);
        assert!(stats.take().starts_with("dials failed=0"));
    }

    #[test]
    fn relay_summary_counts_denials_by_status_and_names_top_sources() {
        let mut stats = RelayServerStats::default();
        let ip: IpAddr = "203.0.113.9".parse().unwrap();
        for _ in 0..3 {
            stats.circuit_denied(
                StatusCode::ResourceLimitExceeded,
                PeerId::random(),
                Some(ip),
            );
        }
        stats.circuit_denied(StatusCode::NoReservation, PeerId::random(), None);
        stats.circuit_accepted();
        stats.reservation_accepted(true);
        let line = stats.take(5);
        assert!(
            line.contains("reservations held=5 new=0 renewed=1"),
            "{line}"
        );
        assert!(
            line.contains(
                "circuits opened=1 closed=0 denied=4 (NoReservation:1 ResourceLimitExceeded:3)"
            ),
            "{line}"
        );
        assert!(
            line.contains("top-denied-sources=[203.0.113.9:3 "),
            "{line}"
        );
    }
}
