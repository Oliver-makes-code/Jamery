use axum::{Json, body::Body, extract::State, http::StatusCode, response::Response};
use jamery_common::auth::OAuthArgs;

use crate::{AppState, auth::get_oauth_url, fluxer::user::FluxerUser};

pub async fn login_get(State(state): State<AppState>) -> String {
    return get_oauth_url(&state, "meow").unwrap().to_string();
}

pub async fn login_post(State(state): State<AppState>, Json(args): Json<OAuthArgs>) -> Response {
    let user = match FluxerUser::fetch(&state, &args).await {
        Ok(user) => user,
        Err(err) => {
            eprintln!("{:?}", err);

            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap();
        }
    };

    let jwt = state.encode_jwt(&user.id).unwrap();

    return Response::builder()
        .status(StatusCode::CREATED)
        .header(
            axum::http::header::SET_COOKIE,
            format!("jamery_token={jwt}; Path=/; HttpOnly; SameSite=Lax; Max-Age=2592000"),
        )
        .body(Body::empty())
        .unwrap();
}
