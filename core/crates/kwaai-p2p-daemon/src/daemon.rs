//! Daemon lifecycle management
//!
//! This module handles spawning, monitoring, and shutting down the
//! go-libp2p-daemon process.

use crate::client::P2PClient;
use crate::error::{Error, Result};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::{Child, Command};
use tracing::{debug, info, warn};

/// Configuration builder for the p2p daemon
pub struct DaemonBuilder {
    binary_path: Option<PathBuf>,
    listen_addr: Option<String>,
    bootstrap_peers: Vec<String>,
    dht: bool,
    relay: bool,
    auto_relay: bool,
    auto_nat: bool,
    nat_portmap: bool,
    host_addrs: Vec<String>,
    announce_addrs: Vec<String>,
    metrics: bool,
    metrics_addr: Option<String>,
}

impl Default for DaemonBuilder {
    fn default() -> Self {
        Self {
            binary_path: None,
            listen_addr: None,
            bootstrap_peers: Vec::new(),
            dht: false,
            relay: false,
            auto_relay: false,
            auto_nat: false,
            nat_portmap: false,
            host_addrs: Vec::new(),
            announce_addrs: Vec::new(),
            metrics: false,
            metrics_addr: None,
        }
    }
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
        self.announce_addrs.extend(addrs.into_iter().map(|s| s.into()));
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
        self.bootstrap_peers.extend(peers.into_iter().map(|s| s.into()));
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
                "/ip4/127.0.0.1/tcp/5005".to_string()  // Use TCP on Windows
            }
            #[cfg(unix)]
            {
                "/unix//tmp/kwaai-p2pd.sock".to_string()  // Use Unix socket on Linux/macOS
            }
        });

        info!("Starting p2pd daemon from: {}", binary_path.display());
        info!("Listen address: {}", listen_addr);

        // Clean up stale Unix socket if it exists
        #[cfg(unix)]
        if listen_addr.starts_with("/unix/") {
            let socket_path = &listen_addr[6..]; // Skip "/unix/"
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
        }

        // AutoNAT
        if self.auto_nat {
            cmd.arg("-autonat");
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

        // Metrics
        if self.metrics {
            cmd.arg("-metrics");
            if let Some(addr) = self.metrics_addr {
                cmd.arg("-metricsAddr").arg(addr);
            }
        }

        // Bootstrap peers
        for peer in &self.bootstrap_peers {
            cmd.arg("-bootstrapPeers").arg(peer);
        }

        // Redirect stderr for logging
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        debug!("Spawning daemon: {:?}", cmd);

        let child = cmd.spawn().map_err(|e| {
            Error::Process(format!(
                "Failed to spawn daemon at {}: {}",
                binary_path.display(),
                e
            ))
        })?;

        info!("Daemon process spawned (PID: {:?})", child.id());

        Ok(P2PDaemon {
            process: Some(child),
            listen_addr,
        })
    }
}

/// Handle to a running p2p daemon process
pub struct P2PDaemon {
    process: Option<Child>,
    listen_addr: String,
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
    pub async fn client(&self) -> Result<P2PClient> {
        // Give daemon a moment to start listening
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

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
            tokio::time::timeout(
                tokio::time::Duration::from_secs(5),
                child.wait()
            )
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
