use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::AppError;
use crate::evidence::audit::{AuditLog, EventKind};
use crate::share::ShareStore;
use crate::storage::chunk::ChunkStore;
use crate::storage::manifest;

pub struct AppState {
    pub chunk_store: ChunkStore,
    pub audit_log: AuditLog,
    pub share_store: ShareStore,
    pub config: crate::config::Config,
}

#[derive(Deserialize)]
pub struct ImportRequest {
    /// Path to a local file to ingest
    pub file_path: String,
    /// Display name for the file
    pub file_name: Option<String>,
    /// MIME content type
    pub content_type: Option<String>,
}

#[derive(Serialize)]
pub struct ImportResponse {
    pub manifest_id: String,
    pub size_bytes: u64,
    pub chunk_count: u32,
}

pub async fn import_file(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ImportRequest>,
) -> Result<Json<ImportResponse>, AppError> {
    let path = std::path::Path::new(&req.file_path);
    if !path.exists() {
        return Err(AppError::BadRequest(format!(
            "file not found: {}",
            req.file_path
        )));
    }

    // Resolve file_name from the path if not provided
    let file_name = req.file_name.unwrap_or_else(|| {
        path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string())
    });
    let content_type = req.content_type.unwrap_or_else(|| "application/octet-stream".to_string());

    // Ingest file into chunk store
    let manifest = manifest::ingest_file(
        &state.chunk_store,
        path,
        &file_name,
        &content_type,
    )
    .await?;

    // Store manifest to disk
    manifest::store_manifest(&state.config.manifests_dir(), &manifest).await?;

    // Emit audit event
    state
        .audit_log
        .append(
            EventKind::ManifestCommitted,
            manifest.manifest_id.clone(),
            serde_json::json!({
                "file_name": file_name,
                "size_bytes": manifest.size_bytes,
                "chunk_count": manifest.chunking.chunk_count,
            }),
        )
        .await?;

    Ok(Json(ImportResponse {
        manifest_id: manifest.manifest_id,
        size_bytes: manifest.size_bytes,
        chunk_count: manifest.chunking.chunk_count,
    }))
}
