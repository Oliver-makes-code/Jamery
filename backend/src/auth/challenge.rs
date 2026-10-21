use axum::{body::Body, extract::State};
use reqwest::StatusCode;

use crate::{
    AppState,
    auth::{generate_oauth_secrets, get_oauth_url},
    map_status,
};

pub async fn challenge(State(app_state): State<AppState>) -> Result<Body, StatusCode> {
    let (state, verifier) =
        map_status(generate_oauth_secrets(), StatusCode::INTERNAL_SERVER_ERROR)?;

    let oauth_url = map_status(
        get_oauth_url(&app_state, &state, &verifier),
        StatusCode::INTERNAL_SERVER_ERROR,
    )?
    .to_string();

    map_status(
        app_state.db.begin_challenge(state, verifier).await,
        StatusCode::INTERNAL_SERVER_ERROR,
    )?;

    return Ok(Body::new(oauth_url));
}
