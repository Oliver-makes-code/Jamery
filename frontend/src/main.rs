use jamery_common::{
    jam::{Jam, JamCreate},
    user::User,
};
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use ulid::Ulid;

use crate::{
    auth::AuthRoutes,
    components::{button::LinkButton, header::HeaderBlock},
    pages::{home::Home, jams::JamRoutes},
};

pub mod auth;
pub mod components;
pub mod pages;

pub async fn create_jam(create: JamCreate) -> Option<Jam> {
    return gloo_net::http::Request::post("/api/jams")
        .json(&create)
        .ok()?
        .send()
        .await
        .ok()?
        .json::<Jam>()
        .await
        .ok();
}

pub async fn fetch_jams() -> Vec<Jam> {
    let Ok(value) = gloo_net::http::Request::get("/api/jams").send().await else {
        return vec![];
    };

    let Ok(json) = value.json().await else {
        return vec![];
    };

    return json;
}

pub async fn fetch_jam(id: Ulid) -> Option<Jam> {
    return gloo_net::http::Request::get(&format!("/api/jams/{}", id))
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok();
}

pub async fn fetch_user(id: Ulid) -> Option<User> {
    return gloo_net::http::Request::get(&format!("/api/users/{}", id))
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok();
}

#[component]
fn NotFound() -> impl IntoView {
    return view! {
        <HeaderBlock title="Page not found.".into()>
            <LinkButton href="/".into()>Go Home</LinkButton>
        </HeaderBlock>
    };
}

#[component]
pub fn App() -> impl IntoView {
    return view! {
        <Router>
            <Routes fallback=NotFound>
                <Route path=path!("") view=Home />

                <AuthRoutes />
                <JamRoutes />
            </Routes>
        </Router>
    };
}

fn main() {
    mount_to_body(App);
}
