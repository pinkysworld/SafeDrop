//! Reachability API — SD-011.
//!
//! POST /api/v1/reachability/check  — run reachability check and return descriptor
//! GET  /api/v1/reachability/status — get latest descriptor without re-checking

use axum::extract::State;
use axum::Json;
use serde::Serialize;
use std::sync::Arc;

use crate::api::files::AppState;
use crate::error::AppError;
use crate::evidence::audit::EventKind;
use crate::network::{self, PathType, ReachabilityStep};

#[derive(Serialize)]
pub struct ReachabilityResponse {
    pub descriptor_id: String,
    pub path_type: PathType,
    pub external_addr: Option<String>,
    pub internal_addr: String,
    pub confirmed_at: Option<String>,
    pub relay_endpoint: Option<String>,
    pub descriptor_hash: String,
    pub steps: Vec<ReachabilityStep>,
}

/// POST /api/v1/reachability/check — run full reachability check.
pub async fn check_reachability(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ReachabilityResponse>, AppError> {
    let result = network::check_reachability(
        state.config.port,
        &state.config.host,
    )
    .await;

    // Persist the descriptor
    let network_dir = state.config.network_dir();
    network::store_descriptor(&network_dir, &result.descriptor).await?;

    // Audit event
    state
        .audit_log
        .append(
            EventKind::ReachabilityConfirmed,
            result.descriptor.descriptor_id.clone(),
            serde_json::json!({
                "path_type": result.descriptor.path_type,
                "external_addr": result.descriptor.external_addr,
                "steps_count": result.steps.len(),
            }),
        )
        .await?;

    tracing::info!(
        path_type = ?result.descriptor.path_type,
        external = ?result.descriptor.external_addr,
        "reachability check complete"
    );

    Ok(Json(ReachabilityResponse {
        descriptor_id: result.descriptor.descriptor_id,
        path_type: result.descriptor.path_type,
        external_addr: result.descriptor.external_addr,
        internal_addr: result.descriptor.internal_addr,
        confirmed_at: result.descriptor.confirmed_at.map(|t| t.to_rfc3339()),
        relay_endpoint: result.descriptor.relay_endpoint,
        descriptor_hash: result.descriptor.descriptor_hash,
        steps: result.steps,
    }))
}

/// GET /api/v1/reachability/status — return last known descriptor.
pub async fn reachability_status(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ReachabilityResponse>, AppError> {
    let network_dir = state.config.network_dir();
    let desc = network::load_latest_descriptor(&network_dir).await?;

    Ok(Json(ReachabilityResponse {
        descriptor_id: desc.descriptor_id,
        path_type: desc.path_type,
        external_addr: desc.external_addr,
        internal_addr: desc.internal_addr,
        confirmed_at: desc.confirmed_at.map(|t| t.to_rfc3339()),
        relay_endpoint: desc.relay_endpoint,
        descriptor_hash: desc.descriptor_hash,
        steps: Vec::new(), // Steps not persisted on status check
    }))
}
