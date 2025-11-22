//! Day 10: P2P Tensor Exchange Example
//!
//! Demonstrates sending tensors between P2P nodes:
//! - Serialize tensors for network transmission
//! - Compress using 8-bit quantization
//! - Exchange via libp2p request-response
//! - Reconstruct tensors on receiving end
//!
//! ## Usage
//!
//! Terminal 1 (receiver):
//! ```bash
//! cargo run --example tensor_exchange -- --listen 4001
//! ```
//!
//! Terminal 2 (sender):
//! ```bash
//! cargo run --example tensor_exchange -- --connect /ip4/127.0.0.1/tcp/4001/p2p/<PEER_ID> --send
//! ```
//!
//! Run with: cargo run --example tensor_exchange

use candle_core::{Device, Tensor};
use futures::StreamExt;
use kwaai_compression::{BlockwiseQuantizer, CompressedData, Compressor, QuantizedTensor};
use libp2p::{
    identify, identity,
    kad::{self, store::MemoryStore, Mode},
    noise,
    request_response::{self, Codec, ProtocolSupport},
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::Duration;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

/// Tensor message for exchange
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TensorMessage {
    /// Message type
    msg_type: TensorMessageType,
    /// Tensor name/identifier
    name: String,
    /// Compressed tensor data (if applicable)
    data: Option<QuantizedTensor>,
    /// Original shape
    shape: Vec<usize>,
    /// Metadata
    metadata: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum TensorMessageType {
    /// Sending tensor data
    TensorData,
    /// Acknowledging receipt
    Ack,
    /// Request tensor by name
    Request,
}

/// Codec for tensor messages
#[derive(Debug, Clone)]
struct TensorCodec;

#[derive(Debug, Clone)]
struct TensorProtocol;

impl AsRef<str> for TensorProtocol {
    fn as_ref(&self) -> &str {
        "/kwaai/tensor/1.0.0"
    }
}

impl Codec for TensorCodec {
    type Protocol = TensorProtocol;
    type Request = TensorMessage;
    type Response = TensorMessage;

    fn read_request<'life0, 'life1, 'life2, 'async_trait, T>(
        &'life0 mut self,
        _protocol: &'life1 Self::Protocol,
        io: &'life2 mut T,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = std::io::Result<Self::Request>> + Send + 'async_trait>,
    >
    where
        T: futures::AsyncRead + Unpin + Send + 'async_trait,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            use futures::AsyncReadExt;
            let mut len_bytes = [0u8; 4];
            io.read_exact(&mut len_bytes).await?;
            let len = u32::from_be_bytes(len_bytes) as usize;

            let mut buf = vec![0u8; len];
            io.read_exact(&mut buf).await?;

            bincode::deserialize(&buf)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
        })
    }

    fn read_response<'life0, 'life1, 'life2, 'async_trait, T>(
        &'life0 mut self,
        protocol: &'life1 Self::Protocol,
        io: &'life2 mut T,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = std::io::Result<Self::Response>> + Send + 'async_trait>,
    >
    where
        T: futures::AsyncRead + Unpin + Send + 'async_trait,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        self.read_request(protocol, io)
    }

    fn write_request<'life0, 'life1, 'life2, 'async_trait, T>(
        &'life0 mut self,
        _protocol: &'life1 Self::Protocol,
        io: &'life2 mut T,
        req: Self::Request,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = std::io::Result<()>> + Send + 'async_trait>,
    >
    where
        T: futures::AsyncWrite + Unpin + Send + 'async_trait,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            use futures::AsyncWriteExt;
            let buf = bincode::serialize(&req)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

            io.write_all(&(buf.len() as u32).to_be_bytes()).await?;
            io.write_all(&buf).await?;
            io.flush().await?;
            Ok(())
        })
    }

    fn write_response<'life0, 'life1, 'life2, 'async_trait, T>(
        &'life0 mut self,
        protocol: &'life1 Self::Protocol,
        io: &'life2 mut T,
        res: Self::Response,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = std::io::Result<()>> + Send + 'async_trait>,
    >
    where
        T: futures::AsyncWrite + Unpin + Send + 'async_trait,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        self.write_request(protocol, io, res)
    }
}

#[derive(NetworkBehaviour)]
struct TensorExchangeBehaviour {
    kademlia: kad::Behaviour<MemoryStore>,
    identify: identify::Behaviour,
    tensor_exchange: request_response::Behaviour<TensorCodec>,
}

