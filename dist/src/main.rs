use std::net::SocketAddr;

use axum::{
    body::Body,
    http::{Response, StatusCode, Uri, header},
};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "../frontend/dist/"]
struct Assets;

async fn serve_frontend(uri: Uri) -> Response<Body> {
    let requested_path = uri.path().trim_start_matches('/');

    if let Some(file) = Assets::get(requested_path) {
        let content_type = mime_guess::from_path(requested_path)
            .first_or_octet_stream()
            .to_string();

        return Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, content_type)
            .body(Body::from(file.data.into_owned()))
            .unwrap();
    }

    let Some(file) = Assets::get("index.html") else {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Frontend index.html not found"))
            .unwrap();
    };

    return Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .body(Body::from(file.data.into_owned()))
        .unwrap();
}

#[tokio::main]
async fn main() {
    let api = jamery_backend::router();

    let app = api.fallback(serve_frontend);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind address");

    println!("Jamery listening on http://{addr}");

    axum::serve(listener, app).await.expect("server failed");
}
