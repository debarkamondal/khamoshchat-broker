use axum::{Router, routing::get};

use crate::handlers::health::health_check;

pub fn gen_routes() -> Router {
    Router::new().route("/health", get(health_check))
}
