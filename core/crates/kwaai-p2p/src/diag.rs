//! Periodic INFO summaries of relay and dial activity.
//!
//! Per-event detail stays at DEBUG/TRACE, where a busy relay or a churning
//! client would flood the log. One line per window is enough to see a relay
//! refusing circuits, or a client failing every hole punch, at the default level.

use std::{
    collections::{BTreeMap, HashMap},
    net::IpAddr,
    time::{Duration, Instant},
};

use libp2p::{core::transport::TransportError, swarm::DialError, PeerId};

use crate::addresses::is_circuit;

/// How often the summaries are logged.
pub const SUMMARY_INTERVAL: Duration = Duration::from_secs(5 * 60);

/// Sources named per summary; the rest are only counted.
const TOP_SOURCES: usize = 3;

/// The span a summary covers: measured, since a stalled loop delays the tick.
#[derive(Debug)]
pub struct SummaryWindow {
    start: Instant,
}

impl SummaryWindow {
    pub fn new(now: Instant) -> Self {
        Self { start: now }
    }

    /// The window just ended, starting the next one at `now`.
    pub fn roll(&mut self, now: Instant) -> Duration {
        now.saturating_duration_since(std::mem::replace(&mut self.start, now))
    }
}

/// Who was denied: peers are named at INFO, their IPs only at DEBUG.
#[derive(Debug, Default)]
struct DeniedSources {
    peers: HashMap<PeerId, u64>,
    ips: HashMap<IpAddr, u64>,
}

impl DeniedSources {
    fn add(&mut self, peer: PeerId, ip: Option<IpAddr>) {
        *self.peers.entry(peer).or_default() += 1;
        if let Some(ip) = ip {
            *self.ips.entry(ip).or_default() += 1;
        }
    }

    /// Distinct IPs are counted, not named, so a per-IP limit still shows.
    fn info(&self) -> String {
        if self.peers.is_empty() {
            return String::new();
        }
        format!(
            " top-denied-peers=[{}] distinct-ips={}",
            fmt_top(&self.peers),
            self.ips.len()
        )
    }
}

/// A relay summary: the INFO fields, and the IP breakdown kept for DEBUG.
#[derive(Debug)]
pub struct RelaySummary {
    pub info: String,
    /// `None` when no denial in the window had a known IP.
    pub ips: Option<String>,
}

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
    reservation_denied_sources: DeniedSources,
    circuit_denied_sources: DeniedSources,
}

impl RelayServerStats {
    pub fn reservation_accepted(&mut self, renewed: bool) {
        if renewed {
            self.reservations_renewed += 1;
        } else {
            self.reservations_accepted += 1;
        }
    }

    /// `src_ip` is recorded for the DEBUG breakdown when its connection is known.
    pub fn reservation_denied(
        &mut self,
        status: impl std::fmt::Debug,
        src: PeerId,
        src_ip: Option<IpAddr>,
    ) {
        *self
            .reservations_denied
            .entry(format!("{status:?}"))
            .or_default() += 1;
        self.reservation_denied_sources.add(src, src_ip);
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

    /// `src_ip` is recorded for the DEBUG breakdown when its connection is known.
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
        self.circuit_denied_sources.add(src, src_ip);
    }

    /// The summary's fields, resetting the window.
    pub fn take(&mut self, peers_with_reservations: usize) -> RelaySummary {
        let s = std::mem::take(self);
        let (r, c) = (&s.reservation_denied_sources, &s.circuit_denied_sources);
        let ips = (!r.ips.is_empty() || !c.ips.is_empty()).then(|| {
            format!(
                "reservations top-denied-ips=[{}] | circuits top-denied-ips=[{}]",
                fmt_top(&r.ips),
                fmt_top(&c.ips)
            )
        });
        let info = format!(
            "reservations peers-holding={peers_with_reservations} new={} renewed={} ended={} \
             denied={}{} | circuits opened={} closed={} denied={}{}",
            s.reservations_accepted,
            s.reservations_renewed,
            s.reservations_ended,
            fmt_counts(&s.reservations_denied),
            r.info(),
            s.circuits_accepted,
            s.circuits_closed,
            fmt_counts(&s.circuits_denied),
            c.info(),
        );
        RelaySummary { info, ips }
    }
}

