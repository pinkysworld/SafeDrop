//! Integration tests for Sprint 2: shares, downloads, receipts.
//!
//! Run with: cargo test --test sprint2_test

use axum::routing::{get, post};
use axum::Router;
use safedrop::api::files::AppState;
use safedrop::evidence::audit::AuditLog;
use safedrop::share::ShareStore;
use safedrop::storage::chunk::ChunkStore;
use std::sync::Arc;
use tempfile::TempDir;

/// Helper: create a test app backed by a temporary directory.
async fn test_app() -> (Router, String, TempDir) {
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
        .route("/api/v1/files/import", post(safedrop::api::files::import_file))
        .route("/api/v1/audit/events", get(safedrop::api::audit_api::get_events))
        .route("/api/v1/shares", post(safedrop::api::shares::create_share))
        .route("/api/v1/shares", get(safedrop::api::shares::list_shares))
        .route("/api/v1/shares/{share_id}", get(safedrop::api::shares::get_share))
        .route("/api/v1/shares/{share_id}/expire", post(safedrop::api::shares::expire_share))
        .route("/api/v1/shares/{share_id}/resume", post(safedrop::api::download::resume_info))
        .route("/api/v1/shares/{share_id}/ack", post(safedrop::api::download::ack_segment))
        .route("/s/{token}", get(safedrop::api::download::download_by_token))
        .route("/api/v1/evidence/{share_id}", get(safedrop::api::download::get_evidence))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = format!("http://{}", listener.local_addr().unwrap());

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    (Router::new(), addr, tmp)
}

