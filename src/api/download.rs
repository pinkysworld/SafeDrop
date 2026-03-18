//! Resumable segmented download with delivery receipts.

use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::api::files::AppState;
use crate::error::AppError;
use crate::evidence::audit::EventKind;
use crate::evidence::receipt;
use crate::storage::manifest;

// ── Download by share token ──

#[derive(Deserialize)]
pub struct DownloadQuery {
    /// Optional segment index for resumable download.
    pub segment: Option<u32>,
}

/// Recipient-facing download: GET /s/{token}?segment=N
pub async fn download_by_token(
    State(state): State<Arc<AppState>>,
    Path(token): Path<String>,
    Query(query): Query<DownloadQuery>,
) -> Result<Response, AppError> {
    let share = state.share_store.find_by_token(&token).await?;

    if !share.is_accessible() {
        return Err(AppError::BadRequest(format!(
            "share is {}",
            serde_json::to_string(&share.effective_status())
                .unwrap_or_else(|_| "inaccessible".to_string())
        )));
    }

    let m = manifest::load_manifest(&state.config.manifests_dir(), &share.manifest_id).await?;

    // If segment requested, return just that chunk
    if let Some(seg) = query.segment {
        return serve_segment(&state, &share.share_id, &m, seg).await;
    }

    // Full download: concatenate all chunks
    state
        .audit_log
        .append(
            EventKind::DownloadStarted,
            share.share_id.clone(),
            serde_json::json!({
                "manifest_id": share.manifest_id,
                "segments": m.chunking.chunk_count,
            }),
        )
        .await?;

    let mut body_bytes = Vec::with_capacity(m.size_bytes as usize);
    for chunk_ref in &m.chunks {
        let data = state.chunk_store.get(&chunk_ref.hash).await?;
        body_bytes.extend_from_slice(&data);
    }

    // Record the download
    state.share_store.record_download(&share.share_id).await?;

    // Emit completion audit event and create receipt
    state
        .audit_log
        .append(
            EventKind::DownloadCompleted,
            share.share_id.clone(),
            serde_json::json!({
                "manifest_id": share.manifest_id,
                "total_bytes": body_bytes.len(),
            }),
        )
        .await?;

    let rcpt = receipt::create_receipt(
        &share.share_id,
        &share.manifest_id,
        body_bytes.len() as u64,
        m.chunking.chunk_count,
    );
    receipt::store_receipt(&state.config.receipts_dir(), &rcpt).await?;

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "application/octet-stream".parse().unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"{}\"", m.file_name_alias)
            .parse()
            .unwrap(),
    );
    headers.insert(
        header::CONTENT_LENGTH,
        body_bytes.len().to_string().parse().unwrap(),
    );
    headers.insert(
        "X-SafeDrop-Manifest",
        m.manifest_id.parse().unwrap(),
    );
    headers.insert(
        "X-SafeDrop-Receipt",
        rcpt.receipt_id.parse().unwrap(),
    );

    Ok((StatusCode::OK, headers, body_bytes).into_response())
}

/// Serve a single segment for resumable download.
async fn serve_segment(
    state: &Arc<AppState>,
    share_id: &str,
    m: &manifest::Manifest,
    segment: u32,
) -> Result<Response, AppError> {
    if segment >= m.chunking.chunk_count {
        return Err(AppError::BadRequest(format!(
            "segment {segment} out of range (0..{})",
            m.chunking.chunk_count
        )));
    }

    let chunk_ref = &m.chunks[segment as usize];
    let data = state.chunk_store.get(&chunk_ref.hash).await?;

    state
        .audit_log
        .append(
            EventKind::SegmentAcknowledged,
            share_id.to_string(),
            serde_json::json!({
                "segment": segment,
                "chunk_hash": chunk_ref.hash,
                "size": data.len(),
            }),
        )
        .await?;

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "application/octet-stream".parse().unwrap(),
    );
    headers.insert(
        header::CONTENT_LENGTH,
        data.len().to_string().parse().unwrap(),
    );
    headers.insert(
        "X-SafeDrop-Segment",
        segment.to_string().parse().unwrap(),
    );
    headers.insert(
        "X-SafeDrop-Segment-Total",
        m.chunking.chunk_count.to_string().parse().unwrap(),
    );
    headers.insert(
        "X-SafeDrop-Chunk-Hash",
        chunk_ref.hash.parse().unwrap(),
    );

    Ok((StatusCode::OK, headers, data).into_response())
}

