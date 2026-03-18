//! External reachability probe — SD-009.
//!
//! Verifies that an address is actually reachable from the outside
//! by attempting a TCP connection back to ourselves.

use super::ReachabilityStep;
use chrono::Utc;

/// Attempt to verify external reachability by connecting to our own address.
/// In a production deployment this would call an external probe service;
/// for now we do a self-connect test as a reasonable local check.
pub async fn check_external_probe(external_addr: &str) -> ReachabilityStep {
    match tokio::time::timeout(
        std::time::Duration::from_secs(5),
        tokio::net::TcpStream::connect(external_addr),
    )
    .await
    {
        Ok(Ok(_stream)) => ReachabilityStep {
            method: "external_probe".to_string(),
            success: true,
            detail: format!("TCP connect to {external_addr} succeeded"),
            timestamp: Utc::now(),
        },
        Ok(Err(e)) => ReachabilityStep {
            method: "external_probe".to_string(),
            success: false,
            detail: format!("TCP connect to {external_addr} failed: {e}"),
            timestamp: Utc::now(),
        },
        Err(_) => ReachabilityStep {
            method: "external_probe".to_string(),
            success: false,
            detail: format!("TCP connect to {external_addr} timed out after 5s"),
            timestamp: Utc::now(),
        },
    }
}
