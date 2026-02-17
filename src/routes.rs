use axum::{Json, Router, response::IntoResponse, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    message: &'static str,
}

async fn health_check() -> impl IntoResponse {
    Json(HealthResponse {
        status: "ok",
        message: "Server is running",
    })
}
pub fn gen_routes() -> Router {
    Router::new().route("/health", get(health_check))
}
