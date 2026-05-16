//! kwaai-rpc — gRPC IPC surface for the `kwaainet` daemon.
//!
//! The proto definitions live in [`proto/kwaai.proto`]; this crate just
//! re-exports the generated tonic code under a stable path so callers don't
//! have to reach into `tonic::include_proto!` themselves.
//!
//! ## Usage (server, inside the daemon)
//!
//! ```ignore
//! use kwaai_rpc::v1::{kwaai_net_server::KwaaiNetServer, KwaaiNetService};
//! tonic::transport::Server::builder()
//!     .add_service(KwaaiNetServer::new(MyImpl))
//!     .serve(addr).await?;
//! ```
//!
//! ## Usage (client, e.g. the Flutter GUI talking over Unix socket)
//!
//! ```ignore
//! use kwaai_rpc::v1::kwaai_net_client::KwaaiNetClient;
//! let channel = tonic::transport::Endpoint::try_from("http://[::]:8093")?
//!     .connect().await?;
//! let mut client = KwaaiNetClient::new(channel);
//! ```

/// Generated protobuf + tonic code for the `kwaai.v1` package.
///
/// Re-exported under `v1` so callers write `kwaai_rpc::v1::ChatMessage`
/// rather than the (longer, less discoverable) `kwaai_rpc::kwaai::v1::…`
/// path that prost would otherwise emit.
pub mod v1 {
    tonic::include_proto!("kwaai.v1");
}

// Re-export tonic so downstream crates don't have to add their own
// dependency on the exact tonic version we generated against. This keeps
// the wire types and the transport pinned together.
pub use tonic;
