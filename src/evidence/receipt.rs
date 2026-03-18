//! Delivery receipts: proof-carrying evidence of completed transfers.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

use crate::error::AppError;

/// A delivery receipt proving a file was successfully transferred.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    pub receipt_id: String,
    pub share_id: String,
    pub manifest_id: String,
    pub total_bytes: u64,
    pub segment_count: u32,
    pub completed_at: DateTime<Utc>,
    /// BLAKE3 hash of the receipt's canonical fields.
    pub receipt_hash: String,
}

/// Create a receipt for a completed download.
pub fn create_receipt(
    share_id: &str,
    manifest_id: &str,
    total_bytes: u64,
    segment_count: u32,
) -> Receipt {
    let receipt_id = uuid::Uuid::new_v4().to_string();
    let completed_at = Utc::now();

    let canonical = serde_json::json!({
        "receipt_id": receipt_id,
        "share_id": share_id,
        "manifest_id": manifest_id,
        "total_bytes": total_bytes,
        "segment_count": segment_count,
        "completed_at": completed_at,
    });
    let receipt_hash = blake3::hash(canonical.to_string().as_bytes())
        .to_hex()
        .to_string();

    Receipt {
        receipt_id,
        share_id: share_id.to_string(),
        manifest_id: manifest_id.to_string(),
        total_bytes,
        segment_count,
        completed_at,
        receipt_hash,
    }
}

/// Persist a receipt to disk.
pub async fn store_receipt(receipts_dir: &PathBuf, receipt: &Receipt) -> Result<(), AppError> {
    fs::create_dir_all(receipts_dir).await?;
    // Store by share_id so we can look up receipts for a given share.
    let path = receipts_dir.join(format!("{}.json", &receipt.share_id));
    let json = serde_json::to_string_pretty(receipt)?;
    fs::write(&path, json.as_bytes()).await?;
    tracing::info!(
        receipt_id = %receipt.receipt_id,
        share_id = %receipt.share_id,
        "receipt stored"
    );
    Ok(())
}

/// Load a receipt by share_id.
pub async fn load_receipt(receipts_dir: &PathBuf, share_id: &str) -> Result<Receipt, AppError> {
    let path = receipts_dir.join(format!("{share_id}.json"));
    if !path.exists() {
        return Err(AppError::NotFound(format!(
            "receipt for share {share_id}"
        )));
    }
    let data = fs::read_to_string(&path).await?;
    let receipt: Receipt = serde_json::from_str(&data)?;
    Ok(receipt)
}
