//! Encrypted relay fallback skeleton — SD-010.
//!
//! When no direct or mapped path is available, SafeDrop can route
//! ciphertext-only traffic through a relay. The relay never sees plaintext.
//!
//! This module provides the relay channel abstraction and session management.
//! The actual relay server implementation is a future milestone.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A relay session represents a pending or active connection through a relay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelaySession {
    pub session_id: String,
    pub relay_endpoint: String,
    pub share_id: String,
    pub created_at: DateTime<Utc>,
    pub status: RelayStatus,
    /// BLAKE3 hash of session params for tamper detection.
    pub session_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RelayStatus {
    Pending,
    Active,
    Completed,
    Failed,
}

/// Create a new relay session (skeleton — returns a pending session).
pub fn create_relay_session(relay_endpoint: &str, share_id: &str) -> RelaySession {
    let session_id = uuid::Uuid::new_v4().to_string();
    let created_at = Utc::now();

    let canonical = serde_json::json!({
        "session_id": session_id,
        "relay_endpoint": relay_endpoint,
        "share_id": share_id,
        "created_at": created_at,
    });
    let session_hash = blake3::hash(canonical.to_string().as_bytes())
        .to_hex()
        .to_string();

    RelaySession {
        session_id,
        relay_endpoint: relay_endpoint.to_string(),
        share_id: share_id.to_string(),
        created_at,
        status: RelayStatus::Pending,
        session_hash,
    }
}

/// Relay channel info returned to recipients when relay path is selected.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayInfo {
    pub relay_endpoint: String,
    pub session_id: String,
    pub path_type: String,
    pub message: String,
}

/// Get relay info for a share (returns placeholder until relay server exists).
pub fn get_relay_info(share_id: &str) -> RelayInfo {
    RelayInfo {
        relay_endpoint: "relay://pending".to_string(),
        session_id: uuid::Uuid::new_v4().to_string(),
        path_type: "relayed".to_string(),
        message: format!(
            "Direct connection unavailable for share {share_id}. \
             Relay transport is not yet operational. \
             The relay will only carry ciphertext — it never sees file contents."
        ),
    }
}
