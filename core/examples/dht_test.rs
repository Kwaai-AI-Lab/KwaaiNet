//! Test example for DHT operations
//!
//! This example demonstrates:
//! 1. Spawning the daemon with DHT enabled
//! 2. Putting a value into the DHT
//! 3. Getting a value from the DHT
//!
//! Run with: cargo run --example dht_test

use kwaai_p2p_daemon::P2PDaemon;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("=== DHT Operations Test ===");

    // Spawn daemon with DHT enabled
    info!("\n[1/4] Spawning daemon with DHT...");
    let mut daemon = P2PDaemon::builder()
        .dht(true)  // Enable DHT
        .spawn()
        .await?;

    info!("Daemon spawned at: {}", daemon.listen_addr());

    // Connect client
    info!("\n[2/4] Connecting client...");
    let mut client = daemon.client().await?;

    let peer_id = client.identify().await?;
    info!("Our Peer ID: {}", peer_id);

    // Put a value into the DHT
    info!("\n[3/4] Testing DHT PUT_VALUE...");
    let key = b"test-key-123".to_vec();
    let value = b"Hello from KwaaiNet DHT!".to_vec();

    match client.dht_put_value(key.clone(), value.clone(), Some(30)).await {
        Ok(_) => info!("✓ Successfully stored value in DHT"),
        Err(e) => info!("⚠ PUT_VALUE failed (expected if not connected to network): {}", e),
    }

    // Try to get the value back
    info!("\n[4/4] Testing DHT GET_VALUE...");
    match client.dht_get_value(key.clone(), Some(30)).await {
        Ok(dht_value) => {
            info!("✓ Successfully retrieved value from DHT");
            info!("  Value: {:?}", String::from_utf8_lossy(&dht_value.value));
            info!("  Value type: {:?}", dht_value.value_type);
        }
        Err(e) => {
            info!("⚠ GET_VALUE failed (expected if not connected to network): {}", e);
        }
    }

    // Shutdown
    info!("\nShutting down daemon...");
    daemon.shutdown().await?;

    info!("\n=== Test Complete ===");
    info!("Note: DHT operations require connection to the network.");
    info!("To test with real Petals network, add bootstrap peers:");
    info!("  .bootstrap_peer(\"/ip4/46.4.84.154/tcp/31337/p2p/...\")");

    Ok(())
}
