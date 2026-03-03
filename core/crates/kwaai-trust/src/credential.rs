//! W3C Verifiable Credential types for KwaaiNet
//!
//! Implements the W3C Verifiable Credentials Data Model 1.1 with
//! KwaaiNet-specific credential types for the decentralized trust graph (DTG).
//!
//! Each credential type maps to a layer-2 attestation in the ToIP stack:
//! - `SummitAttendeeVC`   — Phase 1 (Summit demo)
//! - `FiduciaryPledgeVC`  — Phase 2 (GliaNet Fiduciary Pledge)
//! - `VerifiedNodeVC`     — Phase 2 (Kwaai Foundation onboarding)
//! - `UptimeVC`           — Phase 3 (bootstrap-server issued)
//! - `ThroughputVC`       — Phase 3 (peer-witnessed)
//! - `PeerEndorsementVC`  — Phase 4 (EigenTrust endorsements)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// W3C Verifiable Credentials context URLs
pub const VC_CONTEXT_V1: &str = "https://www.w3.org/2018/credentials/v1";
pub const KWAAI_CONTEXT_V1: &str = "https://kwaai.ai/credentials/v1";

/// KwaaiNet-specific credential types in the trust graph
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KwaaiCredentialType {
    /// Attendance at a Kwaai Personal AI Summit — the first VC in the system (Phase 1)
    SummitAttendeeVC,
    /// Node operator signed the GliaNet Fiduciary Pledge (Phase 2)
    FiduciaryPledgeVC,
    /// Node passed Kwaai Foundation onboarding verification (Phase 2)
    VerifiedNodeVC,
    /// Node demonstrated uptime ≥ threshold over N days, issued by bootstrap servers (Phase 3)
    UptimeVC,
    /// Peer-witnessed throughput within X% of the node's announced value (Phase 3)
    ThroughputVC,
    /// Peer-to-peer endorsement of reliability (Phase 4, EigenTrust source)
    PeerEndorsementVC,
    /// Binding between a passkey `did:key:` identity and a node `did:peer:` (Phase 1+)
    BindingVC,
}

impl KwaaiCredentialType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SummitAttendeeVC => "SummitAttendeeVC",
            Self::FiduciaryPledgeVC => "FiduciaryPledgeVC",
            Self::VerifiedNodeVC => "VerifiedNodeVC",
            Self::UptimeVC => "UptimeVC",
            Self::ThroughputVC => "ThroughputVC",
            Self::PeerEndorsementVC => "PeerEndorsementVC",
            Self::BindingVC => "BindingVC",
        }
    }

    /// Contribution weight to the local trust score (Layer 3).
    ///
    /// Weights are calibrated so a node with FiduciaryPledge + VerifiedNode +
    /// Uptime + Throughput reaches ~0.85 — leaving headroom for peer endorsements.
    pub fn trust_weight(&self) -> f64 {
        match self {
            Self::FiduciaryPledgeVC => 0.30,
            Self::VerifiedNodeVC => 0.20,
            Self::UptimeVC => 0.20,
            Self::ThroughputVC => 0.15,
            Self::SummitAttendeeVC => 0.10,
            Self::PeerEndorsementVC => 0.05,
            // BindingVC is a structural link, not a trust weight contributor
            Self::BindingVC => 0.0,
        }
    }

    /// Parse from a credential type string (e.g., `"SummitAttendeeVC"`)
    pub fn from_type_str(s: &str) -> Option<Self> {
        match s {
            "SummitAttendeeVC" => Some(Self::SummitAttendeeVC),
            "FiduciaryPledgeVC" => Some(Self::FiduciaryPledgeVC),
            "VerifiedNodeVC" => Some(Self::VerifiedNodeVC),
            "UptimeVC" => Some(Self::UptimeVC),
            "ThroughputVC" => Some(Self::ThroughputVC),
            "PeerEndorsementVC" => Some(Self::PeerEndorsementVC),
            "BindingVC" => Some(Self::BindingVC),
            _ => None,
        }
    }
}

/// Subject of a Verifiable Credential — the node or operator being attested about
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialSubject {
    /// DID of the subject (e.g., `did:peer:<base58-peer-id>`)
    pub id: String,
    /// Credential-specific claims (event name, pledge hash, observed uptime, etc.)
    #[serde(flatten)]
    pub claims: HashMap<String, serde_json::Value>,
}

impl CredentialSubject {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            claims: HashMap::new(),
        }
    }

    pub fn with_claim(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.claims.insert(key.into(), value);
        self
    }
}

/// Cryptographic proof attached to a Verifiable Credential
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialProof {
    /// Proof type — always `"Ed25519Signature2020"` for KwaaiNet
    #[serde(rename = "type")]
    pub proof_type: String,
    /// When the proof was created
    pub created: DateTime<Utc>,
    /// Reference to the issuer's signing key: `<issuer-did>#key-1`
    #[serde(rename = "verificationMethod")]
    pub verification_method: String,
    /// Proof purpose — always `"assertionMethod"` for credential issuance
    #[serde(rename = "proofPurpose")]
    pub proof_purpose: String,
    /// Base64url-encoded 64-byte Ed25519 signature over the VC (without the `proof` field)
    #[serde(rename = "proofValue")]
    pub proof_value: String,
}