#[derive(Debug)]
enum Command {
    Listen { port: u16 },
    Connect { addr: Multiaddr, send: bool },
}

fn parse_args() -> Command {
    let args: Vec<String> = std::env::args().collect();
    let mut i = 1;
    let mut port = 0u16;
    let mut connect_addr: Option<Multiaddr> = None;
    let mut send = false;

    while i < args.len() {
        match args[i].as_str() {
            "--listen" => {
                if i + 1 < args.len() {
                    port = args[i + 1].parse().unwrap_or(0);
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--connect" => {
                if i + 1 < args.len() {
                    connect_addr = args[i + 1].parse().ok();
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--send" => {
                send = true;
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }

    if let Some(addr) = connect_addr {
        Command::Connect { addr, send }
    } else {
        Command::Listen { port }
    }
}

fn extract_peer_id(addr: &Multiaddr) -> Option<PeerId> {
    addr.iter().find_map(|p| {
        if let libp2p::multiaddr::Protocol::P2p(peer_id) = p {
            Some(peer_id)
        } else {
            None
        }
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let command = parse_args();
    let device = Device::Cpu;
    let compressor = BlockwiseQuantizer::new(64);

    println!("KwaaiNet Tensor Exchange Demo\n");
    println!("==============================\n");

    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    info!("Local Peer ID: {}", local_peer_id);

    // Setup behaviours
    let kademlia = {
        let store = MemoryStore::new(local_peer_id);
        let mut config = kad::Config::default();
        config.set_replication_factor(std::num::NonZeroUsize::new(3).unwrap());
        let mut behaviour = kad::Behaviour::with_config(local_peer_id, store, config);
        behaviour.set_mode(Some(Mode::Server));
        behaviour
    };

    let identify = identify::Behaviour::new(identify::Config::new(
        "/kwaai/1.0.0".to_string(),
        local_key.public(),
    ));

    let tensor_exchange = request_response::Behaviour::new(
        [(TensorProtocol, ProtocolSupport::Full)],
        request_response::Config::default(),
    );

    let behaviour = TensorExchangeBehaviour {
        kademlia,
        identify,
        tensor_exchange,
    };

    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(local_key)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|_| Ok(behaviour))?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    match command {
        Command::Listen { port } => {
            let listen_addr: Multiaddr = format!("/ip4/0.0.0.0/tcp/{}", port).parse()?;
            swarm.listen_on(listen_addr)?;

            println!("Mode: RECEIVER");
            println!("Waiting for tensor data...\n");

            loop {
                match swarm.select_next_some().await {
                    SwarmEvent::NewListenAddr { address, .. } => {
                        let full_addr = format!("{}/p2p/{}", address, local_peer_id);
                        info!("Listening on: {}", full_addr);
                        println!("\nConnect with: --connect {}\n", full_addr);
                    }
                    SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                        info!("Connected: {}", peer_id);
                    }
                    SwarmEvent::Behaviour(TensorExchangeBehaviourEvent::TensorExchange(
                        request_response::Event::Message {
                            peer,
                            message:
                                request_response::Message::Request {
                                    request, channel, ..
                                },
                        },
                    )) => {
                        info!("Received tensor from: {}", peer);

                        match request.msg_type {
                            TensorMessageType::TensorData => {
                                println!("\n  RECEIVED TENSOR:");
                                println!("  Name: {}", request.name);
                                println!("  Shape: {:?}", request.shape);
                                println!("  Metadata: {}", request.metadata);

                                if let Some(compressed) = &request.data {
                                    println!(
                                        "  Compressed size: {} bytes",
                                        compressed.size_bytes()
                                    );
                                    println!(
                                        "  Compression ratio: {:.2}x",
                                        compressed.compression_ratio()
                                    );

                                    // Decompress
                                    match compressor.decompress(compressed) {
                                        Ok(tensor) => {
                                            let data: Vec<f32> =
                                                tensor.flatten_all()?.to_vec1()?;
                                            let mean =
                                                data.iter().sum::<f32>() / data.len() as f32;
                                            let min = data
                                                .iter()
                                                .cloned()
                                                .fold(f32::INFINITY, f32::min);
                                            let max =
                                                data.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

                                            println!("\n  DECOMPRESSED:");
                                            println!("  Elements: {}", data.len());
                                            println!("  Mean: {:.4}", mean);
                                            println!("  Min:  {:.4}", min);
                                            println!("  Max:  {:.4}", max);
                                            println!("  Sample values: {:?}", &data[..5.min(data.len())]);
                                        }
                                        Err(e) => {
                                            warn!("Decompression error: {}", e);
                                        }
                                    }
                                }

                                // Send ACK
                                let ack = TensorMessage {
                                    msg_type: TensorMessageType::Ack,
                                    name: request.name.clone(),
                                    data: None,
                                    shape: vec![],
                                    metadata: "received".to_string(),
                                };
                                if swarm
                                    .behaviour_mut()
                                    .tensor_exchange
                                    .send_response(channel, ack)
                                    .is_err()
                                {
                                    warn!("Failed to send ACK");
                                }

                                println!("\n  ACK sent to sender\n");
                            }
                            _ => {
                                info!("Received non-data message: {:?}", request.msg_type);
                            }
                        }
                    }
                    SwarmEvent::Behaviour(TensorExchangeBehaviourEvent::Identify(
                        identify::Event::Received { peer_id, info },
                    )) => {
                        info!("Identified: {} ({})", peer_id, info.protocol_version);
                        for addr in info.listen_addrs {
                            swarm
                                .behaviour_mut()
                                .kademlia
                                .add_address(&peer_id, addr);
                        }
                    }
                    _ => {}
                }
            }
        }
        Command::Connect { addr, send } => {
            let peer_id = extract_peer_id(&addr).expect("Address must contain peer ID");

            println!("Mode: SENDER");
            println!("Connecting to: {}\n", addr);

            swarm.dial(addr.clone())?;

            let mut connected = false;
            let mut tensor_sent = false;

            loop {
                match swarm.select_next_some().await {
                    SwarmEvent::ConnectionEstablished { peer_id: pid, .. } if pid == peer_id => {
                        info!("Connected to: {}", pid);
                        connected = true;

                        if send && !tensor_sent {
                            // Create a test tensor
                            println!("Creating test tensor...");
                            let tensor = Tensor::randn(0f32, 1.0, &[32, 64], &device)?;
                            let shape = tensor.dims().to_vec();

                            // Compress
                            println!("Compressing tensor...");
                            let compressed = compressor.compress(&tensor)?;
                            println!(
                                "  Original: {} bytes",
                                shape.iter().product::<usize>() * 4
                            );
                            println!("  Compressed: {} bytes", compressed.size_bytes());
                            println!("  Ratio: {:.2}x", compressed.compression_ratio());

                            // Create message
                            let msg = TensorMessage {
                                msg_type: TensorMessageType::TensorData,
                                name: "gradient_layer_1".to_string(),
                                data: Some(compressed),
                                shape,
                                metadata: "training_step_42".to_string(),
                            };

                            // Send
                            println!("\nSending tensor to peer...");
                            swarm
                                .behaviour_mut()
                                .tensor_exchange
                                .send_request(&peer_id, msg);
                            tensor_sent = true;
                        }
                    }
                    SwarmEvent::Behaviour(TensorExchangeBehaviourEvent::TensorExchange(
                        request_response::Event::Message {
                            message:
                                request_response::Message::Response { response, .. },
                            ..
                        },
                    )) => {
                        match response.msg_type {
                            TensorMessageType::Ack => {
                                println!("\n  RECEIVED ACK:");
                                println!("  For tensor: {}", response.name);
                                println!("  Status: {}", response.metadata);
                                println!("\n==============================");
                                println!("Tensor exchange successful!");
                                return Ok(());
                            }
                            _ => {
                                info!("Received response: {:?}", response.msg_type);
                            }
                        }
                    }
                    SwarmEvent::Behaviour(TensorExchangeBehaviourEvent::TensorExchange(
                        request_response::Event::OutboundFailure { error, .. },
                    )) => {
                        warn!("Outbound failure: {:?}", error);
                    }
                    SwarmEvent::Behaviour(TensorExchangeBehaviourEvent::Identify(
                        identify::Event::Received { peer_id: pid, info },
                    )) => {
                        info!("Identified: {} ({})", pid, info.protocol_version);
                    }
                    _ => {}
                }

                // Timeout if not connected
                if !connected {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }
        }
    }
}
