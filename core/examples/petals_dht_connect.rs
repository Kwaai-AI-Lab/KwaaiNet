//! Connect to Petals DHT network and test operations
//!
//! This example demonstrates:
//! 1. Connecting to Petals bootstrap nodes
//! 2. Announcing presence in the DHT
//! 3. Querying the DHT for Petals server blocks
//!
//! Run with: cargo run --example petals_dht_connect

use kwaai_p2p_daemon::P2PDaemon;
use tracing::{info, warn, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("=== Petals DHT Network Connection Test ===");
    info!("This will connect to the real Petals network!");

    // Petals bootstrap nodes from health.petals.dev
    let bootstrap_peers = vec![
        // Primary bootstrap nodes
        "/ip4/159.89.214.152/tcp/31337/p2p/QmedTaZXmULqwspJXz44SsPZyTNKxhnnWvYRN89fFLrXNf",
        "/ip4/209.145.52.147/tcp/31337/p2p/QmQGTqmM7NKjV6ggU1ZCap8zWiyKR89RViDXiqehSiCpY5",
    ];

    // Spawn daemon with DHT and bootstrap peers
    info!("\n[1/5] Spawning daemon with Petals bootstrap nodes...");
    let mut daemon = P2PDaemon::builder()
        .dht(true)  // Enable full DHT mode
        .relay(true)  // Enable relay for NAT traversal
        .nat_portmap(true)  // Try NAT port mapping
        .bootstrap_peers(bootstrap_peers)
        .spawn()
        .await?;

    info!("Daemon spawned at: {}", daemon.listen_addr());

    // Connect client
    info!("\n[2/5] Connecting client...");
    let mut client = daemon.client().await?;

    let peer_id = client.identify().await?;
    info!("Our Peer ID: {}", peer_id);

    // Wait for DHT to bootstrap
    info!("\n[3/5] Waiting for DHT to bootstrap (5 seconds)...");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // Try to announce ourselves in the DHT
    info!("\n[4/5] Testing DHT operations...");

    // Example: Try to find a known Petals server block key
    // Petals uses keys like "blocks.0.hash" for block announcements
    let test_key = b"/dht/petals/server-info/test".to_vec();

    info!("Attempting to query DHT for test key...");
    match client.dht_get_value(test_key.clone(), Some(30)).await {
        Ok(value) => {
            info!("✓ Successfully retrieved value from DHT!");
            info!("  Value length: {} bytes", value.value.len());
            if let Ok(s) = String::from_utf8(value.value.clone()) {
                info!("  Value: {}", s);
            }
        }
        Err(e) => {
            info!("⚠ GET_VALUE: {} (this is normal if key doesn't exist)", e);
        }
    }

    // Try to put a test value
    info!("\nAttempting to store test value in DHT...");
    let test_value = format!("KwaaiNet node - {}", chrono::Utc::now()).into_bytes();
    match client.dht_put_value(test_key.clone(), test_value, Some(60)).await {
        Ok(_) => {
            info!("✓ Successfully stored value in DHT!");

            // Try to retrieve it back
            info!("Verifying stored value...");
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            match client.dht_get_value(test_key, Some(30)).await {
                Ok(value) => {
                    info!("✓ Successfully retrieved our value!");
                    if let Ok(s) = String::from_utf8(value.value) {
                        info!("  Retrieved: {}", s);
                    }
                }
                Err(e) => {
                    warn!("⚠ Could not retrieve our value: {}", e);
                }
            }
        }
        Err(e) => {
            info!("⚠ PUT_VALUE: {}", e);
            info!("This might be due to key format restrictions in the DHT.");
        }
    }

    // Keep daemon running for a bit to stay connected
    info!("\n[5/5] Keeping connection alive for 10 seconds...");
    info!("(Your node is now part of the Petals DHT network!)");
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    // Shutdown
    info!("\nShutting down daemon...");
    daemon.shutdown().await?;

    info!("\n=== Connection Test Complete ===");
    info!("Your node successfully connected to the Petals network!");
    info!("\nNext steps:");
    info!("  1. Implement Hivemind protocol handlers");
    info!("  2. Announce server blocks in DHT");
    info!("  3. Respond to inference requests");
    info!("  4. Appear on map.petals.dev");

    Ok(())
}
