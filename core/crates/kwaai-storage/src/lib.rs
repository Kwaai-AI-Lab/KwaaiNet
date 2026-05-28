//! kwaai-storage — Multi-tenant vector storage fabric for KwaaiNet
//!
//! Each KwaaiNet node can act as an Eve (storage host), holding encrypted
//! vectors for multiple Bobs (data owners). This crate provides:
//!
//! - **Embedded HNSW storage** via hnsw_rs (pure Rust, no Docker/Postgres)
//! - **Persistent metadata** via redb (pure Rust ACID key-value store)
//! - **Tenant management** (create, list, delete, capacity enforcement)
//! - **REST API** for remote Bobs to upload/search vectors

pub mod api;
pub mod db;
pub mod tenant;
pub mod vectors;

pub use api::{build_app, run_storage_api};
pub use db::{StorageDb, TenantIndex};
pub use tenant::{TenantInfo, TenantManager, TenantStats};
pub use vectors::{SearchResult, VectorStore};
