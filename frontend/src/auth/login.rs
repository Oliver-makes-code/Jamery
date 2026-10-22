use jamery_common::auth::OAuthArgs;
use leptos::prelude::*;
use leptos_router::{
    NavigateOptions,
    hooks::{use_navigate, use_query},
    params::{Params, ParamsError},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Params, PartialEq)]
struct OAuthCallback {
    pub code: Option<String>,
    pub state: Option<String>,
}

pub async fn handle_oauth_redirect(navigate: impl Fn(&str, NavigateOptions)) {
    let Ok(resp) = gloo_net::http::Request::post("/api/auth/challenge")
        .send()
        .await
    else {
        return;
    };

    let Ok(oauth_url) = resp.text().await else {
        return;
    };

    navigate(&oauth_url, NavigateOptions::default());
}

async fn login(oauth: Memo<Result<OAuthCallback, ParamsError>>) {
    let oauth = oauth.with(|it| {
        let Ok(OAuthCallback {
            code: Some(code),
            state: Some(state),
        }) = it
        else {
            return None;
        };

        return Some((code.clone(), state.clone()));
    });

    let Some((code, state)) = oauth else {
        return;
    };

    let _ = gloo_net::http::Request::post("/api/auth/login")
        .json(&OAuthArgs { code, state })
        .unwrap()
        .send()
        .await;
}

pub async fn logout() {
    let _ = gloo_net::http::Request::post("/api/auth/logout")
        .send()
        .await;
}

#[component]
pub fn Login() -> impl IntoView {
    let oauth = use_query::<OAuthCallback>();

    let navigate = use_navigate();

    leptos::task::spawn_local_scoped(async move {
        login(oauth).await;

        navigate(
            "/",
            NavigateOptions {
                replace: true,
                ..Default::default()
            },
        );
    });

    return view! {
        <main>
            "Redirecting..."
        </main>
    };
}
