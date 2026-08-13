//! Remembering peers across restarts, so a node can rejoin without its config.
//!
//! `initial_peers` is a list written once, at install time. It ages: addresses
//! change, hosts are retired, and a node restarted a year later may hold a list
//! where nothing answers — even though it spent that year connected to dozens
//! of peers that would have let it back in. The cache is that knowledge,
//! persisted.
//!
//! On startup the cached peers are dial candidates *alongside* `initial_peers`,
//! not instead of them; on a 60 s timer and again at shutdown the current peer
//! set is written back.
//!
//! # Bounded, and ordered by recency
//!
//! At most [`MAX_CACHED_PEERS`] entries survive a save, the most recently seen
//! first. An unbounded cache on a long-lived node would grow into a file of
//! mostly-dead addresses that slow every startup down; keeping the newest is
//! the cheap approximation of keeping the reachable.
//!
//! # Trust
//!
//! Entries are unsigned and carry no attestation, so this file says only "these
//! addresses answered once". A tampered cache can therefore make a node dial
//! peers of an attacker's choosing — which costs a failed dial and nothing
//! more, because a peer's identity is still proven by the libp2p handshake
//! against the `/p2p/<id>` in the address, and the DHT records it serves carry
//! their own expirations. Signing the cache is deliberately out of scope for
//! this phase.
//!
//! # Failure is never fatal
//!
//! A missing, truncated, or malformed cache loads as empty and is logged at
//! debug. The node then starts exactly as it would have without the feature —
//! the file is an optimization, and treating a bad one as an error would turn a
//! corrupt optimization into an outage.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tracing::debug;

/// How many peers survive a save, newest first.
pub const MAX_CACHED_PEERS: usize = 100;

/// How often the cache is rewritten while the node runs.
pub const CACHE_WRITE_INTERVAL_SECS: u64 = 60;

/// One remembered peer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CachedPeer {
    /// Base58 peer ID.
    pub peer_id: String,
    /// Multiaddrs seen for this peer, each already carrying `/p2p/<id>` so it
    /// can be dialed exactly as an `initial_peers` entry is.
    pub addrs: Vec<String>,
    /// Unix seconds when this peer was last observed. Drives both the recency
    /// bound and the eventual staleness display; never used to expire an entry
    /// outright, because an old address that still answers is still useful.
    pub last_seen: u64,
}

/// The on-disk document.
///
/// A struct rather than a bare array so a later phase can add fields (a
/// signature, a schema version) without invalidating every existing file —
/// serde's `default` handles the reverse direction already.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PeerCache {
    #[serde(default)]
    pub peers: Vec<CachedPeer>,
}

/// Unix seconds now.
fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// The cache path inside a config directory.
pub fn cache_path_in(dir: &Path) -> PathBuf {
    dir.join("peer-cache.json")
}

/// The default cache path: `<kwaainet_dir>/peer-cache.json`.
pub fn default_cache_path() -> PathBuf {
    cache_path_in(&crate::config::kwaainet_dir())
}

impl PeerCache {
    /// Load the cache at `path`, or an empty cache when it is missing or
    /// unreadable.
    ///
    /// Never returns an error: see the module docs on why a bad cache must not
    /// stop a node from starting.
    pub fn load(path: &Path) -> Self {
        let Ok(text) = std::fs::read_to_string(path) else {
            debug!("No peer cache at {} — starting with none", path.display());
            return Self::default();
        };
        match serde_json::from_str::<Self>(&text) {
            Ok(cache) => {
                debug!(
                    "Loaded {} cached peer(s) from {}",
                    cache.peers.len(),
                    path.display()
                );
                cache
            }
            Err(e) => {
                debug!("Ignoring malformed peer cache at {}: {e}", path.display());
                Self::default()
            }
        }
    }

    /// Load from the default path.
    pub fn load_default() -> Self {
        Self::load(&default_cache_path())
    }

