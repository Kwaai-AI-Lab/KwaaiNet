//! Axum route handlers for WebAuthn registration, authentication, and VC operations.

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use webauthn_rs::prelude::*;

use crate::{db, state::SharedState, vc_issuer};
use kwaai_trust::p256_spki_to_did;

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

pub struct AppError(anyhow::Error);

impl<E: Into<anyhow::Error>> From<E> for AppError {
    fn from(e: E) -> Self {
        Self(e.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("{:#}", self.0);
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}

type ApiResult<T> = Result<Json<T>, AppError>;

// ---------------------------------------------------------------------------
// Registration — begin
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct RegisterBeginRequest {
    #[serde(default)]
    pub display_name: String,
}

#[derive(Serialize)]
pub struct RegisterBeginResponse {
    pub challenge_id: Uuid,
    pub options: CreationChallengeResponse,
}

pub async fn register_begin(
    State(state): State<SharedState>,
    Json(req): Json<RegisterBeginRequest>,
) -> ApiResult<RegisterBeginResponse> {
    let user_id = Uuid::new_v4();
    let display_name = if req.display_name.is_empty() {
        "Summit Attendee".to_string()
    } else {
        req.display_name.clone()
    };

    let (ccr, reg_state) = state
        .webauthn
        .start_passkey_registration(user_id, &display_name, &display_name, None)
        .map_err(|e| anyhow::anyhow!("WebAuthn registration start failed: {e}"))?;

    let challenge_id = Uuid::new_v4();
    let state_json = serde_json::to_string(&reg_state)?;

    db::insert_pending_registration(&state.db, challenge_id, user_id, &display_name, &state_json)
        .await?;

    Ok(Json(RegisterBeginResponse {
        challenge_id,
        options: ccr,
    }))
}

// ---------------------------------------------------------------------------
// Registration — complete
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct RegisterCompleteRequest {
    pub challenge_id: Uuid,
    pub credential: RegisterPublicKeyCredential,
    /// DER/SPKI-encoded P-256 public key from `credential.response.getPublicKey()`,
    /// base64url-encoded by the browser.
    pub public_key_spki_b64: String,
}

#[derive(Serialize)]
pub struct RegisterCompleteResponse {
    pub did: String,
    pub session_token: Uuid,
    pub vc: serde_json::Value,
    pub tier: String,
}

pub async fn register_complete(
    State(state): State<SharedState>,
    Json(req): Json<RegisterCompleteRequest>,
) -> ApiResult<RegisterCompleteResponse> {
    let (user_id, display_name, state_json) =
        db::take_pending_registration(&state.db, req.challenge_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Unknown or expired challenge_id"))?;

    let reg_state: PasskeyRegistration = serde_json::from_str(&state_json)?;

    let passkey = state
        .webauthn
        .finish_passkey_registration(&req.credential, &reg_state)
        .map_err(|e| anyhow::anyhow!("WebAuthn registration finish failed: {e}"))?;

    // Derive did:key: from the P-256 SPKI bytes sent by the browser
    let spki_bytes = URL_SAFE_NO_PAD
        .decode(&req.public_key_spki_b64)
        .map_err(|e| anyhow::anyhow!("Invalid public_key_spki_b64: {e}"))?;
    let did = p256_spki_to_did(&spki_bytes)?;

    // Persist the passkey credential
    let cred_id: Vec<u8> = passkey.cred_id().to_vec();
    let passkey_json = serde_json::to_string(&passkey)?;
    db::insert_passkey_credential(
        &state.db,
        user_id,
        &cred_id,
        &did,
        &display_name,
        &passkey_json,
    )
    .await?;

    // Issue and sign SummitAttendeeVC
    let vc = vc_issuer::issue_summit_attendee_vc(&state.signing_key, &state.issuer_did, &did)?;
    let vc_json_str = serde_json::to_string(&vc)?;
    db::insert_vc(&state.db, &did, "SummitAttendeeVC", &vc_json_str).await?;

    // Create session
    let session_token = Uuid::new_v4();
    db::insert_session(&state.db, session_token, user_id, &did).await?;

    Ok(Json(RegisterCompleteResponse {
        did,
        session_token,
        vc: serde_json::to_value(&vc)?,
        tier: "Known".to_string(),
    }))
}

// ---------------------------------------------------------------------------
// Authentication — begin
// The browser provides the user's DID (stored in localStorage after registration)
// so we can look up their passkeys without discoverable credentials.
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct AuthBeginRequest {
    pub did: String,
}

#[derive(Serialize)]
pub struct AuthBeginResponse {
    pub challenge_id: Uuid,
    pub options: RequestChallengeResponse,
}

pub async fn auth_begin(
    State(state): State<SharedState>,
    Json(req): Json<AuthBeginRequest>,
) -> ApiResult<AuthBeginResponse> {
    let rows = db::get_passkeys_for_did(&state.db, &req.did).await?;
    if rows.is_empty() {
        return Err(AppError(anyhow::anyhow!(
            "No passkeys found for this identity"
        )));
    }

    let passkeys: Vec<Passkey> = rows
        .iter()
        .filter_map(|(_, json)| serde_json::from_str(json).ok())
        .collect();

    let (rcr, auth_state) = state
        .webauthn
        .start_passkey_authentication(&passkeys)
        .map_err(|e| anyhow::anyhow!("WebAuthn auth start failed: {e}"))?;

    let challenge_id = Uuid::new_v4();
    let state_json = serde_json::to_string(&auth_state)?;
    db::insert_pending_authentication(&state.db, challenge_id, &state_json).await?;

    Ok(Json(AuthBeginResponse {
        challenge_id,
        options: rcr,
    }))
}

// ---------------------------------------------------------------------------
// Authentication — complete
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct AuthCompleteRequest {
    pub challenge_id: Uuid,
    pub did: String,
    pub credential: PublicKeyCredential,
}

#[derive(Serialize)]
pub struct AuthCompleteResponse {
    pub did: String,
    pub session_token: Uuid,
    pub vcs: Vec<serde_json::Value>,
    pub tier: String,
}

pub async fn auth_complete(
    State(state): State<SharedState>,
    Json(req): Json<AuthCompleteRequest>,
) -> ApiResult<AuthCompleteResponse> {
    let state_json = db::take_pending_authentication(&state.db, req.challenge_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Unknown or expired challenge_id"))?;

    let auth_state: PasskeyAuthentication = serde_json::from_str(&state_json)?;

    let rows = db::get_passkeys_for_did(&state.db, &req.did).await?;
    if rows.is_empty() {
        return Err(AppError(anyhow::anyhow!(
            "No passkeys found for this identity"
        )));
    }

    let (user_id, _) = rows[0];
    let _passkeys: Vec<Passkey> = rows
        .iter()
        .filter_map(|(_, json)| serde_json::from_str(json).ok())
        .collect();

    state
        .webauthn
        .finish_passkey_authentication(&req.credential, &auth_state)
        .map_err(|e| anyhow::anyhow!("WebAuthn auth finish failed: {e}"))?;

    let session_token = Uuid::new_v4();
    db::insert_session(&state.db, session_token, user_id, &req.did).await?;

    let vc_strings = db::get_vcs_for_subject(&state.db, &req.did).await?;
    let vcs: Vec<serde_json::Value> = vc_strings
        .iter()
        .filter_map(|s| serde_json::from_str(s).ok())
        .collect();

    Ok(Json(AuthCompleteResponse {
        did: req.did.clone(),
        session_token,
        vcs,
        tier: compute_tier(&vc_strings),
    }))
}

// ---------------------------------------------------------------------------
// Credentials — fetch for authenticated user
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct SessionQuery {
    pub token: Uuid,
}

#[derive(Serialize)]
pub struct CredentialsResponse {
    pub did: String,
    pub vcs: Vec<serde_json::Value>,
    pub tier: String,
}

pub async fn get_credentials(
    State(state): State<SharedState>,
    Query(q): Query<SessionQuery>,
) -> ApiResult<CredentialsResponse> {
    let (_user_id, passkey_did) = db::get_session(&state.db, q.token)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Invalid or expired session"))?;

    let vc_strings = db::get_vcs_for_subject(&state.db, &passkey_did).await?;
    let vcs: Vec<serde_json::Value> = vc_strings
        .iter()
        .filter_map(|s| serde_json::from_str(s).ok())
        .collect();

    Ok(Json(CredentialsResponse {
        tier: compute_tier(&vc_strings),
        did: passkey_did,
        vcs,
    }))
}

// ---------------------------------------------------------------------------
// Node binding
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct BindNodeRequest {
    pub session_token: Uuid,
    pub node_did: String,
}

#[derive(Serialize)]
pub struct BindNodeResponse {
    pub binding_vc: serde_json::Value,
}

pub async fn bind_node(
    State(state): State<SharedState>,
    Json(req): Json<BindNodeRequest>,
) -> ApiResult<BindNodeResponse> {
    let (_user_id, passkey_did) = db::get_session(&state.db, req.session_token)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Invalid or expired session"))?;

    if !req.node_did.starts_with("did:peer:") {
        return Err(AppError(anyhow::anyhow!(
            "node_did must be a did:peer: identifier"
        )));
    }

    let binding_vc = vc_issuer::issue_binding_vc(
        &state.signing_key,
        &state.issuer_did,
        &req.node_did,
        &passkey_did,
    )?;
    let vc_json_str = serde_json::to_string(&binding_vc)?;

    db::insert_node_binding(&state.db, &passkey_did, &req.node_did, &vc_json_str).await?;
    db::insert_vc(&state.db, &req.node_did, "BindingVC", &vc_json_str).await?;

    Ok(Json(BindNodeResponse {
        binding_vc: serde_json::to_value(&binding_vc)?,
    }))
}

// ---------------------------------------------------------------------------
// Health
// ---------------------------------------------------------------------------

pub async fn health() -> &'static str {
    "ok"
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn compute_tier(vc_strings: &[String]) -> String {
    use kwaai_trust::{TrustScore, VerifiableCredential};
    let vcs: Vec<VerifiableCredential> = vc_strings
        .iter()
        .filter_map(|s| serde_json::from_str(s).ok())
        .collect();
    TrustScore::from_credentials(&vcs).tier_label().to_string()
}
