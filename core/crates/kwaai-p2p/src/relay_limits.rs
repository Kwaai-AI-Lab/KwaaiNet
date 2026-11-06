//! Circuit-request rate limiters for the relay hop server.
//!
//! libp2p keeps its own token-bucket constructors private, so a relay that
//! wants anything but the default limits has to supply its own.

use std::{
    collections::HashMap,
    hash::Hash,
    net::IpAddr,
    num::NonZeroU32,
    time::{Duration, Instant},
};

use libp2p::{multiaddr::Protocol, relay::RateLimiter, Multiaddr, PeerId};

/// Above this many tracked keys, full buckets are dropped; a missing key reads as full.
const PRUNE_ABOVE: usize = 4096;

/// libp2p's per-peer default: 30 circuits in a burst, one more every 2 minutes.
const DEFAULT_PER_PEER_BURST: NonZeroU32 = NonZeroU32::new(30).unwrap();
const DEFAULT_PER_PEER_REFILL: Duration = Duration::from_secs(2 * 60);

struct TokenBucket<K> {
    burst: u32,
    refill: Duration,
    buckets: HashMap<K, (u32, Instant)>,
}

impl<K: Eq + Hash> TokenBucket<K> {
    fn new(burst: NonZeroU32, refill: Duration) -> Self {
        Self {
            burst: burst.get(),
            // libp2p asserts on a zero interval; clamp instead of panicking.
            refill: refill.max(Duration::from_millis(1)),
            buckets: HashMap::new(),
        }
    }

    fn level(&self, (tokens, since): (u32, Instant), now: Instant) -> (u32, Instant) {
        let earned = now.saturating_duration_since(since).as_nanos() / self.refill.as_nanos();
        if u128::from(tokens) + earned >= u128::from(self.burst) {
            return (self.burst, now);
        }
        // earned < burst here, so both casts are lossless.
        (tokens + earned as u32, since + self.refill * earned as u32)
    }

    fn take(&mut self, key: K, now: Instant) -> bool {
        if self.buckets.len() > PRUNE_ABOVE {
            let full = |b: &Self, v: (u32, Instant)| b.level(v, now).0 >= b.burst;
            let stale: Vec<bool> = self.buckets.values().map(|v| full(self, *v)).collect();
            let mut stale = stale.into_iter();
            self.buckets.retain(|_, _| !stale.next().unwrap_or(false));
        }
        let current = self.buckets.get(&key).copied().unwrap_or((self.burst, now));
        let (tokens, since) = self.level(current, now);
        let granted = tokens > 0;
        let left = if granted { tokens - 1 } else { 0 };
        self.buckets.insert(key, (left, since));
        granted
    }
}

/// Limits circuit requests per source peer.
pub fn per_peer(burst: NonZeroU32, refill: Duration) -> Box<dyn RateLimiter> {
    let mut bucket = TokenBucket::new(burst, refill);
    Box::new(move |peer: PeerId, _: &Multiaddr, now: Instant| bucket.take(peer, now))
}

/// libp2p's own per-peer limit, which the hop server keeps.
pub fn per_peer_default() -> Box<dyn RateLimiter> {
    per_peer(DEFAULT_PER_PEER_BURST, DEFAULT_PER_PEER_REFILL)
}

/// Limits circuit requests per source IP, shared by every peer behind that address.
/// A zero `burst` is treated as one.
pub fn per_ip(burst: u32, refill: Duration) -> Box<dyn RateLimiter> {
    let mut bucket = TokenBucket::new(NonZeroU32::new(burst).unwrap_or(NonZeroU32::MIN), refill);
    Box::new(move |_: PeerId, addr: &Multiaddr, now: Instant| {
        ip_of(addr).is_none_or(|ip| bucket.take(ip, now))
    })
}

fn ip_of(addr: &Multiaddr) -> Option<IpAddr> {
    addr.iter().find_map(|p| match p {
        Protocol::Ip4(a) => Some(a.into()),
        Protocol::Ip6(a) => Some(a.into()),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nz(n: u32) -> NonZeroU32 {
        NonZeroU32::new(n).unwrap()
    }

    fn addr(s: &str) -> Multiaddr {
        s.parse().unwrap()
    }

    #[test]
    fn grants_the_burst_then_refuses() {
        let mut b = TokenBucket::new(nz(3), Duration::from_secs(60));
        let t = Instant::now();
        assert!((0..3).all(|_| b.take("k", t)));
        assert!(!b.take("k", t));
    }

    #[test]
    fn refills_one_token_per_interval_and_caps_at_the_burst() {
        let mut b = TokenBucket::new(nz(2), Duration::from_secs(60));
        let t = Instant::now();
        assert!(b.take("k", t) && b.take("k", t));
        assert!(!b.take("k", t + Duration::from_secs(59)));
        assert!(b.take("k", t + Duration::from_secs(60)));
        assert!(!b.take("k", t + Duration::from_secs(61)));
        // A long idle spell refills to the burst, never past it.
        let later = t + Duration::from_secs(3600);
        assert!(b.take("k", later) && b.take("k", later));
        assert!(!b.take("k", later));
    }

    #[test]
    fn a_refused_request_does_not_reset_the_refill_clock() {
        let mut b = TokenBucket::new(nz(1), Duration::from_secs(60));
        let t = Instant::now();
        assert!(b.take("k", t));
        for s in [10, 20, 30, 59] {
            assert!(!b.take("k", t + Duration::from_secs(s)));
        }
        assert!(b.take("k", t + Duration::from_secs(60)));
    }

    #[test]
    fn per_ip_shares_one_budget_across_peers_behind_an_address() {
        let mut limiter = per_ip(2, Duration::from_secs(60));
        let home = addr("/ip4/98.51.100.7/tcp/4001");
        let t = Instant::now();
        assert!(limiter.try_next(PeerId::random(), &home, t));
        assert!(limiter.try_next(PeerId::random(), &home, t));
        assert!(!limiter.try_next(PeerId::random(), &home, t));
        assert!(limiter.try_next(PeerId::random(), &addr("/ip4/98.51.100.8/tcp/4001"), t));
    }

    #[test]
    fn per_ip_allows_an_address_with_no_ip() {
        let mut limiter = per_ip(1, Duration::from_secs(60));
        let dns = addr("/dns4/example.org/tcp/4001");
        let t = Instant::now();
        assert!((0..5).all(|_| limiter.try_next(PeerId::random(), &dns, t)));
    }

    #[test]
    fn per_peer_keys_on_the_peer_not_the_address() {
        let mut limiter = per_peer(nz(1), Duration::from_secs(60));
        let (a, b) = (PeerId::random(), PeerId::random());
        let home = addr("/ip4/98.51.100.7/tcp/4001");
        let t = Instant::now();
        assert!(limiter.try_next(a, &home, t));
        assert!(!limiter.try_next(a, &home, t));
        assert!(limiter.try_next(b, &home, t));
    }

    #[test]
    fn pruning_forgets_only_full_buckets() {
        let mut b = TokenBucket::new(nz(1), Duration::from_secs(60));
        let t = Instant::now();
        for i in 0..=PRUNE_ABOVE {
            b.take(i, t);
        }
        // All drained, so none is pruned and the drained key stays refused.
        assert!(!b.take(0, t + Duration::from_secs(1)));
        // Once refilled they are full, so the next oversized call prunes them.
        let later = t + Duration::from_secs(120);
        b.take(usize::MAX, later);
        assert!(b.buckets.len() <= 2);
    }
}
