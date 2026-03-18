//! Share lifecycle: issuance, expiry, revocation, and persistence.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

use crate::error::AppError;

pub const SHARE_SCHEMA_VERSION: &str = "1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RecipientScope {
    Anonymous,
    OneTimeSecret,
    Named,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ShareStatus {
    Active,
    Expired,
    Revoked,
    Exhausted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Share {
    pub schema_version: String,
    pub share_id: String,
    pub manifest_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub recipient_scope: RecipientScope,
    pub path_preference: String,
    pub relay_allowed: bool,
    pub download_limit: u32,
    pub download_count: u32,
    pub status: ShareStatus,
    pub revoked_at: Option<DateTime<Utc>>,
    /// Capability token the recipient uses to access the share.
    pub token: String,
}

impl Share {
    /// Check whether this share is currently valid for download.
    pub fn is_accessible(&self) -> bool {
        if self.status == ShareStatus::Revoked {
            return false;
        }
        if Utc::now() > self.expires_at {
            return false;
        }
        if self.download_count >= self.download_limit {
            return false;
        }
        true
    }

    /// Return the effective status considering time and download count.
    pub fn effective_status(&self) -> ShareStatus {
        if self.status == ShareStatus::Revoked {
            return ShareStatus::Revoked;
        }
        if Utc::now() > self.expires_at {
            return ShareStatus::Expired;
        }
        if self.download_count >= self.download_limit {
            return ShareStatus::Exhausted;
        }
        ShareStatus::Active
    }
}

/// Filesystem-backed share store.
#[derive(Clone)]
pub struct ShareStore {
    dir: PathBuf,
}

impl ShareStore {
    pub async fn new(dir: PathBuf) -> Result<Self, AppError> {
        fs::create_dir_all(&dir).await?;
        Ok(Self { dir })
    }

    /// Issue a new share for a manifest. Returns the created share.
    pub async fn issue(
        &self,
        manifest_id: String,
        recipient_scope: RecipientScope,
        ttl_hours: u32,
        download_limit: u32,
        relay_allowed: bool,
    ) -> Result<Share, AppError> {
        let now = Utc::now();
        let share_id = uuid::Uuid::new_v4().to_string();
        let token = generate_token();

        let share = Share {
            schema_version: SHARE_SCHEMA_VERSION.to_string(),
            share_id,
            manifest_id,
            created_at: now,
            expires_at: now + Duration::hours(ttl_hours as i64),
            recipient_scope,
            path_preference: "direct_first".to_string(),
            relay_allowed,
            download_limit,
            download_count: 0,
            status: ShareStatus::Active,
            revoked_at: None,
            token,
        };

        self.save(&share).await?;
        tracing::info!(share_id = %share.share_id, manifest = %share.manifest_id, "share issued");
        Ok(share)
    }

    /// Revoke a share by ID.
    pub async fn revoke(&self, share_id: &str) -> Result<Share, AppError> {
        let mut share = self.load(share_id).await?;
        share.status = ShareStatus::Revoked;
        share.revoked_at = Some(Utc::now());
        self.save(&share).await?;
        tracing::info!(share_id = %share_id, "share revoked");
        Ok(share)
    }

    /// Increment the download count. Returns an error if the share is not accessible.
    pub async fn record_download(&self, share_id: &str) -> Result<Share, AppError> {
        let mut share = self.load(share_id).await?;
        if !share.is_accessible() {
            return Err(AppError::BadRequest(format!(
                "share {share_id} is {}",
                serde_json::to_string(&share.effective_status())
                    .unwrap_or_else(|_| "inaccessible".to_string())
            )));
        }
        share.download_count += 1;
        if share.download_count >= share.download_limit {
            share.status = ShareStatus::Exhausted;
        }
        self.save(&share).await?;
        Ok(share)
    }

    /// Look up a share by its recipient token.
    pub async fn find_by_token(&self, token: &str) -> Result<Share, AppError> {
        let mut entries = fs::read_dir(&self.dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                let data = fs::read_to_string(&path).await?;
                if let Ok(share) = serde_json::from_str::<Share>(&data) {
                    if share.token == token {
                        return Ok(share);
                    }
                }
            }
        }
        Err(AppError::NotFound("share not found".to_string()))
    }

    pub async fn load(&self, share_id: &str) -> Result<Share, AppError> {
        let path = self.dir.join(format!("{share_id}.json"));
        if !path.exists() {
            return Err(AppError::NotFound(format!("share {share_id}")));
        }
        let data = fs::read_to_string(&path).await?;
        let share: Share = serde_json::from_str(&data)?;
        Ok(share)
    }

    pub async fn list_all(&self) -> Result<Vec<Share>, AppError> {
        let mut shares = Vec::new();
        let mut entries = fs::read_dir(&self.dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                let data = fs::read_to_string(&path).await?;
                if let Ok(share) = serde_json::from_str::<Share>(&data) {
                    shares.push(share);
                }
            }
        }
        shares.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(shares)
    }

    async fn save(&self, share: &Share) -> Result<(), AppError> {
        let path = self.dir.join(format!("{}.json", &share.share_id));
        let json = serde_json::to_string_pretty(share)?;
        fs::write(&path, json.as_bytes()).await?;
        Ok(())
    }
}

/// Generate a URL-safe capability token (32 bytes, hex-encoded).
fn generate_token() -> String {
    uuid::Uuid::new_v4().to_string().replace('-', "")
        + &uuid::Uuid::new_v4().to_string().replace('-', "")
}
