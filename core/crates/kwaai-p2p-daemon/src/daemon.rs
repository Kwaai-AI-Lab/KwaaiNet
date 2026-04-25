//! Daemon lifecycle management
//!
//! This module handles spawning, monitoring, and shutting down the
//! go-libp2p-daemon process.

use crate::client::P2PClient;
use crate::error::{Error, Result};
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::process::{Child, Command};
use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};

/// Configuration builder for the p2p daemon
#[derive(Default)]
pub struct DaemonBuilder {
    binary_path: Option<PathBuf>,
    listen_addr: Option<String>,
    bootstrap_peers: Vec<String>,
    bootstrap: bool,
    dht: bool,
    relay: bool,
    auto_relay: bool,
    auto_nat: bool,
    nat_portmap: bool,
    host_addrs: Vec<String>,
    announce_addrs: Vec<String>,
    trusted_relays: Vec<String>,
    force_reachability_private: bool,
    metrics: bool,
    metrics_addr: Option<String>,
    /// Path to a protobuf-encoded Ed25519 private key file (`-id` flag).
    /// When set, p2pd uses this key so the PeerId is stable across restarts.
    identity_key_path: Option<PathBuf>,
}

impl DaemonBuilder {
    /// Create a new daemon builder with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the path to the p2pd binary
    ///
    /// If not set, uses the binary built by build.rs
    pub fn with_binary_path<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.binary_path = Some(path.into());
        self
    }

    /// Set the IPC listen address
    ///
    /// - **Windows**: Named pipe path (e.g., `//./pipe/kwaai-p2pd`)
    /// - **Unix**: Unix socket path (e.g., `/tmp/kwaai-p2pd.sock`)
    pub fn with_listen_addr<S: Into<String>>(mut self, addr: S) -> Self {
        self.listen_addr = Some(addr.into());
        self
    }

    /// Enable DHT support
    pub fn dht(mut self, enable: bool) -> Self {
        self.dht = enable;
        self
    }

    /// Enable relay support (this node acts as a relay server)
    pub fn relay(mut self, enable: bool) -> Self {
        self.relay = enable;
        self
    }

    /// Enable auto-relay (this node uses relay servers when behind NAT)
    pub fn auto_relay(mut self, enable: bool) -> Self {
        self.auto_relay = enable;
        self
    }

    /// Enable AutoNAT (detect whether this node is reachable from the internet)
    pub fn auto_nat(mut self, enable: bool) -> Self {
        self.auto_nat = enable;
        self
    }

    /// Set the host multiaddrs p2pd listens on for P2P traffic
    /// e.g. ["/ip4/0.0.0.0/tcp/8080"]
    pub fn host_addrs<I, S>(mut self, addrs: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.host_addrs.extend(addrs.into_iter().map(|s| s.into()));
        self
    }

    /// Set the multiaddrs this node announces to the DHT network
    /// e.g. ["/ip4/203.0.113.1/tcp/8080"] — the public/reachable address
    pub fn announce_addrs<I, S>(mut self, addrs: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.announce_addrs
            .extend(addrs.into_iter().map(|s| s.into()));
        self
    }

    /// Set trusted relay peers for AutoRelay (`-trustedRelays` flag)
    ///
    /// These peers are tried as circuit relay servers. Typically the bootstrap
    /// peers, which run with `-relay=1 -relayService=1`.
    pub fn trusted_relays<I, S>(mut self, relays: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.trusted_relays
            .extend(relays.into_iter().map(|s| s.into()));
        self
    }

    /// Force the node to be treated as behind NAT (`-forceReachabilityPrivate`)
    ///
    /// Skips AutoNAT probing and immediately activates AutoRelay to obtain
    /// relay circuit addresses. Without this, a NATed node must wait for 3+
    /// peers to probe it — which may never happen in small networks.
    pub fn force_reachability_private(mut self, enable: bool) -> Self {
        self.force_reachability_private = enable;
        self
    }

    /// Enable NAT port mapping
    pub fn nat_portmap(mut self, enable: bool) -> Self {
        self.nat_portmap = enable;
        self
    }

    /// Enable metrics endpoint
    pub fn metrics(mut self, enable: bool) -> Self {
        self.metrics = enable;
        self
    }

    /// Set metrics listen address (e.g., "127.0.0.1:8888")
    pub fn metrics_addr<S: Into<String>>(mut self, addr: S) -> Self {
        self.metrics_addr = Some(addr.into());
        self
    }

    /// Add a bootstrap peer
    pub fn bootstrap_peer<S: Into<String>>(mut self, peer: S) -> Self {
        self.bootstrap_peers.push(peer.into());
        self
    }

    /// Add multiple bootstrap peers
    pub fn bootstrap_peers<I, S>(mut self, peers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.bootstrap_peers
            .extend(peers.into_iter().map(|s| s.into()));
        self
    }

    /// Enable Kademlia DHT bootstrap (`-b` flag)
    ///
    /// When enabled, the daemon connects to the bootstrap peers and runs a
    /// Kademlia self-lookup to populate routing tables. Without this, the
    /// daemon's DHT routing table stays empty and the node is invisible to
    /// peers performing DHT lookups (e.g. the health service).
    pub fn bootstrap(mut self, enable: bool) -> Self {
        self.bootstrap = enable;
        self
    }

    /// Set the path to a protobuf-encoded Ed25519 private key file (`-id` flag)
    ///
    /// When provided, p2pd uses this key so the node's `PeerId` is stable
    /// across restarts. This is a prerequisite for meaningful Verifiable
    /// Credentials — credentials are bound to a DID that must not change.
    ///
    /// The file must contain the raw bytes of a libp2p protobuf-encoded private
    /// key (`Keypair::to_protobuf_encoding()`), compatible with Go's
    /// `crypto.UnmarshalPrivateKey`.
    pub fn with_identity_key<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.identity_key_path = Some(path.into());
        self
    }

    /// Spawn the daemon process
    pub async fn spawn(self) -> Result<P2PDaemon> {
        let binary_path = self
            .binary_path
            .unwrap_or_else(|| PathBuf::from(crate::DAEMON_BINARY_PATH));

        // Use platform-specific default if no listen address provided
        // On Windows, use TCP since Go libp2p doesn't support Windows named pipes in multiaddr format
        // On Unix, use Unix domain sockets
        let listen_addr = self.listen_addr.unwrap_or_else(|| {
            #[cfg(windows)]
            {
                "/ip4/127.0.0.1/tcp/5005".to_string() // Use TCP on Windows
            }
            #[cfg(unix)]
            {
                "/unix//tmp/kwaai-p2pd.sock".to_string() // Use Unix socket on Linux/macOS
            }
        });

        info!("Starting p2pd daemon from: {}", binary_path.display());
        info!("Listen address: {}", listen_addr);

        // Clean up stale Unix socket if it exists
        #[cfg(unix)]
        if let Some(socket_path) = listen_addr.strip_prefix("/unix/") {
            if std::path::Path::new(socket_path).exists() {
                debug!("Removing stale Unix socket: {}", socket_path);
                let _ = std::fs::remove_file(socket_path);
            }
        }

        // Build command
        let mut cmd = Command::new(&binary_path);

        // Set listen address
        cmd.arg("-listen").arg(&listen_addr);

        // DHT mode
        if self.dht {
            cmd.arg("-dht");
        }

        // Relay (this node serves as a relay)
        if self.relay {
            cmd.arg("-relay");
        }

        // AutoRelay (this node uses relay servers when behind NAT)
        if self.auto_relay {
            cmd.arg("-autoRelay");
            // Use static trusted relays instead of DHT-based relay discovery
            // when trusted relays are configured. DHT discovery requires a
            // populated routing table which NATed nodes may not have.
            if !self.trusted_relays.is_empty() {
                cmd.arg("-relayDiscovery=false");
            }
        }

        // AutoNAT
        if self.auto_nat {
            cmd.arg("-autonat");
        }

        // Force reachability private (skip AutoNAT, activate relay immediately)
        if self.force_reachability_private {
            cmd.arg("-forceReachabilityPrivate");
        }

        // NAT port mapping
        if self.nat_portmap {
            cmd.arg("-natPortMap");
        }

        // Host addrs (P2P listen addresses)
        if !self.host_addrs.is_empty() {
            cmd.arg("-hostAddrs").arg(self.host_addrs.join(","));
        }

        // Announce addrs (public addresses to advertise in the DHT)
        if !self.announce_addrs.is_empty() {
            cmd.arg("-announceAddrs").arg(self.announce_addrs.join(","));
        }

        // Trusted relay peers for AutoRelay
        if !self.trusted_relays.is_empty() {
            cmd.arg("-trustedRelays")
                .arg(self.trusted_relays.join(","));
        }

        // Metrics
        if self.metrics {
            cmd.arg("-metrics");
            if let Some(addr) = self.metrics_addr {
                cmd.arg("-metricsAddr").arg(addr);
            }
        }

        // Bootstrap peers (comma-separated; Go flag.String accepts one value)
        if !self.bootstrap_peers.is_empty() {
            cmd.arg("-bootstrapPeers")
                .arg(self.bootstrap_peers.join(","));
        }

        // Kademlia bootstrap walk (connect to bootstrap peers + self-lookup)
        if self.bootstrap {
            cmd.arg("-b");
        }

        // Persistent identity key — makes PeerId stable across restarts
        if let Some(ref key_path) = self.identity_key_path {
            info!("Using persistent identity key: {}", key_path.display());
            cmd.arg("-id").arg(key_path);
        }

        // Forward GOLOG_LOG_LEVEL to the Go daemon for diagnostics
        if let Ok(level) = std::env::var("GOLOG_LOG_LEVEL") {
            cmd.env("GOLOG_LOG_LEVEL", level);
        }

        // Redirect stderr for logging
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        // Suppress console window on Windows
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
        }

        debug!("Spawning daemon: {:?}", cmd);

        let mut child = cmd.spawn().map_err(|e| {
            Error::Process(format!(
                "Failed to spawn daemon at {}: {}",
                binary_path.display(),
                e
            ))
        })?;

        info!("Daemon process spawned (PID: {:?})", child.id());

        // Capture stderr in a background task so we can surface crash reasons.
        // When GOLOG_LOG_LEVEL is set, also log lines as they arrive so
        // go-libp2p diagnostics are visible in `docker compose logs`.
        let stderr_buf: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
        if let Some(stderr) = child.stderr.take() {
            let buf = stderr_buf.clone();
            let forward_logs = std::env::var("GOLOG_LOG_LEVEL").is_ok();
            tokio::spawn(async move {
                use tokio::io::AsyncBufReadExt;
                let mut lines = tokio::io::BufReader::new(stderr).lines();
                let mut output = String::new();
                while let Ok(Some(line)) = lines.next_line().await {
                    if forward_logs {
                        debug!(target: "p2pd", "{}", line);
                    }
                    if output.len() < 8192 {
                        output.push_str(&line);
                        output.push('\n');
                    }
                }
                *buf.lock().await = output;
            });
        }

        Ok(P2PDaemon {
            process: Some(child),
            listen_addr,
            stderr_buf,
        })
    }
}

