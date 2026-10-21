use axum::{Json, body::Body, extract::State, http::StatusCode, response::Response};
use jamery_common::auth::OAuthArgs;

use crate::{AppState, fluxer::user::FluxerUser, map_status, map_status_opt};

pub async fn login(
    State(app_state): State<AppState>,
    Json(args): Json<OAuthArgs>,
) -> Result<Response, StatusCode> {
    let verifier = map_status_opt(
        app_state.db.consume_challenge(&args.state).await,
        StatusCode::INTERNAL_SERVER_ERROR,
        StatusCode::FORBIDDEN,
    )?;

    let user = map_status(
        FluxerUser::fetch(&app_state, &args, &verifier).await,
        StatusCode::INTERNAL_SERVER_ERROR,
    )?;

    let jwt = app_state.encode_jwt(user.id).unwrap();

    let username = user.full_username();
    let avatar_url = user.avatar_url(&app_state);

    map_status(
        app_state
            .db
            .upsert_user(user.id, username, user.global_name, avatar_url)
            .await,
        StatusCode::INTERNAL_SERVER_ERROR,
    )?;

    return Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header(
            axum::http::header::SET_COOKIE,
            format!("jamery_token={jwt}; Path=/; HttpOnly; SameSite=Lax; Max-Age=2592000"),
        )
        .body(Body::empty())
        .unwrap());
}
