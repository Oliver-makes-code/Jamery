use serde::{Deserialize, Serialize};

pub mod user;

#[derive(Debug, Serialize, Deserialize)]
pub struct FluxerWellKnown {
    pub endpoints: Endpoints,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Endpoints {
    pub api: String,
    pub media: String,
}

pub struct FluxerEndpoints;

impl FluxerEndpoints {
    pub fn well_known(url: &str) -> String {
        return format!("{url}/.well-known/fluxer");
    }

    pub fn oauth_token(url: &str) -> String {
        return format!("{url}/v1/oauth2/token");
    }

    pub fn users_me(url: &str) -> String {
        return format!("{url}/users/@me");
    }

    pub fn avatar(url: &str, user_id: &str, avatar_hash: &str) -> String {
        return format!("{url}/avatars/{user_id}/{avatar_hash}.webp");
    }
}