/// W3C Verifiable Credential (Data Model 1.1)
///
/// VCs carry cryptographically signed attestations about KwaaiNet nodes.
/// Both `issuer` and `subject` are `did:peer:` DIDs that map directly to
/// libp2p PeerIds — the trust anchor that already exists in the network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiableCredential {
    /// JSON-LD context
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    /// Unique identifier (e.g., `urn:kwaai:vc:<sha256-hash>`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Credential types — always starts with `"VerifiableCredential"` plus a KwaaiNet type
    #[serde(rename = "type")]
    pub credential_type: Vec<String>,
    /// DID of the issuing entity (e.g., GliaNet Foundation, Kwaai Foundation, bootstrap peer)
    pub issuer: String,
    /// Date the credential was issued
    #[serde(rename = "issuanceDate")]
    pub issuance_date: DateTime<Utc>,
    /// Optional expiry date
    #[serde(rename = "expirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<DateTime<Utc>>,
    /// The claims being made about the subject node/operator
    #[serde(rename = "credentialSubject")]
    pub subject: CredentialSubject,
    /// Ed25519 cryptographic proof by the issuer (absent on draft/unsigned VCs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<CredentialProof>,
}

impl VerifiableCredential {
    /// Create a new unsigned VC
    pub fn new(
        issuer: impl Into<String>,
        subject: CredentialSubject,
        credential_types: Vec<String>,
    ) -> Self {
        Self {
            context: vec![VC_CONTEXT_V1.to_string(), KWAAI_CONTEXT_V1.to_string()],
            id: None,
            credential_type: credential_types,
            issuer: issuer.into(),
            issuance_date: Utc::now(),
            expiration_date: None,
            subject,
            proof: None,
        }
    }

    /// Extract the KwaaiNet-specific credential type, if any
    pub fn kwaai_type(&self) -> Option<KwaaiCredentialType> {
        self.credential_type
            .iter()
            .find_map(|t| KwaaiCredentialType::from_type_str(t))
    }

    /// Returns `true` if the credential's expiration date has passed
    pub fn is_expired(&self) -> bool {
        self.expiration_date
            .map(|exp| exp < Utc::now())
            .unwrap_or(false)
    }

    /// DID of the credential subject
    pub fn subject_did(&self) -> &str {
        &self.subject.id
    }

    /// DID of the credential issuer
    pub fn issuer_did(&self) -> &str {
        &self.issuer
    }

    /// Serialize to compact single-line JSON (for DHT storage)
    pub fn to_compact_json(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    /// Parse from a JSON string
    pub fn from_json(s: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(s)?)
    }

    /// Serialize to the canonical bytes that are signed/verified (VC without the `proof` field)
    pub fn to_signing_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let mut unsigned = self.clone();
        unsigned.proof = None;
        Ok(serde_json::to_string(&unsigned)?.into_bytes())
    }
}

// ---------------------------------------------------------------------------
// Convenience constructors for each credential type
// ---------------------------------------------------------------------------

/// Build a `SummitAttendeeVC` — issued by the summit on-ramp server (Phase 1)
pub fn summit_attendee_vc(
    issuer_did: impl Into<String>,
    subject_did: impl Into<String>,
    event_name: impl Into<String>,
    event_date: impl Into<String>,
) -> VerifiableCredential {
    let subject = CredentialSubject::new(subject_did)
        .with_claim("eventName", serde_json::Value::String(event_name.into()))
        .with_claim("eventDate", serde_json::Value::String(event_date.into()));

    let mut vc = VerifiableCredential::new(
        issuer_did,
        subject,
        vec![
            "VerifiableCredential".to_string(),
            "SummitAttendeeVC".to_string(),
        ],
    );
    // Summit VCs are valid for 2 years
    vc.expiration_date = Some(Utc::now() + chrono::Duration::days(730));
    vc
}

/// Build a `FiduciaryPledgeVC` — issued by GliaNet Foundation (Phase 2)
pub fn fiduciary_pledge_vc(
    issuer_did: impl Into<String>,
    subject_did: impl Into<String>,
    pledge_hash: impl Into<String>,
) -> VerifiableCredential {
    let subject = CredentialSubject::new(subject_did)
        .with_claim("pledgeHash", serde_json::Value::String(pledge_hash.into()))
        .with_claim(
            "pledgeName",
            serde_json::Value::String("GliaNet Fiduciary Pledge v1".to_string()),
        );

    VerifiableCredential::new(
        issuer_did,
        subject,
        vec![
            "VerifiableCredential".to_string(),
            "FiduciaryPledgeVC".to_string(),
        ],
    )
}

/// Build a `PeerEndorsementVC` — issued by one node endorsing another (Phase 4)
pub fn peer_endorsement_vc(
    issuer_did: impl Into<String>,
    subject_did: impl Into<String>,
    interaction_count: u64,
) -> VerifiableCredential {
    let subject = CredentialSubject::new(subject_did).with_claim(
        "interactionCount",
        serde_json::Value::Number(interaction_count.into()),
    );

    let mut vc = VerifiableCredential::new(
        issuer_did,
        subject,
        vec![
            "VerifiableCredential".to_string(),
            "PeerEndorsementVC".to_string(),
        ],
    );
    // Peer endorsements expire after 90 days (require fresh interactions)
    vc.expiration_date = Some(Utc::now() + chrono::Duration::days(90));
    vc
}

/// Build a `BindingVC` — issued by the summit server to link a passkey `did:key:`
/// to a KwaaiNet node `did:peer:` so the node inherits the attendee's trust score.
pub fn binding_vc(
    issuer_did: impl Into<String>,
    node_did: impl Into<String>,
    passkey_did: impl Into<String>,
) -> VerifiableCredential {
    let passkey_did_str = passkey_did.into();
    let subject = CredentialSubject::new(node_did)
        .with_claim("linkedIdentity", serde_json::Value::String(passkey_did_str))
        .with_claim(
            "linkType",
            serde_json::Value::String("PasskeyBinding".to_string()),
        );

    VerifiableCredential::new(
        issuer_did,
        subject,
        vec!["VerifiableCredential".to_string(), "BindingVC".to_string()],
    )
}
