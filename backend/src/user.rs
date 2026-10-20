use axum::{
    Extension, Json,
    extract::{Path, State},
};
use jamery_common::{auth::Claims, user::User};
use reqwest::StatusCode;

use crate::AppState;

pub async fn users(
    State(app_state): State<AppState>,
    Path(user_id): Path<u64>,
) -> Result<Json<User>, StatusCode> {
    get_user(&app_state, user_id).await
}

pub async fn users_me(
    State(app_state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<User>, StatusCode> {
    get_user(&app_state, claims.sub).await
}

async fn get_user(app_state: &AppState, user_id: u64) -> Result<Json<User>, StatusCode> {
    match app_state.db.fetch_user(user_id).await {
        Ok(Some(user)) => return Ok(Json(user)),
        Ok(None) => {
            return Err(StatusCode::NOT_FOUND);
        }
        Err(err) => {
            eprintln!("{:?}", err);

            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}
