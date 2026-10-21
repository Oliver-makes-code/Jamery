use leptos::prelude::*;
use leptos_router::{
    components::{A, Route, Router, Routes, RoutingProgress},
    path,
};

use crate::{
    auth::AuthRoutes,
    card::{Card, ShowCards},
};

pub mod auth;
pub mod card;

#[component]
pub fn Index() -> impl IntoView {
    let cards = (0..16)
        .map(|_| Card {
            title: "My Submission".into(),
            link: "https://example.com".into(),
            authors: vec!["meow".into()],
            description: Some("My submission description".into()),
            icon: Some("https://media.discordapp.net/stickers/1360302217017692373.webp?size=240&quality=lossless".into()),
        })
        .collect();

    return view! {
        <nav>
            <A href="/auth/login">"Log in with FluxFur"</A>
        </nav>

        <main>
            <p>meow</p>
            <ShowCards cards/>
        </main>
    };
}

#[component]
pub fn App() -> impl IntoView {
    let (is_routing, set_is_routing) = signal(false);

    return view! {
        <Router set_is_routing=set_is_routing>
            <RoutingProgress is_routing />

            <Routes fallback=|| "Global 404: Page not found.">
                <Route path=path!("") view=Index />

                <AuthRoutes />
            </Routes>
        </Router>
    };
}

fn main() {
    mount_to_body(App);
}
