use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new().route("/api/test", get(|| async { "meow" }))
}