/// Handle to a running p2p daemon process
pub struct P2PDaemon {
    process: Option<Child>,
    listen_addr: String,
    /// Captured stderr from the daemon process (populated by background reader)
    stderr_buf: Arc<Mutex<String>>,
}

impl P2PDaemon {
    /// Create a new daemon builder
    pub fn builder() -> DaemonBuilder {
        DaemonBuilder::new()
    }

    /// Get the IPC listen address
    pub fn listen_addr(&self) -> &str {
        &self.listen_addr
    }

    /// Create a client connected to this daemon
    ///
    /// Waits for the daemon to create its IPC socket (up to 5 s), checking
    /// that the process is still alive on each poll.  If the daemon exits
    /// before the socket appears, its captured stderr is included in the
    /// error so the operator can see *why* it crashed.
    pub async fn client(&mut self) -> Result<P2PClient> {
        // Determine the filesystem path we're waiting for
        #[cfg(unix)]
        let socket_path = self.listen_addr.strip_prefix("/unix/").map(PathBuf::from);
        #[cfg(not(unix))]
        let socket_path: Option<PathBuf> = None; // TCP — no file to poll

        let poll_interval = tokio::time::Duration::from_millis(100);
        let deadline = tokio::time::Instant::now() + tokio::time::Duration::from_secs(5);

        while tokio::time::Instant::now() < deadline {
            // Check if the daemon is still alive
            if let Some(child) = &mut self.process {
                if let Ok(Some(status)) = child.try_wait() {
                    // Daemon exited — give the stderr reader a moment to finish
                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                    let stderr = self.stderr_buf.lock().await;
                    let detail = if stderr.is_empty() {
                        format!("p2pd exited with {status} (no stderr output)")
                    } else {
                        format!("p2pd exited with {status}:\n{}", stderr.trim())
                    };
                    error!("{}", detail);
                    return Err(Error::Process(detail));
                }
            }

            // Check if the socket file exists yet (Unix only)
            if let Some(ref path) = socket_path {
                if path.exists() {
                    debug!("Socket ready: {}", path.display());
                    return P2PClient::connect(&self.listen_addr).await;
                }
            } else {
                // TCP mode — just try connecting directly
                break;
            }

            tokio::time::sleep(poll_interval).await;
        }

        // Final attempt (covers TCP mode and timeout after polling)
        P2PClient::connect(&self.listen_addr).await
    }

