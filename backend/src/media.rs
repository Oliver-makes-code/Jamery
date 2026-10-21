use axum::{
    Json, Router,
    body::Body,
    extract::{Path, State},
    response::Response,
    routing::get,
};
use jamery_common::media::Media;
use reqwest::StatusCode;
use ulid::Ulid;

use crate::{AppState, map_status, map_status_opt};

pub async fn fetch_media(
    State(state): State<AppState>,
    Path(id): Path<Ulid>,
) -> Result<Response, StatusCode> {
    let media = map_status_opt(
        state.db.fetch_media(id).await,
        StatusCode::INTERNAL_SERVER_ERROR,
        StatusCode::NOT_FOUND,
    )?;

    let data = map_status(state.fetch_file(id).await, StatusCode::NOT_FOUND)?;

    return Response::builder()
        .header(axum::http::header::CONTENT_TYPE, media.content_type)
        .body(Body::from_stream(data))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
}

pub async fn fetch_media_meta(
    State(state): State<AppState>,
    Path(id): Path<Ulid>,
) -> Result<Json<Media>, StatusCode> {
    return map_status_opt(
        state.db.fetch_media(id).await,
        StatusCode::INTERNAL_SERVER_ERROR,
        StatusCode::NOT_FOUND,
    )
    .map(Json);
}

impl AppState {
    pub fn media_routes(&self, router: Router<Self>) -> Router<Self> {
        return router
            .route("/media/{path}", get(fetch_media))
            .route("/media/{path}/meta", get(fetch_media_meta));
    }
}
