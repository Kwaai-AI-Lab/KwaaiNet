//! Test daemon stream handler registration
//!
//! This example tests registering Hivemind protocol handlers with the p2p-daemon.
//!
//! Run with: cargo run --example daemon_test

use kwaai_p2p::NetworkConfig;
use kwaai_p2p_daemon::P2PDaemon;
use std::error::Error;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    println!("Daemon Stream Handler Test");
    println!("===========================\n");

    // Use Petals bootstrap servers
    let config = NetworkConfig::with_petals_bootstrap();
    println!("Bootstrap servers:");
    for server in &config.bootstrap_peers {
        println!("  {}", server);
    }
    println!();

    // Start p2p-daemon
    info!("Starting p2p daemon...");
    let mut daemon = P2PDaemon::builder()
        .dht(true)
        .relay(true)
        .nat_portmap(true)
        .bootstrap_peers(config.bootstrap_peers.clone())
        .spawn()
        .await?;

    println!("[DAEMON] Spawned at: {}", daemon.listen_addr());

    let mut client = daemon.client().await?;
    let peer_id = client.identify().await?;
    println!("[PEER ID] {}", peer_id);

    // Create local listener for incoming streams
    let handler_listener = TcpListener::bind("127.0.0.1:9000").await?;
    let handler_addr = handler_listener.local_addr()?;
    println!("[HANDLER] Listening on: {}", handler_addr);

    // Register stream handlers with daemon
    let handler_multiaddr = format!("/ip4/127.0.0.1/tcp/{}", handler_addr.port());
    let protocols = vec![
        "DHTProtocol.rpc_ping".to_string(),
        "DHTProtocol.rpc_store".to_string(),
        "DHTProtocol.rpc_find".to_string(),
    ];

    info!("Registering stream handlers...");
    client.register_stream_handler(&handler_multiaddr, protocols.clone()).await?;
    println!("[HANDLER] Registered protocols: {:?}", protocols);

    println!("\n[STATUS] Daemon is running with registered handlers!");
    println!("         When a peer calls one of our protocols, daemon will forward");
    println!("         the stream to 127.0.0.1:9000\n");
    println!("         Press Ctrl+C to shutdown.\n");

    // Accept incoming streams from daemon
    tokio::select! {
        _ = daemon.wait() => {
            info!("Daemon exited");
        }
        _ = async {
            loop {
                let (stream, addr) = handler_listener.accept().await?;
                info!("Received stream from daemon: {}", addr);
                // TODO: Parse StreamInfo and handle the request
                drop(stream);
            }
            #[allow(unreachable_code)]
            Ok::<(), Box<dyn Error>>(())
        } => {
            info!("Handler exited");
        }
    }

    Ok(())
}
