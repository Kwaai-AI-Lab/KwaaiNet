//! # kwaai-trust — KwaaiNet Decentralized Trust Graph
//!
//! Implements the ToIP/DIF Decentralized Trust Graph (DTG) framework for
//! KwaaiNet, mapping the four-layer model onto the existing libp2p infrastructure.
//!
//! ## Layer 1 — Identity (already exists)
//! Each node's libp2p `PeerId` (Ed25519 keypair) is the trust anchor.
//! This crate exposes it as a `did:peer:` DID via [`did::peer_id_to_did`].
//!
//! ## Layer 2 — Trust Assertions (this crate)
//! Cryptographically signed [`VerifiableCredential`]s issued by trusted parties:
//! - `SummitAttendeeVC` — summit on-ramp server (Phase 1, active)
//! - `FiduciaryPledgeVC` — GliaNet Foundation (Phase 2)
//! - `VerifiedNodeVC` — Kwaai Foundation (Phase 2)
//! - `UptimeVC` / `ThroughputVC` — bootstrap servers / peers (Phase 3)
//! - `PeerEndorsementVC` — node-to-node (Phase 4)
//!
//! ## Layer 3 — Trust Scoring (this crate, Phase 2 baseline)
//! [`TrustScore`] computes a weighted, time-decayed score from stored VCs.
//! Full EigenTrust propagation over the endorsement graph comes in Phase 4.
//!
//! ## Layer 4 — Governance
//! Which issuers are trusted for which VC types, revocation policy, and
//! minimum trust thresholds are defined in the KwaaiNet governance docs.

pub mod credential;
pub mod did;
pub mod storage;
pub mod trust_score;
pub mod verify;

// Convenient re-exports
pub use credential::{
    binding_vc, fiduciary_pledge_vc, peer_endorsement_vc, summit_attendee_vc, CredentialProof,
    CredentialSubject, KwaaiCredentialType, VerifiableCredential,
};
pub use did::{did_to_peer_id, p256_spki_to_did, peer_id_to_did, verification_method};
pub use storage::CredentialStore;
pub use trust_score::TrustScore;
pub use verify::{sign_credential_bytes, verify, VerificationResult};