    /// Check if the daemon process is still running
    pub fn is_running(&mut self) -> bool {
        if let Some(child) = &mut self.process {
            child.try_wait().ok().flatten().is_none()
        } else {
            false
        }
    }

    /// Wait for the daemon to exit
    pub async fn wait(&mut self) -> Result<()> {
        if let Some(mut child) = self.process.take() {
            let status = child.wait().await?;
            if !status.success() {
                warn!("Daemon exited with status: {:?}", status.code());
                return Err(Error::Process(format!(
                    "Daemon exited with code: {:?}",
                    status.code()
                )));
            }
        }
        Ok(())
    }

    /// Shutdown the daemon gracefully
    pub async fn shutdown(&mut self) -> Result<()> {
        if let Some(mut child) = self.process.take() {
            info!("Shutting down daemon...");

            #[cfg(unix)]
            {
                // Send SIGTERM
                if let Some(pid) = child.id() {
                    unsafe {
                        libc::kill(pid as i32, libc::SIGTERM);
                    }
                }
            }

            #[cfg(windows)]
            {
                // Windows: kill the process
                child.kill().await?;
            }

            // Wait for exit
            tokio::time::timeout(tokio::time::Duration::from_secs(5), child.wait())
                .await
                .map_err(|_| Error::Timeout)?
                .map_err(|e| Error::Process(format!("Failed to wait for daemon exit: {}", e)))?;

            info!("Daemon shutdown complete");
        }
        Ok(())
    }
}

impl Drop for P2PDaemon {
    fn drop(&mut self) {
        if let Some(mut child) = self.process.take() {
            // Attempt to kill the process if not already exited
            if child.try_wait().ok().flatten().is_none() {
                warn!("Daemon process still running, killing...");
                let _ = child.start_kill();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_daemon_builder() {
        let builder = DaemonBuilder::new()
            .dht(true)
            .relay(true)
            .bootstrap_peer("/ip4/127.0.0.1/tcp/8000/p2p/QmTest");

        assert!(builder.dht);
        assert!(builder.relay);
        assert_eq!(builder.bootstrap_peers.len(), 1);
    }
}