// ── Resume info ──

#[derive(Serialize)]
pub struct ResumeInfo {
    pub manifest_id: String,
    pub total_segments: u32,
    pub segment_size: usize,
    pub total_bytes: u64,
    pub file_name: String,
}

/// POST /api/v1/shares/{share_id}/resume — returns info needed to resume.
pub async fn resume_info(
    State(state): State<Arc<AppState>>,
    Path(share_id): Path<String>,
) -> Result<Json<ResumeInfo>, AppError> {
    let share = state.share_store.load(&share_id).await?;
    if !share.is_accessible() {
        return Err(AppError::BadRequest("share is not accessible".to_string()));
    }

    let m = manifest::load_manifest(&state.config.manifests_dir(), &share.manifest_id).await?;

    Ok(Json(ResumeInfo {
        manifest_id: m.manifest_id,
        total_segments: m.chunking.chunk_count,
        segment_size: crate::storage::chunk::CHUNK_SIZE,
        total_bytes: m.size_bytes,
        file_name: m.file_name_alias,
    }))
}

// ── Segment acknowledgment ──

#[derive(Deserialize)]
pub struct AckRequest {
    pub segment: u32,
    pub chunk_hash: String,
}

#[derive(Serialize)]
pub struct AckResponse {
    pub acknowledged: bool,
    pub segment: u32,
}

/// POST /api/v1/shares/{share_id}/ack — acknowledge a received segment.
pub async fn ack_segment(
    State(state): State<Arc<AppState>>,
    Path(share_id): Path<String>,
    Json(req): Json<AckRequest>,
) -> Result<Json<AckResponse>, AppError> {
    let share = state.share_store.load(&share_id).await?;
    let m = manifest::load_manifest(&state.config.manifests_dir(), &share.manifest_id).await?;

    if req.segment >= m.chunking.chunk_count {
        return Err(AppError::BadRequest("segment out of range".to_string()));
    }

    let expected_hash = &m.chunks[req.segment as usize].hash;
    if req.chunk_hash != *expected_hash {
        return Err(AppError::BadRequest("chunk hash mismatch".to_string()));
    }

    state
        .audit_log
        .append(
            EventKind::SegmentAcknowledged,
            share_id,
            serde_json::json!({
                "segment": req.segment,
                "verified_hash": req.chunk_hash,
            }),
        )
        .await?;

    Ok(Json(AckResponse {
        acknowledged: true,
        segment: req.segment,
    }))
}

// ── Get receipt for a share ──

#[derive(Serialize)]
pub struct ReceiptResponse {
    pub receipt_id: String,
    pub share_id: String,
    pub manifest_id: String,
    pub total_bytes: u64,
    pub segment_count: u32,
    pub completed_at: String,
    pub hash: String,
}

/// GET /api/v1/evidence/{share_id} — fetch the delivery receipt.
pub async fn get_evidence(
    State(state): State<Arc<AppState>>,
    Path(share_id): Path<String>,
) -> Result<Json<ReceiptResponse>, AppError> {
    let rcpt = receipt::load_receipt(&state.config.receipts_dir(), &share_id).await?;

    Ok(Json(ReceiptResponse {
        receipt_id: rcpt.receipt_id,
        share_id: rcpt.share_id,
        manifest_id: rcpt.manifest_id,
        total_bytes: rcpt.total_bytes,
        segment_count: rcpt.segment_count,
        completed_at: rcpt.completed_at.to_rfc3339(),
        hash: rcpt.receipt_hash,
    }))
}
