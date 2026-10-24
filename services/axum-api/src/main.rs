use axum::{
    extract::State,
    http::{header::CONTENT_TYPE, HeaderValue, Method, StatusCode},
    routing::{get, post},
    Json, Router,
};
use qoin_vector_engine::{decide, EngineConfig, ForceObservation, Signal};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::CorsLayer;

#[derive(Clone)]
struct AppState {
    config: EngineConfig,
    config_version: String,
}

#[derive(Serialize)]
struct Health {
    status: &'static str,
    mode: &'static str,
    live_trading: bool,
}

#[derive(Deserialize)]
struct SignalRequest {
    observation: ForceObservation,
    has_open_position: bool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let state = Arc::new(AppState {
        config: EngineConfig::default(),
        config_version: "baseline-v1".to_owned(),
    });
    let cors = CorsLayer::new()
        .allow_origin([
            HeaderValue::from_static("http://localhost:5173"),
            HeaderValue::from_static("http://127.0.0.1:5173"),
            HeaderValue::from_static("http://localhost:3000"),
            HeaderValue::from_static("http://127.0.0.1:3000"),
        ])
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE]);
    let app = Router::new()
        .route("/health", get(health))
        .route("/v1/signal", post(signal))
        .layer(cors)
        .with_state(state);

    // Loopback binding keeps this research API local by default.
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind API listener");
    tracing::info!(%addr, "Qoin research API listening; live trading is disabled");
    axum::serve(listener, app).await.expect("serve API");
}

async fn health() -> Json<Health> {
    Json(Health { status: "ok", mode: "research", live_trading: false })
}

async fn signal(
    State(state): State<Arc<AppState>>,
    Json(request): Json<SignalRequest>,
) -> Result<Json<Signal>, (StatusCode, String)> {
    let result = decide(
        request.observation,
        &state.config,
        &state.config_version,
        request.has_open_position,
    )
    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    // A signal is not an order. No exchange adapter is called here.
    Ok(Json(result))
}
