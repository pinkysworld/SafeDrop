//! Integration tests for Sprint 3: Reachability engine.
//!
//! Run with: cargo test --test sprint3_test

use axum::routing::{get, post};
use axum::Router;
use safedrop::api::files::AppState;
use safedrop::evidence::audit::AuditLog;
use safedrop::share::ShareStore;
use safedrop::storage::chunk::ChunkStore;
use std::sync::Arc;
use tempfile::TempDir;

/// Helper: create a test app backed by a temporary directory.
async fn test_app() -> (String, TempDir) {
    let tmp = tempfile::tempdir().unwrap();
    let data_dir = tmp.path().to_path_buf();

    let config = safedrop::config::Config {
        port: 0,
        host: "127.0.0.1".to_string(),
        data_dir: data_dir.clone(),
        log_level: "error".to_string(),
    };

    let chunk_store = ChunkStore::new(config.chunks_dir()).await.unwrap();
    let audit_log = AuditLog::new(config.audit_dir()).await.unwrap();
    let share_store = ShareStore::new(config.shares_dir()).await.unwrap();

    let state = Arc::new(AppState {
        chunk_store,
        audit_log,
        share_store,
        config,
    });

    let app = Router::new()
        .route("/health", get(safedrop::api::health::health))
        .route(
            "/api/v1/reachability/check",
            post(safedrop::api::reachability::check_reachability),
        )
        .route(
            "/api/v1/reachability/status",
            get(safedrop::api::reachability::reachability_status),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = format!("http://{}", listener.local_addr().unwrap());

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    (addr, tmp)
}

// ── Reachability Check Tests ──

#[tokio::test]
async fn reachability_check_returns_descriptor() {
    let (addr, _tmp) = test_app().await;
    let client = reqwest::Client::new();

    let resp = client
        .post(format!("{addr}/api/v1/reachability/check"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200, "reachability check should succeed");
    let body: serde_json::Value = resp.json().await.unwrap();

    // Must have required fields
    assert!(body["descriptor_id"].is_string(), "must have descriptor_id");
    assert!(body["path_type"].is_string(), "must have path_type");
    assert!(body["internal_addr"].is_string(), "must have internal_addr");
    assert!(body["descriptor_hash"].is_string(), "must have descriptor_hash");

    // Path type must be a known variant
    let path_type = body["path_type"].as_str().unwrap();
    assert!(
        ["direct", "mapped", "assisted", "relayed"].contains(&path_type),
        "path_type must be a known variant, got: {path_type}"
    );

    // Steps array should be non-empty
    assert!(body["steps"].is_array(), "must have steps array");
    assert!(
        !body["steps"].as_array().unwrap().is_empty(),
        "steps should not be empty"
    );
}

#[tokio::test]
async fn reachability_status_404_before_check() {
    let (addr, _tmp) = test_app().await;
    let client = reqwest::Client::new();

    let resp = client
        .get(format!("{addr}/api/v1/reachability/status"))
        .send()
        .await
        .unwrap();

    // Should return 404 since no check has been performed yet
    assert_eq!(
        resp.status(),
        404,
        "status should be 404 before any check is performed"
    );
}

#[tokio::test]
async fn reachability_status_returns_descriptor_after_check() {
    let (addr, _tmp) = test_app().await;
    let client = reqwest::Client::new();

    // First, run a check to create a descriptor
    let check_resp = client
        .post(format!("{addr}/api/v1/reachability/check"))
        .send()
        .await
        .unwrap();
    assert_eq!(check_resp.status(), 200);
    let check_body: serde_json::Value = check_resp.json().await.unwrap();
    let check_id = check_body["descriptor_id"].as_str().unwrap().to_string();

    // Now status should return the stored descriptor
    let status_resp = client
        .get(format!("{addr}/api/v1/reachability/status"))
        .send()
        .await
        .unwrap();
    assert_eq!(status_resp.status(), 200, "status should succeed after check");
    let status_body: serde_json::Value = status_resp.json().await.unwrap();

    assert_eq!(
        status_body["descriptor_id"].as_str().unwrap(),
        check_id,
        "status descriptor_id must match the check result"
    );
    assert_eq!(
        status_body["descriptor_hash"].as_str().unwrap(),
        check_body["descriptor_hash"].as_str().unwrap(),
        "descriptor hash must be consistent"
    );
}

#[tokio::test]
async fn reachability_check_steps_contain_direct_bind() {
    let (addr, _tmp) = test_app().await;
    let client = reqwest::Client::new();

    let resp = client
        .post(format!("{addr}/api/v1/reachability/check"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();

    // The first step should always be a direct bind attempt
    let steps = body["steps"].as_array().unwrap();
    let first = &steps[0];
    assert_eq!(
        first["method"].as_str().unwrap(),
        "direct_bind",
        "first step must be direct_bind"
    );
}

#[tokio::test]
async fn reachability_descriptor_hash_is_blake3() {
    let (addr, _tmp) = test_app().await;
    let client = reqwest::Client::new();

    let resp = client
        .post(format!("{addr}/api/v1/reachability/check"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();

    let hash = body["descriptor_hash"].as_str().unwrap();
    // BLAKE3 produces a 64-char hex string (256 bits)
    assert_eq!(hash.len(), 64, "BLAKE3 hash must be 64 hex chars");
    assert!(
        hash.chars().all(|c| c.is_ascii_hexdigit()),
        "hash must be valid hex"
    );
}

// ── Unit-level: Descriptor persistence ──

#[tokio::test]
async fn store_and_load_descriptor() {
    let tmp = tempfile::tempdir().unwrap();
    let network_dir = tmp.path().join("network");

    let desc = safedrop::network::ReachabilityDescriptor {
        descriptor_id: "test-desc-001".to_string(),
        path_type: safedrop::network::PathType::Direct,
        external_addr: Some("1.2.3.4:9090".to_string()),
        internal_addr: "127.0.0.1:9090".to_string(),
        confirmed_at: Some(chrono::Utc::now()),
        probe_result: Some("ok".to_string()),
        relay_endpoint: None,
        descriptor_hash: "a".repeat(64),
    };

    safedrop::network::store_descriptor(&network_dir, &desc)
        .await
        .expect("store should succeed");

    let loaded = safedrop::network::load_latest_descriptor(&network_dir)
        .await
        .expect("load should succeed");

    assert_eq!(loaded.descriptor_id, desc.descriptor_id);
    assert_eq!(loaded.path_type, desc.path_type);
    assert_eq!(loaded.external_addr, desc.external_addr);
    assert_eq!(loaded.descriptor_hash, desc.descriptor_hash);
}

#[tokio::test]
async fn load_latest_picks_most_recent() {
    let tmp = tempfile::tempdir().unwrap();
    let network_dir = tmp.path().join("network");

    let now = chrono::Utc::now();

    let old = safedrop::network::ReachabilityDescriptor {
        descriptor_id: "old-desc".to_string(),
        path_type: safedrop::network::PathType::Relayed,
        external_addr: None,
        internal_addr: "127.0.0.1:9090".to_string(),
        confirmed_at: Some(now - chrono::Duration::hours(1)),
        probe_result: None,
        relay_endpoint: Some("relay://old".to_string()),
        descriptor_hash: "b".repeat(64),
    };

    let new = safedrop::network::ReachabilityDescriptor {
        descriptor_id: "new-desc".to_string(),
        path_type: safedrop::network::PathType::Direct,
        external_addr: Some("5.6.7.8:9090".to_string()),
        internal_addr: "127.0.0.1:9090".to_string(),
        confirmed_at: Some(now),
        probe_result: Some("probe_ok".to_string()),
        relay_endpoint: None,
        descriptor_hash: "c".repeat(64),
    };

    safedrop::network::store_descriptor(&network_dir, &old)
        .await
        .unwrap();
    safedrop::network::store_descriptor(&network_dir, &new)
        .await
        .unwrap();

    let latest = safedrop::network::load_latest_descriptor(&network_dir)
        .await
        .unwrap();

    assert_eq!(latest.descriptor_id, "new-desc", "should return most recent");
}
