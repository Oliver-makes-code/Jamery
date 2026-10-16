use std::env;

use url::Url;

pub mod login;

pub fn get_oauth_url(state: &str) -> anyhow::Result<Url> {
    let client_id = env::var("JAMERY_CLIENT_ID")?;
    let fluxer_url = env::var("JAMERY_FLUXER_URL")?;
    let base_url = env::var("JAMERY_BASE_URL")?;

    let mut url = Url::parse(&fluxer_url)?;
    url.set_path("/oauth2/authorize");

    let mut redirect_uri = Url::parse(&base_url)?;
    redirect_uri.set_path("/auth/login");

    url.query_pairs_mut()
        .append_pair("client_id", &client_id)
        .append_pair("redirect_uri", redirect_uri.as_str())
        .append_pair("response_type", "code")
        .append_pair("scope", "identify")
        .append_pair("state", state);

    return Ok(url);
}
