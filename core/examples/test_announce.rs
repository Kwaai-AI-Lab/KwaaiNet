//! Test the integrated DHT announcement system

use kwaai_p2p::{KwaaiNetwork, NetworkBehaviour, NetworkConfig, ServerInfo};
use tokio::time::{sleep, Duration};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    println!("Testing KwaaiNetwork DHT announcement system");
    println!("==========================================\n");

    // Create network with Petals bootstrap
    let config = NetworkConfig::with_kwaai_bootstrap();
    let mut network = KwaaiNetwork::new(config).await?;

    println!("Local Peer ID: {}\n", network.local_peer_id());

    // Start listening
    network.start().await?;
    println!("Network started\n");

    // Bootstrap to Petals network
    let bootstrap_peers = vec![
        "/ip4/18.219.43.67/tcp/8000/p2p/QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc".parse()?,
        "/ip4/52.23.252.2/tcp/8000/p2p/Qmd3A8N5aQBATe2SYvNikaeCS9CAKN4E86jdCPacZ6RZJY".parse()?,
    ];

    network.bootstrap(bootstrap_peers).await?;
    println!("Bootstrap initiated\n");

    // Wait for bootstrap to complete
    println!("Waiting 5s for bootstrap...");
    sleep(Duration::from_secs(5)).await;

    // Create server info
    let server_info = ServerInfo::new("rust-integrated-test")
        .with_span(0, 8)
        .with_cache_tokens(10000)
        .with_throughput(10.0)
        .with_dtype("float16");

    println!("\nAnnouncing blocks to DHT...");
    network.announce_blocks("Llama-3.1-8B-Instruct", &server_info).await?;

    println!("Processing DHT commands...");
    // Process DHT commands for 30 seconds
    for i in 0..60 {
        if network.process_dht_command().await? {
            println!("  [{}] Processed DHT command", i);
        }
        sleep(Duration::from_millis(500)).await;
    }

    println!("\n✅ Announcement complete!");
    println!("Check map.kwaai.ai in 2-3 minutes for node: rust-integrated-test");

    // Keep running
    println!("Keeping node alive... (Ctrl+C to stop)");
    loop {
        network.process_dht_command().await?;
        sleep(Duration::from_secs(1)).await;
    }
}
