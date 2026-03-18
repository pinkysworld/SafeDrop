//! Network reachability engine — Sprint 3.
//!
//! Progressive fallback: Direct → UPnP/NAT-PMP → External probe → Relay.

pub mod probe;
pub mod relay;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::fs;
use tokio::net::TcpListener;

use crate::error::AppError;

/// How the recipient can reach the host.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PathType {
    /// Server is directly reachable (public IP or already port-forwarded).
    Direct,
    /// Port mapping via UPnP or NAT-PMP succeeded.
    Mapped,
    /// External probe confirmed reachability but mapping method unknown.
    Assisted,
    /// No direct path; traffic must go through an encrypted relay.
    Relayed,
}

/// Signed statement describing which network path is available.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReachabilityDescriptor {
    pub descriptor_id: String,
    pub path_type: PathType,
    pub external_addr: Option<String>,
    pub internal_addr: String,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub probe_result: Option<String>,
    pub relay_endpoint: Option<String>,
    /// BLAKE3 hash of the descriptor's canonical fields.
    pub descriptor_hash: String,
}

/// Result of a reachability check attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReachabilityResult {
    pub descriptor: ReachabilityDescriptor,
    pub steps: Vec<ReachabilityStep>,
}

/// A single step in the reachability determination process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReachabilityStep {
    pub method: String,
    pub success: bool,
    pub detail: String,
    pub timestamp: DateTime<Utc>,
}

impl ReachabilityStep {
    pub fn new(method: &str, success: bool, detail: &str) -> Self {
        Self {
            method: method.to_string(),
            success,
            detail: detail.to_string(),
            timestamp: Utc::now(),
        }
    }
}

/// Run the full reachability check sequence.
/// Returns a descriptor describing the best available path.
pub async fn check_reachability(
    listen_port: u16,
    listen_host: &str,
) -> ReachabilityResult {
    let mut steps = Vec::new();
    let internal_addr = format!("{listen_host}:{listen_port}");

    // Step 1: Check if we can bind and are potentially directly reachable
    let direct = check_direct_bind(listen_port, listen_host).await;
    steps.push(direct.clone());

    // Step 2: Attempt UPnP/NAT-PMP mapping
    let (mapped_addr, upnp_step) = attempt_upnp_mapping(listen_port).await;
    steps.push(upnp_step.clone());

    // Step 3: External probe (if we have an external address)
    let external_addr = mapped_addr.clone();
    if let Some(ref addr) = external_addr {
        let probe_step = probe::check_external_probe(addr).await;
        steps.push(probe_step.clone());

        if probe_step.success {
            let descriptor = build_descriptor(
                if upnp_step.success { PathType::Mapped } else { PathType::Direct },
                Some(addr.clone()),
                &internal_addr,
                Some("probe_confirmed".to_string()),
                None,
            );
            return ReachabilityResult { descriptor, steps };
        }
    }

    // Step 4: If direct bind worked but no external confirmation, report as direct (best-effort)
    if direct.success {
        let descriptor = build_descriptor(
            PathType::Direct,
            external_addr,
            &internal_addr,
            Some("direct_bind_only".to_string()),
            None,
        );
        return ReachabilityResult { descriptor, steps };
    }

    // Step 5: Fall back to relay
    let relay_step = ReachabilityStep::new(
        "relay_fallback",
        true,
        "No direct or mapped path available; relay required",
    );
    steps.push(relay_step);

    let descriptor = build_descriptor(
        PathType::Relayed,
        None,
        &internal_addr,
        None,
        Some("relay://pending".to_string()),
    );
    ReachabilityResult { descriptor, steps }
}

/// Check if we can bind to the listen address (basic direct reachability test).
async fn check_direct_bind(port: u16, host: &str) -> ReachabilityStep {
    let addr = format!("{host}:{port}");
    match TcpListener::bind(&addr).await {
        Ok(listener) => {
            let local = listener.local_addr().unwrap_or_else(|_| {
                SocketAddr::from(([127, 0, 0, 1], port))
            });
            drop(listener);
            ReachabilityStep::new(
                "direct_bind",
                true,
                &format!("Bound successfully to {local}"),
            )
        }
        Err(e) => ReachabilityStep::new(
            "direct_bind",
            false,
            &format!("Bind failed: {e}"),
        ),
    }
}

