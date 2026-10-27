use axum::{
    Extension, Json, Router,
    extract::{Path, Query, State},
    routing::{get, post},
};
use jamery_common::{
    auth::Claims,
    jam::{Jam, JamCreate},
};
use reqwest::StatusCode;
use ulid::Ulid;

use crate::{AppState, Pagination, map_status, map_status_opt};

async fn create_jam(
    State(app_state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(mut jam): Json<JamCreate>,
) -> Result<Json<Jam>, StatusCode> {
    jam.title = jam.title.trim().into();
    jam.description = jam.description.map(|it| it.trim().into());
    jam.summary = jam.summary.map(|it| it.trim().into());

    if jam.title.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    if jam
        .description
        .as_ref()
        .is_some_and(|it| it.trim().is_empty())
    {
        jam.description = None;
    }

    if jam.summary.as_ref().is_some_and(|it| it.trim().is_empty()) {
        jam.summary = None;
    }

    return map_status(
        app_state.db.create_jam(claims.sub, jam).await,
        StatusCode::INTERNAL_SERVER_ERROR,
    )
    .map(Json);
}

async fn fetch_jams(
    State(app_state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Box<[Jam]>>, StatusCode> {
    return map_status(
        app_state
            .db
            .fetch_jams(pagination.page, pagination.page_size.get())
            .await,
        StatusCode::INTERNAL_SERVER_ERROR,
    )
    .map(Json);
}

async fn fetch_jam(
    State(app_state): State<AppState>,
    Path(jam_id): Path<Ulid>,
) -> Result<Json<Jam>, StatusCode> {
    return map_status_opt(
        app_state.db.fetch_jam(jam_id).await,
        StatusCode::INTERNAL_SERVER_ERROR,
        StatusCode::NOT_FOUND,
    )
    .map(Json);
}

impl AppState {
    pub fn jam_routes(&self, router: Router<Self>) -> Router<Self> {
        return router
            .route("/api/jams", self.authorized(post(create_jam)))
            .route("/api/jams", get(fetch_jams))
            .route("/api/jams/{jam_id}", get(fetch_jam));
    }
}
