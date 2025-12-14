//! Protobuf protocol definitions for p2p daemon communication
//!
//! This module contains the generated protobuf code from p2pd.proto.
//! The proto file is copied from go-libp2p-daemon during build.

// Include the generated protobuf code
pub mod p2pd {
    #[cfg(windows)]
    include!(concat!(env!("OUT_DIR"), "\\p2pd.pb.rs"));

    #[cfg(not(windows))]
    include!(concat!(env!("OUT_DIR"), "/p2pd.pb.rs"));
}