    /// Write the cache to `path`, newest first and bounded to
    /// [`MAX_CACHED_PEERS`].
    ///
    /// The parent directory is created if needed, so a fresh install saves
    /// without a separate setup step. The write is atomic via a temp file and
    /// rename: a node killed mid-write leaves the previous cache intact rather
    /// than a half-written file that would load as empty.
    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let mut bounded = self.clone();
        bounded.sort_and_bound();

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(&bounded)?;

        let tmp = path.with_extension("json.tmp");
        std::fs::write(&tmp, json)?;
        std::fs::rename(&tmp, path)?;
        Ok(())
    }

    /// Save to the default path, logging rather than propagating a failure —
    /// callers are shutdown paths and timers where there is nothing to do about
    /// it.
    pub fn save_default_quietly(&self) {
        if let Err(e) = self.save(&default_cache_path()) {
            debug!("Could not write the peer cache: {e}");
        }
    }

    /// Newest first, then truncated to the bound.
    fn sort_and_bound(&mut self) {
        self.peers.sort_by(|a, b| b.last_seen.cmp(&a.last_seen));
        self.peers.truncate(MAX_CACHED_PEERS);
    }

    /// Fold a currently-observed peer into the cache, stamped now.
    ///
    /// An existing entry is refreshed rather than duplicated: its addresses are
    /// unioned (an address that worked before may work again from a different
    /// network) and its `last_seen` moves forward.
    pub fn observe(&mut self, peer_id: &str, addrs: Vec<String>) {
        let now = now_secs();
        if let Some(existing) = self.peers.iter_mut().find(|p| p.peer_id == peer_id) {
            for addr in addrs {
                if !existing.addrs.contains(&addr) {
                    existing.addrs.push(addr);
                }
            }
            existing.last_seen = now;
            return;
        }
        self.peers.push(CachedPeer {
            peer_id: peer_id.to_string(),
            addrs,
            last_seen: now,
        });
    }

    /// Replace the cache's view with the peers currently known, keeping
    /// remembered peers that are merely not connected right now.
    ///
    /// This is what the periodic writer calls: a peer we cannot see this minute
    /// is not evidence the peer is gone, so it stays — the recency bound, not
    /// liveness, is what eventually drops it.
    pub fn observe_all(&mut self, peers: &[(String, Vec<String>)]) {
        for (peer_id, addrs) in peers {
            self.observe(peer_id, addrs.clone());
        }
    }

    /// Dialable multiaddrs from the cache, newest peer first, excluding any
    /// already present in `have`.
    ///
    /// The exclusion keeps the startup dial set from listing a configured peer
    /// twice when it is also remembered.
    pub fn dial_addrs(&self, have: &HashSet<String>) -> Vec<String> {
        let mut sorted = self.peers.clone();
        sorted.sort_by(|a, b| b.last_seen.cmp(&a.last_seen));
        sorted
            .iter()
            .flat_map(|p| p.addrs.iter())
            .filter(|a| !have.contains(*a))
            .cloned()
            .collect()
    }

    /// How many peers are remembered.
    pub fn len(&self) -> usize {
        self.peers.len()
    }

    /// Whether nothing is remembered.
    ///
    /// Currently only the tests ask — the node paths care about the dial list,
    /// not the count — but the pair with `len` belongs on the type.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.peers.is_empty()
    }

    /// Whether `peer_id` is remembered.
    #[allow(dead_code)]
    pub fn contains(&self, peer_id: &str) -> bool {
        self.peers.iter().any(|p| p.peer_id == peer_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn peer(id: &str, last_seen: u64) -> CachedPeer {
        CachedPeer {
            peer_id: id.to_string(),
            addrs: vec![format!("/ip4/10.0.0.1/tcp/8000/p2p/{id}")],
            last_seen,
        }
    }

    // ── Round trip ──────────────────────────────────────────────────────────

    /// The property the cache exists for: what was written comes back.
    #[test]
    fn a_saved_cache_loads_back_identically() {
        let dir = TempDir::new().unwrap();
        let path = cache_path_in(dir.path());

        let cache = PeerCache {
            peers: vec![peer("alice", 100), peer("bob", 200)],
        };
        cache.save(&path).unwrap();

        let loaded = PeerCache::load(&path);
        assert_eq!(loaded.len(), 2);
        assert!(loaded.contains("alice"));
        assert!(loaded.contains("bob"));
        // Addresses survive intact — they are dialed verbatim.
        let alice = loaded.peers.iter().find(|p| p.peer_id == "alice").unwrap();
        assert_eq!(alice.addrs, vec!["/ip4/10.0.0.1/tcp/8000/p2p/alice"]);
    }

    /// Saving creates the config directory, so a fresh install needs no setup.
    #[test]
    fn saving_creates_a_missing_directory() {
        let dir = TempDir::new().unwrap();
        let path = cache_path_in(&dir.path().join("nested").join("deeper"));

        PeerCache {
            peers: vec![peer("alice", 1)],
        }
        .save(&path)
        .unwrap();

        assert!(path.exists());
        assert_eq!(PeerCache::load(&path).len(), 1);
    }

    // ── The bound ───────────────────────────────────────────────────────────

    /// At most 100 peers survive, and they are the 100 most recently seen — an
    /// unbounded cache would accumulate dead addresses forever.
    #[test]
    fn the_cache_is_bounded_to_the_hundred_most_recent() {
        let dir = TempDir::new().unwrap();
        let path = cache_path_in(dir.path());

        // 150 peers whose last_seen increases with their index.
        let cache = PeerCache {
            peers: (0..150)
                .map(|i| peer(&format!("peer{i}"), i as u64))
                .collect(),
        };
        cache.save(&path).unwrap();

        let loaded = PeerCache::load(&path);
        assert_eq!(loaded.len(), MAX_CACHED_PEERS);
        // The newest (149) is kept, the oldest (0) dropped.
        assert!(loaded.contains("peer149"), "the newest peer must survive");
        assert!(loaded.contains("peer50"), "the 100th-newest must survive");
        assert!(!loaded.contains("peer49"), "the 101st-newest is dropped");
        assert!(!loaded.contains("peer0"), "the oldest must be dropped");
    }

    /// The saved order is newest-first, so a reader that takes a prefix takes
    /// the freshest peers.
    #[test]
    fn saved_peers_are_ordered_newest_first() {
        let dir = TempDir::new().unwrap();
        let path = cache_path_in(dir.path());

        PeerCache {
            peers: vec![peer("old", 10), peer("newest", 300), peer("mid", 100)],
        }
        .save(&path)
        .unwrap();

        let ids: Vec<String> = PeerCache::load(&path)
            .peers
            .iter()
            .map(|p| p.peer_id.clone())
            .collect();
        assert_eq!(ids, vec!["newest", "mid", "old"]);
    }

    // ── Tolerating bad input ────────────────────────────────────────────────

    /// A missing file is an empty cache, not an error — the ordinary first-run
    /// state.
    #[test]
    fn a_missing_cache_loads_empty() {
        let dir = TempDir::new().unwrap();
        let cache = PeerCache::load(&cache_path_in(dir.path()));
        assert!(cache.is_empty());
    }

    /// Malformed JSON is ignored rather than fatal: a corrupt optimization must
    /// not stop the node from starting.
    #[test]
    fn a_malformed_cache_loads_empty_rather_than_failing() {
        let dir = TempDir::new().unwrap();
        let path = cache_path_in(dir.path());

        for bad in [
            "not json at all",
            "{",
            "[]",                           // right JSON, wrong shape
            r#"{"peers": "not-an-array"}"#, // right key, wrong type
            "",
        ] {
            std::fs::write(&path, bad).unwrap();
            assert!(
                PeerCache::load(&path).is_empty(),
                "malformed cache {bad:?} must load as empty"
            );
        }
    }

    /// A cache with unknown extra fields still loads — forward compatibility
    /// with a later phase that adds a signature.
    #[test]
    fn unknown_fields_are_tolerated() {
        let dir = TempDir::new().unwrap();
        let path = cache_path_in(dir.path());
        std::fs::write(
            &path,
            r#"{"peers":[{"peer_id":"alice","addrs":["/a"],"last_seen":5,"signature":"xx"}],
                "schema":2}"#,
        )
        .unwrap();

        let loaded = PeerCache::load(&path);
        assert_eq!(loaded.len(), 1);
        assert!(loaded.contains("alice"));
    }

    /// A half-written file cannot replace a good one: the save is atomic, so an
    /// interrupted write leaves the previous cache readable.
    #[test]
    fn a_leftover_temp_file_does_not_corrupt_the_cache() {
        let dir = TempDir::new().unwrap();
        let path = cache_path_in(dir.path());

        PeerCache {
            peers: vec![peer("alice", 1)],
        }
        .save(&path)
        .unwrap();

        // A crashed write leaves the .tmp behind; the real file is untouched.
        std::fs::write(path.with_extension("json.tmp"), "half-written {{{").unwrap();

        assert!(PeerCache::load(&path).contains("alice"));
    }

    // ── Observation ─────────────────────────────────────────────────────────

    /// Observing a new peer adds it; observing a known one refreshes rather
    /// than duplicating.
    #[test]
    fn observing_a_known_peer_refreshes_it_in_place() {
        let mut cache = PeerCache::default();
        cache.observe("alice", vec!["/addr-1".to_string()]);
        assert_eq!(cache.len(), 1);

        cache.observe("alice", vec!["/addr-2".to_string()]);
        assert_eq!(cache.len(), 1, "the same peer must not be duplicated");

        let alice = &cache.peers[0];
        assert_eq!(
            alice.addrs,
            vec!["/addr-1", "/addr-2"],
            "addresses are unioned — an old one may work again elsewhere"
        );
    }

    /// A repeated address is not appended twice.
    #[test]
    fn observing_the_same_address_twice_does_not_duplicate_it() {
        let mut cache = PeerCache::default();
        cache.observe("alice", vec!["/addr".to_string()]);
        cache.observe("alice", vec!["/addr".to_string()]);
        assert_eq!(cache.peers[0].addrs, vec!["/addr"]);
    }

    /// A peer not seen in this round is kept: absence from one snapshot is not
    /// evidence the peer is gone.
    #[test]
    fn peers_absent_from_an_update_are_retained() {
        let mut cache = PeerCache::default();
        cache.observe("alice", vec!["/a".to_string()]);
        cache.observe("bob", vec!["/b".to_string()]);

        cache.observe_all(&[("alice".to_string(), vec!["/a".to_string()])]);

        assert!(
            cache.contains("bob"),
            "bob must survive not being connected"
        );
        assert_eq!(cache.len(), 2);
    }

    // ── Dial set ────────────────────────────────────────────────────────────

    /// Cached addresses come back newest-peer-first and skip ones already in
    /// the configured set, so the startup dial list has no duplicates.
    #[test]
    fn dial_addrs_are_newest_first_and_exclude_what_we_have() {
        let cache = PeerCache {
            peers: vec![
                CachedPeer {
                    peer_id: "old".to_string(),
                    addrs: vec!["/old".to_string()],
                    last_seen: 1,
                },
                CachedPeer {
                    peer_id: "new".to_string(),
                    addrs: vec!["/new".to_string(), "/configured".to_string()],
                    last_seen: 99,
                },
            ],
        };

        let have: HashSet<String> = ["/configured".to_string()].into_iter().collect();
        assert_eq!(cache.dial_addrs(&have), vec!["/new", "/old"]);
    }

    /// An empty cache contributes no dial addresses.
    #[test]
    fn an_empty_cache_yields_no_dial_addresses() {
        assert!(PeerCache::default().dial_addrs(&HashSet::new()).is_empty());
    }
}
