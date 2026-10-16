use jamery_common::auth::OAuthArgs;
use serde::{Deserialize, Serialize};

use crate::{AppState, auth::get_redirect_uri, fluxer::FluxerEndpoints};

#[derive(Debug, Serialize, Deserialize)]
pub struct FluxerUser {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OAuthToken {
    access_token: String,
}

impl FluxerUser {
    pub async fn fetch(state: &AppState, args: &OAuthArgs) -> anyhow::Result<Self> {
        let response = state
            .http
            .post(FluxerEndpoints::oauth_token(&state.fluxer_api))
            .form(&[
                ("grant_type", "authorization_code"),
                ("code", &args.code),
                ("redirect_uri", get_redirect_uri(&state.base_url)?.as_str()),
                ("client_id", state.client_id.as_str()),
                ("client_secret", state.client_secret.as_str()),
            ])
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            anyhow::bail!(
                "OAuth token exchange failed ({status}): {}",
                response.text().await?
            );
        }

        let token = response.json::<OAuthToken>().await?.access_token;

        let user = state
            .http
            .get(FluxerEndpoints::users_me(&state.fluxer_api))
            .bearer_auth(&token)
            .send()
            .await?
            .json::<Self>()
            .await?;

        return Ok(user);
    }

    pub fn full_username(&self) -> String {
        return format!("{}#{}", self.username, self.discriminator);
    }

    pub fn avatar_url(&self, state: &AppState) -> Option<String> {
        return Some(FluxerEndpoints::avatar(
            &state.fluxer_media,
            &self.id,
            self.avatar.as_deref()?,
        ));
    }
}
