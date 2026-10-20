use axum::{
    body::Body,
    extract::{Path, State},
    response::Response,
};
use reqwest::StatusCode;
use ulid::Ulid;

use crate::AppState;

pub async fn fetch_media(
    State(state): State<AppState>,
    Path(id): Path<Ulid>,
) -> Result<Response, StatusCode> {
    let data = state
        .fetch_file(id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    return Response::builder()
        .header(axum::http::header::CONTENT_TYPE, "application/octet-stream")
        .body(Body::from_stream(data))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
}
