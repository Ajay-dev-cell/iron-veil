use axum::{
    extract::State,
    routing::get,
    Router,
    Json,
};
use serde_json::{json, Value};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tower_http::cors::CorsLayer;
use crate::state::AppState;
use std::sync::atomic::Ordering;

pub async fn start_api_server(port: u16, state: AppState) {
    // Define the routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/rules", get(get_rules))
        .route("/connections", get(get_connections))
        .route("/schema", get(get_schema))
        .route("/logs", get(get_logs))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Management API listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "db-proxy",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

async fn get_rules(State(state): State<AppState>) -> Json<Value> {
    Json(json!(*state.config))
}

async fn get_connections(State(state): State<AppState>) -> Json<Value> {
    let count = state.active_connections.load(Ordering::Relaxed);
    Json(json!({
        "active_connections": count
    }))
}

async fn get_schema() -> Json<Value> {
    Json(json!({
        "tables": [],
        "note": "Schema discovery requires upstream connection. Coming in Phase 3.4"
    }))
}

async fn get_logs() -> Json<Value> {
    Json(json!({
        "logs": [],
        "note": "In-memory log buffer coming in Phase 4.2"
    }))
}