/// Helper: import a test file and return its manifest_id.
async fn import_test_file(addr: &str, tmp: &TempDir) -> String {
    // Create a small test file
    let test_file = tmp.path().join("testfile.bin");
    std::fs::write(&test_file, b"Hello, SafeDrop! This is a test file for Sprint 2.").unwrap();

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{addr}/api/v1/files/import"))
        .json(&serde_json::json!({
            "file_path": test_file.to_str().unwrap(),
            "file_name": "testfile.bin",
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200, "import should succeed");
    let body: serde_json::Value = resp.json().await.unwrap();
    body["manifest_id"].as_str().unwrap().to_string()
}

// ── Share Lifecycle Tests ──

#[tokio::test]
async fn create_and_get_share() {
    let (_app, addr, tmp) = test_app().await;
    let manifest_id = import_test_file(&addr, &tmp).await;

    let client = reqwest::Client::new();

    // Create share
    let resp = client
        .post(format!("{addr}/api/v1/shares"))
        .json(&serde_json::json!({
            "manifest_id": manifest_id,
            "ttl_hours": 1,
            "download_limit": 3,
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let share: serde_json::Value = resp.json().await.unwrap();
    let share_id = share["share_id"].as_str().unwrap();
    let token = share["token"].as_str().unwrap();
    assert!(!share_id.is_empty());
    assert!(!token.is_empty());
    assert_eq!(share["status"], "active");

    // Get share metadata
    let resp = client
        .get(format!("{addr}/api/v1/shares/{share_id}"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let meta: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(meta["manifest_id"], manifest_id);
    assert_eq!(meta["file_name"], "testfile.bin");
    assert_eq!(meta["download_count"], 0);
    assert_eq!(meta["download_limit"], 3);
    assert_eq!(meta["status"], "active");
}

#[tokio::test]
async fn list_shares() {
    let (_app, addr, tmp) = test_app().await;
    let manifest_id = import_test_file(&addr, &tmp).await;

    let client = reqwest::Client::new();

    // Create two shares
    for _ in 0..2 {
        client
            .post(format!("{addr}/api/v1/shares"))
            .json(&serde_json::json!({"manifest_id": manifest_id}))
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .get(format!("{addr}/api/v1/shares"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let shares: Vec<serde_json::Value> = resp.json().await.unwrap();
    assert_eq!(shares.len(), 2);
}

#[tokio::test]
async fn expire_share() {
    let (_app, addr, tmp) = test_app().await;
    let manifest_id = import_test_file(&addr, &tmp).await;

    let client = reqwest::Client::new();

    let resp = client
        .post(format!("{addr}/api/v1/shares"))
        .json(&serde_json::json!({"manifest_id": manifest_id}))
        .send()
        .await
        .unwrap();
    let share: serde_json::Value = resp.json().await.unwrap();
    let share_id = share["share_id"].as_str().unwrap();
    let token = share["token"].as_str().unwrap().to_string();

    // Expire it
    let resp = client
        .post(format!("{addr}/api/v1/shares/{share_id}/expire"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let expired: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(expired["status"], "revoked");

    // Download should now fail
    let resp = client
        .get(format!("{addr}/s/{token}"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 400);
}

// ── Download Tests ──

#[tokio::test]
async fn full_download_with_receipt() {
    let (_app, addr, tmp) = test_app().await;
    let manifest_id = import_test_file(&addr, &tmp).await;

    let client = reqwest::Client::new();

    let resp = client
        .post(format!("{addr}/api/v1/shares"))
        .json(&serde_json::json!({
            "manifest_id": manifest_id,
            "download_limit": 2,
        }))
        .send()
        .await
        .unwrap();
    let share: serde_json::Value = resp.json().await.unwrap();
    let share_id = share["share_id"].as_str().unwrap().to_string();
    let token = share["token"].as_str().unwrap();

    // Download
    let resp = client
        .get(format!("{addr}/s/{token}"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    let receipt_id = resp
        .headers()
        .get("x-safedrop-receipt")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    assert!(!receipt_id.is_empty());

    let body = resp.bytes().await.unwrap();
    assert_eq!(
        &body[..],
        b"Hello, SafeDrop! This is a test file for Sprint 2."
    );

    // Check evidence receipt
    let resp = client
        .get(format!("{addr}/api/v1/evidence/{share_id}"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let receipt: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(receipt["share_id"], share_id);
    assert_eq!(receipt["manifest_id"], manifest_id);
    assert!(!receipt["hash"].as_str().unwrap().is_empty());
}

#[tokio::test]
async fn download_limit_enforced() {
    let (_app, addr, tmp) = test_app().await;
    let manifest_id = import_test_file(&addr, &tmp).await;

    let client = reqwest::Client::new();

    let resp = client
        .post(format!("{addr}/api/v1/shares"))
        .json(&serde_json::json!({
            "manifest_id": manifest_id,
            "download_limit": 1,
        }))
        .send()
        .await
        .unwrap();
    let share: serde_json::Value = resp.json().await.unwrap();
    let token = share["token"].as_str().unwrap();

    // First download succeeds
    let resp = client
        .get(format!("{addr}/s/{token}"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    // Second download should fail (exhausted)
    let resp = client
        .get(format!("{addr}/s/{token}"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 400);
}

// ── Resume Info & Segment Ack Tests ──

#[tokio::test]
async fn resume_info_and_segment_ack() {
    let (_app, addr, tmp) = test_app().await;
    let manifest_id = import_test_file(&addr, &tmp).await;

    let client = reqwest::Client::new();

    let resp = client
        .post(format!("{addr}/api/v1/shares"))
        .json(&serde_json::json!({
            "manifest_id": manifest_id,
            "download_limit": 5,
        }))
        .send()
        .await
        .unwrap();
    let share: serde_json::Value = resp.json().await.unwrap();
    let share_id = share["share_id"].as_str().unwrap();
    let token = share["token"].as_str().unwrap();

    // Get resume info
    let resp = client
        .post(format!("{addr}/api/v1/shares/{share_id}/resume"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let info: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(info["manifest_id"], manifest_id);
    assert!(info["total_segments"].as_u64().unwrap() >= 1);
    assert_eq!(info["file_name"], "testfile.bin");

    // Download segment 0
    let resp = client
        .get(format!("{addr}/s/{token}?segment=0"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let chunk_hash = resp
        .headers()
        .get("x-safedrop-chunk-hash")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    // Ack segment 0
    let resp = client
        .post(format!("{addr}/api/v1/shares/{share_id}/ack"))
        .json(&serde_json::json!({
            "segment": 0,
            "chunk_hash": chunk_hash,
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let ack: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(ack["acknowledged"], true);
    assert_eq!(ack["segment"], 0);
}

// ── Audit Trail Tests ──

#[tokio::test]
async fn audit_trail_records_share_events() {
    let (_app, addr, tmp) = test_app().await;
    let manifest_id = import_test_file(&addr, &tmp).await;

    let client = reqwest::Client::new();

    // Create share
    client
        .post(format!("{addr}/api/v1/shares"))
        .json(&serde_json::json!({"manifest_id": manifest_id}))
        .send()
        .await
        .unwrap();

    // Check audit events
    let resp = client
        .get(format!("{addr}/api/v1/audit/events"))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    let events = body["events"].as_array().unwrap();

    // Should have at least manifest_committed + share_issued
    let kinds: Vec<&str> = events.iter().map(|e| e["kind"].as_str().unwrap()).collect();
    assert!(kinds.contains(&"manifest_committed"), "expected manifest_committed in {kinds:?}");
    assert!(kinds.contains(&"share_issued"), "expected share_issued in {kinds:?}");
}

#[tokio::test]
async fn create_share_for_missing_manifest_fails() {
    let (_app, addr, _tmp) = test_app().await;

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{addr}/api/v1/shares"))
        .json(&serde_json::json!({
            "manifest_id": "nonexistent-manifest-id-12345",
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 404);
}
