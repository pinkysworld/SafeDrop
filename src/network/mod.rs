//! Network reachability engine — Sprint 3.

use serde::{Deserialize, Serialize};

/// How the recipient reached the host.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PathType {
    Direct,
    Mapped,
    Assisted,
    Relayed,
}

/// Signed statement describing the path used.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReachabilityDescriptor {
    pub path_type: PathType,
    pub external_addr: Option<String>,
    pub confirmed_at: Option<chrono::DateTime<chrono::Utc>>,
}
