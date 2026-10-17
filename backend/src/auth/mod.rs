use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use getrandom::fill;
use sha2::{Digest, Sha256};
use url::Url;

use crate::AppState;

pub mod challenge;
pub mod login;

pub fn get_redirect_uri(base_url: &str) -> Result<Url, url::ParseError> {
    let mut redirect_uri = Url::parse(base_url)?;
    redirect_uri.set_path("/auth/login");

    return Ok(redirect_uri);
}

pub fn generate_oauth_secrets() -> Result<(String, String), getrandom::Error> {
    let mut state_bytes = [0u8; 32];
    let mut verifier_bytes = [0u8; 32];

    fill(&mut state_bytes)?;
    fill(&mut verifier_bytes)?;

    let state = URL_SAFE_NO_PAD.encode(state_bytes);
    let verifier = URL_SAFE_NO_PAD.encode(verifier_bytes);

    return Ok((state, verifier));
}

pub fn get_oauth_url(
    app_state: &AppState,
    state: &str,
    verifier: &str,
) -> Result<Url, url::ParseError> {
    let mut url = Url::parse(&app_state.fluxer_url)?;
    url.set_path("/oauth2/authorize");

    let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));

    url.query_pairs_mut()
        .append_pair("client_id", &app_state.client_id)
        .append_pair(
            "redirect_uri",
            get_redirect_uri(&app_state.base_url)?.as_str(),
        )
        .append_pair("response_type", "code")
        .append_pair("scope", "identify")
        .append_pair("state", state)
        .append_pair("code_challenge", &challenge)
        .append_pair("code_challenge_method", "S256");

    return Ok(url);
}
