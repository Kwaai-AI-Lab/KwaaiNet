//! Dual-stack binds for the node's own local servers.
//!
//! The swarm's listeners are libp2p's problem; these are ours — the gRPC
//! control port, the OpenAI-compatible HTTP APIs, the storage health endpoint.
//! Every one of them bound a single IPv4 socket, so a client reaching for
//! `localhost` on a host where that resolves to `::1` first got connection
//! refused from a daemon that was running perfectly well.
//!
//! Two sockets rather than one v6 socket accepting mapped v4: `IPV6_V6ONLY`
//! defaults differ across platforms (Windows and most BSDs force it on), so a
//! dual-stack socket is portable only in the direction that does not help us.
//! Binding both explicitly is the same shape libp2p-tcp uses, and it is why
//! both sockets can share one port number.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpListener};

use anyhow::{Context, Result};
use kwaai_p2p::Ipv6Mode;
use socket2::{Domain, Protocol, Socket, Type};
use tracing::warn;

/// Backlog depth for the v6 twin. `std`'s own bind uses 128; 1024 matches what
/// the gRPC control port wants when a supervisor restarts a fleet at once.
const BACKLOG: i32 = 1024;

/// Which addresses a server should answer on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scope {
    /// This host only: `127.0.0.1` and `::1`.
    Loopback,
    /// Every interface: `0.0.0.0` and `::`.
    Any,
}

impl Scope {
    fn v4(self) -> Ipv4Addr {
        match self {
            Scope::Loopback => Ipv4Addr::LOCALHOST,
            Scope::Any => Ipv4Addr::UNSPECIFIED,
        }
    }

    fn v6(self) -> Ipv6Addr {
        match self {
            Scope::Loopback => Ipv6Addr::LOCALHOST,
            Scope::Any => Ipv6Addr::UNSPECIFIED,
        }
    }
}

/// A bound pair. `v4` always exists; `v6` is absent when disabled or refused.
pub struct Bound {
    pub v4: TcpListener,
    pub v6: Option<TcpListener>,
    /// The resolved port, which both listeners share. Meaningful when the
    /// caller asked for `0`.
    pub port: u16,
}

/// Bind IPv4 and, unless disabled, its IPv6 twin on the same port.
///
/// The v4 bind happens first and its failure is the caller's error verbatim —
/// that is the behaviour every one of these servers already had. The v6 twin is
/// built on the *resolved* port, so `port: 0` still yields one number for both.
pub fn bind_dual_stack(scope: Scope, port: u16, mode: Ipv6Mode) -> Result<Bound> {
    let v4 = TcpListener::bind(SocketAddr::new(IpAddr::V4(scope.v4()), port))?;
    let port = v4.local_addr().context("reading the bound v4 port")?.port();

    if mode.is_off() {
        return Ok(Bound { v4, v6: None, port });
    }

    match bind_v6(scope, port) {
        Ok(v6) => Ok(Bound {
            v4,
            v6: Some(v6),
            port,
        }),
        Err(e) if mode == Ipv6Mode::On => {
            Err(e.context(format!("ipv6 is required but binding [::]:{port} failed")))
        }
        Err(e) => {
            warn!("IPv6 unavailable on port {port} ({e:#}); serving IPv4 only");
            Ok(Bound { v4, v6: None, port })
        }
    }
}

/// The v6 half, always `IPV6_V6ONLY` so it cannot swallow the v4 bind.
fn bind_v6(scope: Scope, port: u16) -> Result<TcpListener> {
    let sock = Socket::new(Domain::IPV6, Type::STREAM, Some(Protocol::TCP))
        .context("creating the IPv6 socket")?;
    sock.set_only_v6(true).context("setting IPV6_V6ONLY")?;
    // Not on Windows, where SO_REUSEADDR lets a *live* listener be stolen.
    #[cfg(unix)]
    sock.set_reuse_address(true)
        .context("setting SO_REUSEADDR")?;
    sock.bind(&SocketAddr::new(IpAddr::V6(scope.v6()), port).into())
        .with_context(|| format!("binding [{}]:{port}", scope.v6()))?;
    sock.listen(BACKLOG).context("listening")?;
    Ok(sock.into())
}

impl Bound {
    /// Hand both listeners to tokio, IPv4 first.
    pub fn into_tokio(self) -> Result<Vec<tokio::net::TcpListener>> {
        let mut out = Vec::with_capacity(2);
        for std_listener in std::iter::once(self.v4).chain(self.v6) {
            std_listener
                .set_nonblocking(true)
                .context("set_nonblocking")?;
            out.push(
                tokio::net::TcpListener::from_std(std_listener).context("adopting the listener")?,
            );
        }
        Ok(out)
    }
}

