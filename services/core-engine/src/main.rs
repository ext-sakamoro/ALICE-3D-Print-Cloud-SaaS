//! Core Engine

use axum::{extract::State, http::StatusCode, response::Json, routing::{get, post}, Router};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc, time::Instant};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
struct AppState { start_time: Instant }

#[derive(Debug, Serialize)]
struct HealthResponse { status: String, version: String, uptime_seconds: f64 }

#[derive(Debug, Deserialize)]
struct ProcessRequest { data: String, #[serde(default)] options: serde_json::Value }

#[derive(Debug, Serialize)]
struct ProcessResponse { result: String, latency_ms: f64 }

async fn health(State(s): State<Arc<AppState>>) -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok".into(), version: env!("CARGO_PKG_VERSION").into(), uptime_seconds: s.start_time.elapsed().as_secs_f64() })
}

async fn process(Json(req): Json<ProcessRequest>) -> Result<Json<ProcessResponse>, StatusCode> {
    let start = Instant::now();
    let result = format!("[stub] {}", &req.data[..req.data.len().min(100)]);
    Ok(Json(ProcessResponse { result, latency_ms: start.elapsed().as_secs_f64() * 1000.0 }))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env().add_directive("info".parse().unwrap())).init();
    let state = Arc::new(AppState { start_time: Instant::now() });
    let app = Router::new().route("/health", get(health)).route("/api/v1/process", post(process)).with_state(state);
    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    info!("Engine listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
