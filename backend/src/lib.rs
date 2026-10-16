#![feature(array_try_from_fn, duration_constructors)]

use std::{
    env,
    time::{Duration, SystemTime},
};

use axum::{
    Router,
    routing::{get, post},
};
use jamery_common::auth::Claims;
use jamery_database::handler::DatabaseHandler;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header};
use pawkit_interner::InternString;
use reqwest::Client;

use crate::{
    auth::login::{login_get, login_post},
    fluxer::{FluxerEndpoints, FluxerWellKnown},
};

pub mod auth;
pub mod fluxer;

#[derive(Clone)]
pub struct AppState {
    pub http: Client,
    pub db: DatabaseHandler,

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

        return Ok(Self {
            http,
            db,
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

    pub fn encode_jwt(&self, user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
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
}

pub async fn router() -> anyhow::Result<Router> {
    let state = AppState::new().await?;

    return Ok(Router::new()
        .route("/api/auth/login", get(login_get))
        .route("/api/auth/login", post(login_post))
        .with_state(state));
}
