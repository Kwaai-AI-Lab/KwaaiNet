//! Error types for Hivemind DHT

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Serialization error: {0}")]
    Serialization(#[from] rmp_serde::encode::Error),

    #[error("Deserialization error: {0}")]
    Deserialization(#[from] rmp_serde::decode::Error),

    #[error("Protobuf encode error: {0}")]
    ProtobufEncode(#[from] prost::EncodeError),

    #[error("Protobuf decode error: {0}")]
    ProtobufDecode(#[from] prost::DecodeError),

    #[error("Value expired at {0}")]
    Expired(f64),

    #[error("Key not found: {0}")]
    NotFound(String),

    #[error("Store operation failed")]
    StoreFailed,

    #[error("Network error: {0}")]
    Network(String),

    #[error("Invalid DHT time: {0}")]
    InvalidTime(f64),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
