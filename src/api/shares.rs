//! Share lifecycle API: issuance, expiry, revocation, and download.

use axum::extract::{Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::api::files::AppState;
use crate::error::AppError;
use crate::evidence::audit::EventKind;
use crate::share::{RecipientScope, ShareStatus};

// ── Create Share ──

#[derive(Deserialize)]
pub struct CreateShareRequest {
    pub manifest_id: String,
    #[serde(default = "default_scope")]
    pub recipient_scope: RecipientScope,
    /// Time-to-live in hours (default 24).
    #[serde(default = "default_ttl")]
    pub ttl_hours: u32,
    #[serde(default = "default_download_limit")]
    pub download_limit: u32,
    #[serde(default = "default_relay")]
    pub relay_allowed: bool,
}

fn default_scope() -> RecipientScope { RecipientScope::Anonymous }
fn default_ttl() -> u32 { 24 }
fn default_download_limit() -> u32 { 1 }
fn default_relay() -> bool { true }

#[derive(Serialize)]
pub struct ShareResponse {
    pub share_id: String,
    pub token: String,
    pub download_url: String,
    pub expires_at: String,
    pub status: ShareStatus,
}

pub async fn create_share(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateShareRequest>,
) -> Result<Json<ShareResponse>, AppError> {
    // Verify the manifest exists
    let manifests_dir = state.config.manifests_dir();
    let manifest_path = manifests_dir.join(format!("{}.json", &req.manifest_id));
    if !manifest_path.exists() {
        return Err(AppError::NotFound(format!(
            "manifest {}",
            req.manifest_id
        )));
    }

    let share = state
        .share_store
        .issue(
            req.manifest_id,
            req.recipient_scope,
            req.ttl_hours,
            req.download_limit,
            req.relay_allowed,
        )
        .await?;

    // Audit event
    state
        .audit_log
        .append(
            EventKind::ShareIssued,
            share.share_id.clone(),
            serde_json::json!({
                "manifest_id": share.manifest_id,
                "expires_at": share.expires_at.to_rfc3339(),
                "download_limit": share.download_limit,
            }),
        )
        .await?;

    let status = share.effective_status();
    Ok(Json(ShareResponse {
        share_id: share.share_id,
        download_url: format!("/s/{}", share.token),
        token: share.token,
        expires_at: share.expires_at.to_rfc3339(),
        status,
    }))
}

// ── Expire / Revoke Share ──

pub async fn expire_share(
    State(state): State<Arc<AppState>>,
    Path(share_id): Path<String>,
) -> Result<Json<ShareResponse>, AppError> {
    let share = state.share_store.revoke(&share_id).await?;

    state
        .audit_log
        .append(
            EventKind::ShareExpired,
            share.share_id.clone(),
            serde_json::json!({"revoked_at": share.revoked_at}),
        )
        .await?;

    let status = share.effective_status();
    Ok(Json(ShareResponse {
        share_id: share.share_id,
        download_url: format!("/s/{}", share.token),
        token: share.token,
        expires_at: share.expires_at.to_rfc3339(),
        status,
    }))
}

// ── Get Share Metadata ──

#[derive(Serialize)]
pub struct ShareMetaResponse {
    pub share_id: String,
    pub manifest_id: String,
    pub file_name: String,
    pub size_bytes: u64,
    pub status: ShareStatus,
    pub expires_at: String,
    pub download_count: u32,
    pub download_limit: u32,
}

pub async fn get_share(
    State(state): State<Arc<AppState>>,
    Path(share_id): Path<String>,
) -> Result<Json<ShareMetaResponse>, AppError> {
    let share = state.share_store.load(&share_id).await?;
    let manifest = crate::storage::manifest::load_manifest(
        &state.config.manifests_dir(),
        &share.manifest_id,
    )
    .await?;

    let status = share.effective_status();
    Ok(Json(ShareMetaResponse {
        share_id: share.share_id,
        manifest_id: share.manifest_id,
        file_name: manifest.file_name_alias,
        size_bytes: manifest.size_bytes,
        status,
        expires_at: share.expires_at.to_rfc3339(),
        download_count: share.download_count,
        download_limit: share.download_limit,
    }))
}

// ── List All Shares ──

pub async fn list_shares(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ShareMetaResponse>>, AppError> {
    let shares = state.share_store.list_all().await?;
    let manifests_dir = state.config.manifests_dir();
    let mut out = Vec::with_capacity(shares.len());

    for share in shares {
        let file_name = match crate::storage::manifest::load_manifest(
            &manifests_dir,
            &share.manifest_id,
        )
        .await
        {
            Ok(m) => m.file_name_alias,
            Err(_) => "(unknown)".to_string(),
        };

        let status = share.effective_status();
        out.push(ShareMetaResponse {
            share_id: share.share_id,
            manifest_id: share.manifest_id,
            file_name,
            size_bytes: 0,
            status,
            expires_at: share.expires_at.to_rfc3339(),
            download_count: share.download_count,
            download_limit: share.download_limit,
        });
    }
    Ok(Json(out))
}
