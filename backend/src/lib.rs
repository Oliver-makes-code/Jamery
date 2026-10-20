#![feature(array_try_from_fn, duration_constructors)]

use std::{
    env,
    time::{Duration, SystemTime},
};

use axum::{
    Router,
    extract::{DefaultBodyLimit, Request, State},
    http::HeaderMap,
    middleware::{self, Next},
    response::Response,
    routing::{MethodRouter, get, post},
};
use jamery_common::auth::Claims;
use jamery_database::handler::DatabaseHandler;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use opendal::{Buffer, Operator};
use pawkit_interner::InternString;
use reqwest::{
    Client, StatusCode,
    header::{self, COOKIE},
};
use ulid::Ulid;

use crate::{
    auth::{challenge::challenge, login::login},
    fluxer::{FluxerEndpoints, FluxerWellKnown},
    media::fetch_media,
    user::{users, users_me},
};

pub mod auth;
pub mod fluxer;
pub mod media;
pub mod user;

#[derive(Clone)]
pub struct AppState {
    pub http: Client,
    pub db: DatabaseHandler,
    pub op: Operator,

    pub fluxer_url: InternString,
    pub fluxer_api: InternString,
    pub fluxer_media: InternString,
    pub client_id: InternString,
    pub client_secret: InternString,
    pub base_url: InternString,

    pub jwt_header: Header,
    pub jwt_encoding: EncodingKey,
    pub jwt_decoding: DecodingKey,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let database_url = env::var("JAMERY_DATABASE_URL")?;
        let fluxer_url: InternString = env::var("JAMERY_FLUXER_URL")?.trim_end_matches('/').into();
        let client_id = env::var("JAMERY_CLIENT_ID")?.into();
        let client_secret = env::var("JAMERY_CLIENT_SECRET")?.into();
        let base_url = env::var("JAMERY_BASE_URL")?.trim_end_matches('/').into();
        let jwt_secret = env::var("JAMERY_JWT_SECRET")?;
        let bulk_path = env::var("JAMERY_BULK_PATH")?;

        anyhow::ensure!(
            jwt_secret.len() == 64 && jwt_secret.bytes().all(|b| b.is_ascii_hexdigit()),
            "JAMERY_JWT_SECRET must be 64 hex characters"
        );

        let jwt_secret = std::array::try_from_fn::<_, 32, _>(|n| {
            let start = n * 2;
            let end = start + 2;

            let substr = &jwt_secret[start..end];

            u8::from_str_radix(substr, 16)
        })?;

        let jwt_encoding = jsonwebtoken::EncodingKey::from_secret(&jwt_secret);
        let jwt_decoding = jsonwebtoken::DecodingKey::from_secret(&jwt_secret);

        let http = Client::builder()
            .user_agent(format!(
                "Jamery/{} (+{})",
                env!("CARGO_PKG_VERSION"),
                base_url
            ))
            .build()?;

        let endpoints = http
            .get(FluxerEndpoints::well_known(&fluxer_url))
            .send()
            .await?
            .json::<FluxerWellKnown>()
            .await?
            .endpoints;

        let fluxer_api = endpoints.api.trim_end_matches('/').into();
        let fluxer_media = endpoints.media.trim_end_matches('/').into();

        let db = DatabaseHandler::new(&database_url).await?;

        tokio::fs::create_dir_all(&bulk_path).await?;
        let builder = opendal::services::Fs::default().root(&bulk_path);
        let op = Operator::new(builder)?;

        return Ok(Self {
            http,
            db,
            op,
            fluxer_url,
            fluxer_api,
            fluxer_media,
            client_id,
            client_secret,
            base_url,
            jwt_header: Header::new(Algorithm::HS256),
            jwt_encoding,
            jwt_decoding,
        });
    }

    pub fn encode_jwt(&self, user_id: u64) -> Result<String, jsonwebtoken::errors::Error> {
        let now = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock is before Unix epoch")
            .as_secs();

        let exp = now + Duration::from_days(30).as_secs();

        return jsonwebtoken::encode(
            &self.jwt_header,
            &Claims {
                exp,
                sub: user_id.into(),
            },
            &self.jwt_encoding,
        );
    }

    pub async fn upload_file(&self, data: &[u8]) -> Result<Ulid, opendal::Error> {
        let id = Ulid::generate();

        self.op.write(&id.to_string(), data.to_vec()).await?;

        return Ok(id);
    }

    pub async fn fetch_file(&self, id: Ulid) -> Result<Buffer, opendal::Error> {
        return self.op.read(&id.to_string()).await;
    }

    fn extract_token<'a>(&self, headers: &'a HeaderMap) -> Result<&'a str, StatusCode> {
        if let Some(auth) = headers.get(header::AUTHORIZATION) {
            let auth = auth.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?.trim();

            let Some((scheme, token)) = auth.split_once(' ') else {
                return Err(StatusCode::UNAUTHORIZED);
            };

            if !scheme.eq_ignore_ascii_case("bearer") || token.is_empty() {
                return Err(StatusCode::UNAUTHORIZED);
            }

            return Ok(token);
        }

        let cookie_header = headers
            .get(COOKIE)
            .and_then(|value| value.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let token = cookie_header
            .split(';')
            .filter_map(|cookie| cookie.trim().split_once('='))
            .find(|(name, _)| *name == "jamery_token")
            .map(|(_, value)| value)
            .ok_or(StatusCode::UNAUTHORIZED)?;

        return Ok(token);
    }

    pub fn get_token(&self, headers: &HeaderMap) -> Result<Claims, StatusCode> {
        let token = self.extract_token(headers)?;

        let token_data = jsonwebtoken::decode::<Claims>(
            token,
            &self.jwt_decoding,
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

        return Ok(token_data.claims);
    }

    async fn authorization_middleware(
        State(app_state): State<AppState>,
        mut request: Request,
        next: Next,
    ) -> Result<Response, StatusCode> {
        let claims = app_state.get_token(request.headers())?;

        request.extensions_mut().insert(claims);

        return Ok(next.run(request).await);
    }

    pub fn authorized(&self, route: MethodRouter<AppState>) -> MethodRouter<AppState> {
        route.route_layer(middleware::from_fn_with_state(
            self.clone(),
            Self::authorization_middleware,
        ))
    }
}

pub async fn router() -> anyhow::Result<Router> {
    let state = AppState::new().await?;

    return Ok(Router::new()
        .route("/api/auth/challenge", post(challenge))
        .route("/api/auth/login", post(login))
        .route("/api/users/@me", state.authorized(get(users_me)))
        .route("/api/users/{user}", get(users))
        .route("/media/{path}", get(fetch_media))
        .layer(DefaultBodyLimit::max(2 * 1024 * 1024))
        .with_state(state));
}