/// Why one address of an outbound dial failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DialFailure {
    RelayRateLimited,
    RelayNoReservation,
    RelayRefused,
    RelayCannotReachPeer,
    RelayUnsupported,
    Timeout,
    Refused,
    Unreachable,
    WrongPeer,
    AddrNotSupported,
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
            Self::RelayCannotReachPeer => "relay-cannot-reach-peer",
            Self::RelayUnsupported => "relay-unsupported",
            Self::Timeout => "timeout",
            Self::Refused => "refused",
            Self::Unreachable => "unreachable",
            Self::WrongPeer => "wrong-peer",
            Self::AddrNotSupported => "addr-not-supported",
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
            .map(|(addr, e)| {
                let why = match e {
                    // No enabled transport takes it, e.g. a QUIC address with QUIC off.
                    TransportError::MultiaddrNotSupported(_) => DialFailure::AddrNotSupported,
                    TransportError::Other(io) => classify_error(io),
                };
                (is_circuit(addr), why)
            })
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

/// Walks the error chain for the relay's typed refusal or an I/O error kind;
/// text matching is only the fallback. `io::Error::source` skips a custom
/// error's own frame, so its `get_ref` is followed instead.
fn classify_error(e: &(dyn std::error::Error + 'static)) -> DialFailure {
    use libp2p::relay::outbound::hop::ConnectError;
    use std::io::ErrorKind;

    let mut cur: Option<&(dyn std::error::Error + 'static)> = Some(e);
    while let Some(err) = cur {
        if let Some(c) = err.downcast_ref::<ConnectError>() {
            match c {
                ConnectError::ResourceLimitExceeded => return DialFailure::RelayRateLimited,
                ConnectError::NoReservation => return DialFailure::RelayNoReservation,
                ConnectError::PermissionDenied => return DialFailure::RelayRefused,
                ConnectError::ConnectionFailed => return DialFailure::RelayCannotReachPeer,
                ConnectError::Unsupported => return DialFailure::RelayUnsupported,
                ConnectError::Io(_) | ConnectError::Protocol(_) => {}
            }
        }
        if let Some(io) = err.downcast_ref::<std::io::Error>() {
            match io.kind() {
                ErrorKind::TimedOut => return DialFailure::Timeout,
                ErrorKind::ConnectionRefused => return DialFailure::Refused,
                ErrorKind::NetworkUnreachable | ErrorKind::HostUnreachable => {
                    return DialFailure::Unreachable
                }
                _ => {}
            }
            if let Some(inner) = io.get_ref() {
                cur = Some(inner);
                continue;
            }
        }
        cur = err.source();
    }
    classify_text(&error_chain(e))
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

/// Outbound dial and hole-punch outcomes since the last summary. A failed
/// punch's own dials also count as failed dials; punch counts are dcutr events.
#[derive(Debug, Default)]
pub struct DialStats {
    punch_failures_logged: std::collections::HashSet<PeerId>,
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

    /// Whether this is `peer`'s first failed punch in the window, the one worth an INFO line.
    pub fn first_punch_failure(&mut self, peer: PeerId) -> bool {
        self.punch_failures_logged.insert(peer)
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

fn fmt_top<K: ToString>(sources: &HashMap<K, u64>) -> String {
    let mut top: Vec<(String, u64)> = sources.iter().map(|(k, v)| (k.to_string(), *v)).collect();
    top.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let shown: Vec<String> = top
        .iter()
        .take(TOP_SOURCES)
        .map(|(who, n)| format!("{who}:{n}"))
        .collect();
    shown.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn classifies_the_relay_error_as_the_transport_stack_wraps_it() {
        use libp2p::relay::{client::transport::Error, outbound::hop::ConnectError};
        let circuit = "/ip4/198.51.100.1/tcp/8000/p2p/QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc/p2p-circuit";
        let wrapped = |c: ConnectError| {
            (
                circuit.parse().unwrap(),
                TransportError::Other(std::io::Error::other(Error::Connect(c))),
            )
        };
        let err = DialError::Transport(vec![
            wrapped(ConnectError::NoReservation),
            wrapped(ConnectError::Unsupported),
            (
                "/ip4/192.0.2.1/tcp/8080".parse().unwrap(),
                TransportError::Other(std::io::Error::from(std::io::ErrorKind::ConnectionRefused)),
            ),
        ]);
        assert_eq!(
            classify(&err),
            vec![
                (true, DialFailure::RelayNoReservation),
                (true, DialFailure::RelayUnsupported),
                (false, DialFailure::Refused),
            ]
        );
    }

    #[test]
    fn denials_name_peers_at_info_and_ips_only_in_the_debug_breakdown() {
        let mut stats = RelayServerStats::default();
        let ip: IpAddr = "203.0.113.9".parse().unwrap();
        let peer = PeerId::random();
        stats.reservation_denied(StatusCode::ResourceLimitExceeded, peer, Some(ip));
        let summary = stats.take(0);
        assert!(
            summary.info.contains(&format!(
                "denied=1 (ResourceLimitExceeded:1) top-denied-peers=[{peer}:1] distinct-ips=1"
            )),
            "{}",
            summary.info
        );
        assert!(!summary.info.contains("203.0.113.9"), "{}", summary.info);
        assert_eq!(
            summary.ips.as_deref(),
            Some("reservations top-denied-ips=[203.0.113.9:1] | circuits top-denied-ips=[]")
        );
    }

    #[test]
    fn no_ip_breakdown_without_a_known_ip() {
        let mut stats = RelayServerStats::default();
        stats.circuit_denied(StatusCode::NoReservation, PeerId::random(), None);
        let summary = stats.take(0);
        assert!(summary.info.contains("distinct-ips=0"), "{}", summary.info);
        assert!(summary.ips.is_none());
    }

    #[test]
    fn window_reports_the_real_elapsed_span_after_a_stall() {
        let t0 = Instant::now();
        let mut window = SummaryWindow::new(t0);
        let stalled = t0 + SUMMARY_INTERVAL + Duration::from_secs(120);
        assert_eq!(
            window.roll(stalled),
            SUMMARY_INTERVAL + Duration::from_secs(120)
        );
        let next = stalled + SUMMARY_INTERVAL;
        assert_eq!(
            window.roll(next),
            SUMMARY_INTERVAL,
            "roll restarts the window"
        );
    }

    #[test]
    fn an_address_no_transport_takes_is_named() {
        let quic: Multiaddr = "/ip4/198.51.100.1/udp/8000/quic-v1/p2p/QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc/p2p-circuit"
            .parse()
            .unwrap();
        let err = DialError::Transport(vec![(
            quic.clone(),
            TransportError::MultiaddrNotSupported(quic),
        )]);
        assert_eq!(classify(&err), vec![(true, DialFailure::AddrNotSupported)]);
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
        let noisy = PeerId::random();
        for _ in 0..3 {
            stats.circuit_denied(StatusCode::ResourceLimitExceeded, noisy, Some(ip));
        }
        stats.circuit_denied(StatusCode::NoReservation, PeerId::random(), None);
        stats.reservation_denied(
            StatusCode::ResourceLimitExceeded,
            PeerId::random(),
            Some(ip),
        );
        stats.circuit_accepted();
        stats.reservation_accepted(true);
        let line = stats.take(5).info;
        assert!(
            line.contains("reservations peers-holding=5 new=0 renewed=1"),
            "{line}"
        );
        assert!(
            line.contains(
                "circuits opened=1 closed=0 denied=4 (NoReservation:1 ResourceLimitExceeded:3)"
            ),
            "{line}"
        );
        assert!(
            line.contains(&format!("top-denied-peers=[{noisy}:3 ")),
            "{line}"
        );
    }
}
