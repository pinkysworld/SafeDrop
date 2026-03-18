use axum::routing::{get, post};
use axum::Router;
use clap::Parser;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use safedrop::api::files::AppState;
use safedrop::config::Config;
use safedrop::evidence::audit::AuditLog;
use safedrop::share::ShareStore;
use safedrop::storage::chunk::ChunkStore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| config.log_level.clone().into()),
        )
        .init();

    tracing::info!(
        port = config.port,
        data_dir = %config.data_dir.display(),
        "starting SafeDrop server"
    );

    // Initialize subsystems
    let chunk_store = ChunkStore::new(config.chunks_dir()).await?;
    let audit_log = AuditLog::new(config.audit_dir()).await?;
    let share_store = ShareStore::new(config.shares_dir()).await?;

    let state = Arc::new(AppState {
        chunk_store,
        audit_log,
        share_store,
        config: config.clone(),
    });

    let app = Router::new()
        // Health
        .route("/health", get(safedrop::api::health::health))
        // Storage (Sprint 1)
        .route("/api/v1/files/import", post(safedrop::api::files::import_file))
        .route("/api/v1/audit/events", get(safedrop::api::audit_api::get_events))
        // Shares (Sprint 2)
        .route("/api/v1/shares", post(safedrop::api::shares::create_share))
        .route("/api/v1/shares", get(safedrop::api::shares::list_shares))
        .route("/api/v1/shares/{share_id}", get(safedrop::api::shares::get_share))
        .route("/api/v1/shares/{share_id}/expire", post(safedrop::api::shares::expire_share))
        .route("/api/v1/shares/{share_id}/resume", post(safedrop::api::download::resume_info))
        .route("/api/v1/shares/{share_id}/ack", post(safedrop::api::download::ack_segment))
        // Download (Sprint 2)
        .route("/s/{token}", get(safedrop::api::download::download_by_token))
        // Evidence (Sprint 2)
        .route("/api/v1/evidence/{share_id}", get(safedrop::api::download::get_evidence))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(addr = %addr, "SafeDrop listening");

    axum::serve(listener, app).await?;

    Ok(())
}
