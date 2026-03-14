use axum::extract::State;
use axum::Json;
use serde::Serialize;
use std::sync::Arc;

use crate::api::files::AppState;
use crate::error::AppError;
use crate::evidence::audit::AuditEvent;

#[derive(Serialize)]
pub struct AuditResponse {
    pub events: Vec<AuditEvent>,
    pub count: usize,
    pub chain_valid: bool,
}

pub async fn get_events(
    State(state): State<Arc<AppState>>,
) -> Result<Json<AuditResponse>, AppError> {
    let events = state.audit_log.read_all().await?;
    let chain_valid = state.audit_log.verify_chain().await?;
    let count = events.len();
    Ok(Json(AuditResponse {
        events,
        count,
        chain_valid,
    }))
}