/// Attempt UPnP/NAT-PMP port mapping via IGD.
async fn attempt_upnp_mapping(port: u16) -> (Option<String>, ReachabilityStep) {
    match tokio::task::spawn_blocking(move || try_igd_mapping(port)).await {
        Ok(Ok(external)) => {
            let addr = format!("{external}:{port}");
            let step = ReachabilityStep::new(
                "upnp_mapping",
                true,
                &format!("UPnP mapped to {addr}"),
            );
            (Some(addr), step)
        }
        Ok(Err(e)) => {
            let step = ReachabilityStep::new(
                "upnp_mapping",
                false,
                &format!("UPnP/NAT-PMP failed: {e}"),
            );
            (None, step)
        }
        Err(e) => {
            let step = ReachabilityStep::new(
                "upnp_mapping",
                false,
                &format!("UPnP task join error: {e}"),
            );
            (None, step)
        }
    }
}

/// Synchronous IGD gateway discovery and port mapping.
fn try_igd_mapping(port: u16) -> Result<std::net::IpAddr, String> {
    use igd_next::SearchOptions;
    use std::time::Duration;

    let opts = SearchOptions {
        timeout: Some(Duration::from_secs(3)),
        ..Default::default()
    };

    let gateway = igd_next::search_gateway(opts).map_err(|e| format!("{e}"))?;

    let external_ip = gateway.get_external_ip().map_err(|e| format!("{e}"))?;

    let local_ip = get_local_ip().ok_or_else(|| "cannot determine local IP".to_string())?;
    let local_addr = std::net::SocketAddr::from((local_ip, port));

    gateway
        .add_port(
            igd_next::PortMappingProtocol::TCP,
            port,
            local_addr,
            300, // 5-minute lease
            "SafeDrop",
        )
        .map_err(|e| format!("{e}"))?;

    tracing::info!(%external_ip, port, "UPnP port mapping successful");
    Ok(external_ip)
}

/// Get the local (non-loopback) IPv4 address.
fn get_local_ip() -> Option<std::net::Ipv4Addr> {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:53").ok()?;
    match socket.local_addr().ok()? {
        std::net::SocketAddr::V4(addr) => Some(*addr.ip()),
        _ => None,
    }
}

/// Build and hash a reachability descriptor.
fn build_descriptor(
    path_type: PathType,
    external_addr: Option<String>,
    internal_addr: &str,
    probe_result: Option<String>,
    relay_endpoint: Option<String>,
) -> ReachabilityDescriptor {
    let descriptor_id = uuid::Uuid::new_v4().to_string();
    let confirmed_at = Some(Utc::now());

    let canonical = serde_json::json!({
        "descriptor_id": descriptor_id,
        "path_type": path_type,
        "external_addr": external_addr,
        "internal_addr": internal_addr,
        "confirmed_at": confirmed_at,
        "probe_result": probe_result,
        "relay_endpoint": relay_endpoint,
    });
    let descriptor_hash = blake3::hash(canonical.to_string().as_bytes())
        .to_hex()
        .to_string();

    ReachabilityDescriptor {
        descriptor_id,
        path_type,
        external_addr,
        internal_addr: internal_addr.to_string(),
        confirmed_at,
        probe_result,
        relay_endpoint,
        descriptor_hash,
    }
}

/// Persist a reachability descriptor to the network directory.
pub async fn store_descriptor(
    dir: &PathBuf,
    desc: &ReachabilityDescriptor,
) -> Result<(), AppError> {
    fs::create_dir_all(dir).await?;
    let path = dir.join(format!("{}.json", &desc.descriptor_id));
    let json = serde_json::to_string_pretty(desc)?;
    fs::write(&path, json.as_bytes()).await?;
    tracing::info!(descriptor_id = %desc.descriptor_id, path_type = ?desc.path_type, "descriptor stored");
    Ok(())
}

/// Load the most recent reachability descriptor.
pub async fn load_latest_descriptor(dir: &PathBuf) -> Result<ReachabilityDescriptor, AppError> {
    if !dir.exists() {
        return Err(AppError::NotFound("no reachability data".to_string()));
    }
    let mut entries = fs::read_dir(dir).await?;
    let mut latest: Option<ReachabilityDescriptor> = None;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            let data = fs::read_to_string(&path).await?;
            if let Ok(desc) = serde_json::from_str::<ReachabilityDescriptor>(&data) {
                match &latest {
                    None => latest = Some(desc),
                    Some(prev) => {
                        if desc.confirmed_at > prev.confirmed_at {
                            latest = Some(desc);
                        }
                    }
                }
            }
        }
    }

    latest.ok_or_else(|| AppError::NotFound("no reachability descriptor found".to_string()))
}
