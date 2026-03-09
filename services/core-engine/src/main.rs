//! ALICE 3D Print Cloud — Core Engine
//!
//! SDF → View → Physics → Print パイプライン

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Instant,
};
use tracing::info;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct AppState {
    start_time: Instant,
    models: Arc<Mutex<HashMap<String, SdfModel>>>,
    jobs: Arc<Mutex<Vec<PrintJob>>>,
}

#[derive(Clone, Debug, Serialize)]
struct SdfModel {
    id: String,
    name: String,
    sdf_expression: String,
    uploaded_at: String,
}

#[derive(Clone, Debug, Serialize)]
struct PrintJob {
    id: String,
    model_id: String,
    status: String,
    gcode_lines: usize,
    created_at: String,
}

// ---------------------------------------------------------------------------
// Request / Response types
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    uptime_seconds: f64,
}

#[derive(Debug, Deserialize)]
struct UploadRequest {
    name: String,
    sdf_expression: String,
}

#[derive(Debug, Serialize)]
struct UploadResponse {
    id: String,
    name: String,
    message: String,
}

#[derive(Debug, Deserialize)]
struct PreviewRequest {
    model_id: String,
    #[serde(default = "default_resolution")]
    resolution: u32,
}
fn default_resolution() -> u32 { 256 }

#[derive(Debug, Serialize)]
struct PreviewResponse {
    model_id: String,
    resolution: u32,
    vertices: usize,
    triangles: usize,
    preview_url: String,
}

#[derive(Debug, Deserialize)]
struct ValidateRequest {
    model_id: String,
    #[serde(default = "default_material")]
    material: String,
}
fn default_material() -> String { "PLA".into() }

#[derive(Debug, Serialize)]
struct ValidateResponse {
    model_id: String,
    material: String,
    max_stress_mpa: f64,
    yield_strength_mpa: f64,
    safety_factor: f64,
    passed: bool,
}

#[derive(Debug, Deserialize)]
struct SliceRequest {
    model_id: String,
    #[serde(default = "default_layer_height")]
    layer_height_mm: f64,
    #[serde(default = "default_infill")]
    infill_percent: u8,
}
fn default_layer_height() -> f64 { 0.2 }
fn default_infill() -> u8 { 20 }

#[derive(Debug, Serialize)]
struct SliceResponse {
    job_id: String,
    model_id: String,
    layers: usize,
    gcode_lines: usize,
    estimated_minutes: f64,
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

async fn health(State(s): State<Arc<AppState>>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        uptime_seconds: s.start_time.elapsed().as_secs_f64(),
    })
}

async fn upload(
    State(s): State<Arc<AppState>>,
    Json(req): Json<UploadRequest>,
) -> Result<Json<UploadResponse>, StatusCode> {
    let id = Uuid::new_v4().to_string();
    let model = SdfModel {
        id: id.clone(),
        name: req.name.clone(),
        sdf_expression: req.sdf_expression,
        uploaded_at: chrono_stub_now(),
    };
    s.models.lock().unwrap().insert(id.clone(), model);
    Ok(Json(UploadResponse {
        id,
        name: req.name,
        message: "SDF model uploaded".into(),
    }))
}

async fn preview(
    State(s): State<Arc<AppState>>,
    Json(req): Json<PreviewRequest>,
) -> Result<Json<PreviewResponse>, StatusCode> {
    let models = s.models.lock().unwrap();
    if !models.contains_key(&req.model_id) {
        return Err(StatusCode::NOT_FOUND);
    }

    // alice-view stub: mesh generation from SDF
    let voxels = (req.resolution as usize).pow(3);
    let vertices = voxels / 8;
    let triangles = vertices * 2;

    Ok(Json(PreviewResponse {
        model_id: req.model_id.clone(),
        resolution: req.resolution,
        vertices,
        triangles,
        preview_url: format!("/preview/{}.glb", req.model_id),
    }))
}

async fn validate(
    State(s): State<Arc<AppState>>,
    Json(req): Json<ValidateRequest>,
) -> Result<Json<ValidateResponse>, StatusCode> {
    let models = s.models.lock().unwrap();
    if !models.contains_key(&req.model_id) {
        return Err(StatusCode::NOT_FOUND);
    }

    // alice-physics stub: structural stress analysis
    let yield_strength = match req.material.as_str() {
        "PLA" => 60.0,
        "ABS" => 40.0,
        "PETG" => 50.0,
        "Nylon" => 70.0,
        _ => 45.0,
    };
    let max_stress = yield_strength * 0.6; // stub
    let safety_factor = yield_strength / max_stress;

    Ok(Json(ValidateResponse {
        model_id: req.model_id,
        material: req.material,
        max_stress_mpa: max_stress,
        yield_strength_mpa: yield_strength,
        safety_factor,
        passed: safety_factor >= 1.5,
    }))
}

async fn slice(
    State(s): State<Arc<AppState>>,
    Json(req): Json<SliceRequest>,
) -> Result<Json<SliceResponse>, StatusCode> {
    let models = s.models.lock().unwrap();
    if !models.contains_key(&req.model_id) {
        return Err(StatusCode::NOT_FOUND);
    }

    // alice-print stub: slice & G-code generation
    let model_height_mm = 50.0; // stub
    let layers = (model_height_mm / req.layer_height_mm) as usize;
    let infill_factor = 1.0 + (req.infill_percent as f64 / 100.0);
    let gcode_lines = (layers as f64 * 120.0 * infill_factor) as usize + 200;
    let estimated_minutes = layers as f64 * 0.3;

    let job_id = Uuid::new_v4().to_string();
    let job = PrintJob {
        id: job_id.clone(),
        model_id: req.model_id.clone(),
        status: "completed".into(),
        gcode_lines,
        created_at: chrono_stub_now(),
    };
    s.jobs.lock().unwrap().push(job);

    Ok(Json(SliceResponse {
        job_id,
        model_id: req.model_id,
        layers,
        gcode_lines,
        estimated_minutes,
    }))
}

async fn list_jobs(State(s): State<Arc<AppState>>) -> Json<Vec<PrintJob>> {
    Json(s.jobs.lock().unwrap().clone())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn chrono_stub_now() -> String {
    // 軽量実装: chrono依存なしでUTC風タイムスタンプ
    format!("2026-03-07T00:00:00Z")
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("info".parse().unwrap()),
        )
        .init();

    let state = Arc::new(AppState {
        start_time: Instant::now(),
        models: Arc::new(Mutex::new(HashMap::new())),
        jobs: Arc::new(Mutex::new(Vec::new())),
    });

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/upload", post(upload))
        .route("/api/v1/preview", post(preview))
        .route("/api/v1/validate", post(validate))
        .route("/api/v1/slice", post(slice))
        .route("/api/v1/jobs", get(list_jobs))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    info!("3D Print Cloud Engine listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
