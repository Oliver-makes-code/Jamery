use axum::{
    Router,
    routing::{get, post},
};

use crate::auth::login::{login_get, login_post};

pub mod auth;

pub fn router() -> Router {
    return Router::new()
        .route("/api/auth/login", get(login_get))
        .route("/api/auth/login", post(login_post));
}