/// Whether `port` is free on every family this node would bind.
///
/// Only `AddrInUse` counts as taken: a bind refused for any other reason (a
/// privileged port, a host with no v6 stack) is not something the caller can
/// fix by picking a different port, and reporting it as "in use" would send
/// them chasing a squatter that does not exist.
pub fn port_is_free(port: u16, mode: Ipv6Mode) -> bool {
    let in_use = |e: &std::io::Error| e.kind() == std::io::ErrorKind::AddrInUse;

    if let Err(e) = TcpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, port))) {
        if in_use(&e) {
            return false;
        }
    }
    if mode.is_off() {
        return true;
    }
    match bind_v6(Scope::Any, port) {
        Ok(_) => true,
        Err(e) => !e
            .chain()
            .any(|c| c.downcast_ref::<std::io::Error>().is_some_and(in_use)),
    }
}

/// The IPv6 mode from the on-disk config, for the servers that are started
/// outside the node process and so have no config in hand.
///
/// `auto` when the config cannot be read: a v6 twin that will not bind is a
/// warning, never a reason to refuse to serve at all.
pub fn configured_ipv6_mode() -> Ipv6Mode {
    crate::config::KwaaiNetConfig::load_or_create()
        .map(|c| c.ipv6)
        .unwrap_or(Ipv6Mode::Auto)
        .effective()
}

/// Whether a host or URL points back at this machine.
///
/// Substring matching, which is what the callers that predate this function
/// did — they compare against a configured `inference_url`, not a parsed one.
/// A bare `::1` has to be an exact match rather than a substring, or every
/// address ending `::1<hex>` (`[2606:4700::1111]`) reads as loopback.
pub fn is_loopback_host(host_or_url: &str) -> bool {
    let s = host_or_url.trim();
    s.contains("localhost") || s.contains("127.0.0.1") || s.contains("[::1]") || s == "::1"
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::net::TcpStream;

    /// Both halves have to answer, and on the *same* port — a v6 twin on a
    /// different number is not the thing that fixes `localhost` resolving to
    /// `::1`.
    #[test]
    fn a_loopback_pair_shares_one_port_and_both_accept() {
        let bound = bind_dual_stack(Scope::Loopback, 0, Ipv6Mode::Auto).expect("v4 must bind");
        let port = bound.port;
        assert_eq!(bound.v4.local_addr().unwrap().port(), port);

        assert!(TcpStream::connect(("127.0.0.1", port)).is_ok());

        match &bound.v6 {
            Some(v6) => {
                assert_eq!(v6.local_addr().unwrap().port(), port, "same port");
                let mut s = TcpStream::connect(format!("[::1]:{port}"))
                    .expect("the v6 twin should accept a connection");
                let _ = s.write_all(b"x");
            }
            None => println!("skipping v6 assertions: no IPv6 loopback on this host"),
        }
    }

    #[test]
    fn off_binds_no_v6_twin() {
        let bound = bind_dual_stack(Scope::Loopback, 0, Ipv6Mode::Off).expect("v4 must bind");
        assert!(bound.v6.is_none());
        assert!(TcpStream::connect(format!("[::1]:{}", bound.port)).is_err());
    }

    /// `port_is_free` gates the friendly "already running" message, so a false
    /// negative sends an operator hunting a squatter that is not there.
    #[test]
    fn a_port_is_free_only_once_the_listener_is_gone() {
        let bound = bind_dual_stack(Scope::Any, 0, Ipv6Mode::Auto).expect("bind");
        let port = bound.port;
        assert!(!port_is_free(port, Ipv6Mode::Auto));
        drop(bound);
        assert!(port_is_free(port, Ipv6Mode::Auto));
    }

    /// Callers that report "the" bound address take the first listener, so the
    /// order is part of the contract, not an accident of iteration.
    #[test]
    fn into_tokio_keeps_v4_first() {
        let bound = bind_dual_stack(Scope::Loopback, 0, Ipv6Mode::Auto).expect("bind");
        let expected = if bound.v6.is_some() { 2 } else { 1 };
        let rt = tokio::runtime::Runtime::new().expect("runtime");
        let listeners = rt.block_on(async { bound.into_tokio().expect("adopt") });
        assert_eq!(listeners.len(), expected);
        assert!(listeners[0].local_addr().unwrap().is_ipv4(), "v4 first");
    }

    #[test]
    fn loopback_hosts_are_recognised_without_swallowing_global_v6() {
        for host in [
            "localhost",
            "http://localhost:11434",
            "127.0.0.1",
            "http://127.0.0.1:8080/v1",
            "http://[::1]:8080/v1",
            "::1",
        ] {
            assert!(is_loopback_host(host), "{host} should read as loopback");
        }
        for host in [
            "p2p://auto",
            "http://198.18.0.20:8080",
            // The trap a bare `::1` substring test falls into.
            "http://[2606:4700::1111]:8080",
        ] {
            assert!(!is_loopback_host(host), "{host} should read as remote");
        }
    }
}
