//! Integration test: health endpoint.
//!
//! Run with: cargo test --test health_test

use axum::routing::get;
use axum::Router;

#[tokio::test]
async fn health_endpoint_returns_ok() {
    let app = Router::new()
        .route("/health", get(safedrop::api::health::health));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let resp = reqwest::get(format!("http://{addr}/health"))
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["status"], "ok");
    assert_eq!(body["service"], "safedrop");
}
