//! API Gateway

use axum::{body::Body, http::{Request, StatusCode}, response::{IntoResponse, Json, Response}, routing::get, Router};
use serde::Serialize;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::EnvFilter;
use std::net::SocketAddr;

#[derive(Serialize)]
struct GatewayHealth { status: String, version: String }

async fn health() -> Json<GatewayHealth> {
    Json(GatewayHealth { status: "ok".into(), version: env!("CARGO_PKG_VERSION").into() })
}

async fn proxy(req: Request<Body>) -> Result<Response, StatusCode> {
    let path = req.uri().path().to_string();
    let query = req.uri().query().map(|q| format!("?{q}")).unwrap_or_default();
    let upstream = format!("http://localhost:8081{path}{query}");
    let client = reqwest::Client::new();
    let method = req.method().clone();
    let body = match axum::body::to_bytes(req.into_body(), 10 * 1024 * 1024).await {
        Ok(b) => b, Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    match client.request(method, &upstream).body(body.to_vec()).send().await {
        Ok(r) => { let s = StatusCode::from_u16(r.status().as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR); let b = r.bytes().await.unwrap_or_default(); Ok((s, b).into_response()) }
        Err(_) => Err(StatusCode::BAD_GATEWAY),
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env().add_directive("info".parse().unwrap())).init();
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);
    let app = Router::new().route("/health", get(health)).fallback(proxy).layer(cors);
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Gateway listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
