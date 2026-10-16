use url::Url;

use crate::AppState;

pub mod login;

pub fn get_redirect_uri(base_url: &str) -> Result<Url, url::ParseError> {
    let mut redirect_uri = Url::parse(base_url)?;
    redirect_uri.set_path("/auth/login");

    return Ok(redirect_uri);
}

pub fn get_oauth_url(app_state: &AppState, state: &str) -> Result<Url, url::ParseError> {
    let mut url = Url::parse(&app_state.fluxer_url)?;
    url.set_path("/oauth2/authorize");

    url.query_pairs_mut()
        .append_pair("client_id", &app_state.client_id)
        .append_pair(
            "redirect_uri",
            get_redirect_uri(&app_state.base_url)?.as_str(),
        )
        .append_pair("response_type", "code")
        .append_pair("scope", "identify")
        .append_pair("state", state);

    return Ok(url);
}
