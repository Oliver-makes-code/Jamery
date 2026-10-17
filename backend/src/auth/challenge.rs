use axum::{body::Body, extract::State, response::Response};
use reqwest::StatusCode;

use crate::{
    AppState,
    auth::{generate_oauth_secrets, get_oauth_url},
};

pub async fn challenge(State(app_state): State<AppState>) -> Response {
    let (state, verifier) = match generate_oauth_secrets() {
        Ok(it) => it,
        Err(err) => {
            eprintln!("{:?}", err);

            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap();
        }
    };

    let oauth_url = get_oauth_url(&app_state, &state, &verifier)
        .unwrap()
        .to_string();

    if let Err(err) = app_state.db.begin_challenge(state, verifier).await {
        eprintln!("{:?}", err);

        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())
            .unwrap();
    }

    return Response::builder()
        .status(StatusCode::OK)
        .body(Body::new(oauth_url))
        .unwrap();
}
