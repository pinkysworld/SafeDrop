//! Append-only audit event log with BLAKE3 checkpoint chaining.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs::{self, OpenOptions};
use tokio::io::AsyncWriteExt;

use crate::error::AppError;

/// Audit event types matching the data model.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    ManifestCommitted,
    ShareIssued,
    ReachabilityConfirmed,
    DownloadStarted,
    SegmentAcknowledged,
    DownloadCompleted,
    ShareExpired,
    ObjectDeleted,
}

/// A single audit event in the append-only log.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub event_id: String,
    pub kind: EventKind,
    pub timestamp: DateTime<Utc>,
    pub subject_id: String,
    pub detail: serde_json::Value,
    /// BLAKE3 hash of the previous event (chain integrity).
    pub prev_hash: String,
    /// BLAKE3 hash of this event's canonical serialization.
    pub event_hash: String,
}

/// Append-only audit log.
#[derive(Clone)]
pub struct AuditLog {
    dir: PathBuf,
}

impl AuditLog {
    pub async fn new(dir: PathBuf) -> Result<Self, AppError> {
        fs::create_dir_all(&dir).await?;
        Ok(Self { dir })
    }

    fn log_path(&self) -> PathBuf {
        self.dir.join("audit.jsonl")
    }

    /// Get the hash of the last event in the log (or a zero hash if empty).
    pub async fn last_hash(&self) -> Result<String, AppError> {
        let path = self.log_path();
        if !path.exists() {
            return Ok("0".repeat(64));
        }

        let content = fs::read_to_string(&path).await?;
        let last_line = content.lines().rev().find(|l| !l.trim().is_empty());

        match last_line {
            Some(line) => {
                let event: AuditEvent = serde_json::from_str(line)?;
                Ok(event.event_hash)
            }
            None => Ok("0".repeat(64)),
        }
    }

    /// Append an event to the log. Returns the event with computed hashes.
    pub async fn append(
        &self,
        kind: EventKind,
        subject_id: String,
        detail: serde_json::Value,
    ) -> Result<AuditEvent, AppError> {
        let prev_hash = self.last_hash().await?;
        let event_id = uuid::Uuid::new_v4().to_string();
        let timestamp = Utc::now();

        // Canonical form for hashing: deterministic JSON
        let canonical = serde_json::json!({
            "event_id": event_id,
            "kind": kind,
            "timestamp": timestamp,
            "subject_id": subject_id,
            "detail": detail,
            "prev_hash": prev_hash,
        });
        let event_hash = blake3::hash(canonical.to_string().as_bytes())
            .to_hex()
            .to_string();

        let event = AuditEvent {
            event_id,
            kind,
            timestamp,
            subject_id,
            detail,
            prev_hash,
            event_hash,
        };

        // Append to JSONL file
        let line = serde_json::to_string(&event)? + "\n";
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.log_path())
            .await?;
        file.write_all(line.as_bytes()).await?;
        file.flush().await?;
        file.sync_all().await?;

        tracing::info!(
            event_id = %event.event_id,
            kind = ?event.kind,
            subject = %event.subject_id,
            "audit event appended"
        );

        Ok(event)
    }

    /// Read all events from the log.
    pub async fn read_all(&self) -> Result<Vec<AuditEvent>, AppError> {
        let path = self.log_path();
        if !path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&path).await?;
        let mut events = Vec::new();
        for line in content.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let event: AuditEvent = serde_json::from_str(line)?;
            events.push(event);
        }
        Ok(events)
    }

    /// Verify the integrity chain of the audit log.
    pub async fn verify_chain(&self) -> Result<bool, AppError> {
        let events = self.read_all().await?;
        let mut expected_prev = "0".repeat(64);

        for event in &events {
            if event.prev_hash != expected_prev {
                tracing::warn!(
                    event_id = %event.event_id,
                    expected = %expected_prev,
                    actual = %event.prev_hash,
                    "audit chain integrity failure"
                );
                return Ok(false);
            }
            expected_prev = event.event_hash.clone();
        }
        Ok(true)
    }
}
