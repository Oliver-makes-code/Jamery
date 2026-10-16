use axum::{Json, body::Body, http::StatusCode, response::Response};
use jamery_common::auth::OAuthArgs;

use crate::auth::get_oauth_url;

pub async fn login_get() -> String {
    return get_oauth_url("meow").unwrap().to_string();
}

pub async fn login_post(Json(args): Json<OAuthArgs>) -> Response {
    return Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("meow"))
        .unwrap();
}
