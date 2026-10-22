use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    routing::get,
};
use jamery_common::{auth::Claims, user::User};
use reqwest::StatusCode;
use ulid::Ulid;

use crate::{AppState, map_status_opt};

pub async fn users(
    State(app_state): State<AppState>,
    Path(user_id): Path<Ulid>,
) -> Result<Json<User>, StatusCode> {
    get_user(&app_state, user_id).await
}

pub async fn users_me(
    State(app_state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<User>, StatusCode> {
    get_user(&app_state, claims.sub).await
}

async fn get_user(app_state: &AppState, user_id: Ulid) -> Result<Json<User>, StatusCode> {
    return map_status_opt(
        app_state.db.fetch_user(user_id).await,
        StatusCode::INTERNAL_SERVER_ERROR,
        StatusCode::NOT_FOUND,
    )
    .map(Json);
}

impl AppState {
    pub fn user_routes(&self, router: Router<Self>) -> Router<Self> {
        return router
            .route("/api/users/@me", self.authorized(get(users_me)))
            .route("/api/users/{user}", get(users));
    }
}
