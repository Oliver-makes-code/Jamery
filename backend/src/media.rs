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
    Path(file): Path<String>,
) -> Result<Response, StatusCode> {
    let (id, extension) = file.rsplit_once('.').ok_or(StatusCode::BAD_REQUEST)?;

    let id = id.parse::<Ulid>().map_err(|_| StatusCode::BAD_REQUEST)?;

    let data = state
        .fetch_file(id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let mime = mime_guess::from_ext(extension).first_or_octet_stream();

    let content_type = mime.essence_str();

    return Response::builder()
        .header(axum::http::header::CONTENT_TYPE, content_type)
        .body(Body::from_stream(data))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
}
