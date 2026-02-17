use axum::{Json, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    message: &'static str,
}
pub async fn health_check() -> impl IntoResponse {
    Json(HealthResponse {
        status: "ok",
        message: "Server is running",
    })
}
