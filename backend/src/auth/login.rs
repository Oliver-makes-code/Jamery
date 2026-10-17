use axum::{Json, body::Body, extract::State, http::StatusCode, response::Response};
use jamery_common::auth::OAuthArgs;

use crate::{AppState, fluxer::user::FluxerUser};

pub async fn login(State(app_state): State<AppState>, Json(args): Json<OAuthArgs>) -> Response {
    let verifier = match app_state.db.consume_challenge(&args.state).await {
        Ok(Some(verifier)) => verifier,
        Ok(None) => {
            return Response::builder()
                .status(StatusCode::FORBIDDEN)
                .body(Body::empty())
                .unwrap();
        }
        Err(err) => {
            eprintln!("{:?}", err);

            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap();
        }
    };

    let user = match FluxerUser::fetch(&app_state, &args, &verifier).await {
        Ok(user) => user,
        Err(err) => {
            eprintln!("{:?}", err);

            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap();
        }
    };

    let jwt = app_state.encode_jwt(user.id).unwrap();

    let username = user.full_username();
    let avatar_url = user.avatar_url(&app_state);

    if let Err(err) = app_state
        .db
        .upsert_user(user.id, username, user.global_name, avatar_url)
        .await
    {
        eprintln!("{:?}", err);

        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())
            .unwrap();
    }

    return Response::builder()
        .status(StatusCode::CREATED)
        .header(
            axum::http::header::SET_COOKIE,
            format!("jamery_token={jwt}; Path=/; HttpOnly; SameSite=Lax; Max-Age=2592000"),
        )
        .body(Body::empty())
        .unwrap();
}
