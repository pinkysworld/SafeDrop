//! Share lifecycle — Sprint 2.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub const SHARE_SCHEMA_VERSION: &str = "1";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecipientScope {
    Anonymous,
    OneTimeSecret,
    Named,
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
}
