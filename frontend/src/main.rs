use codee::string::FromToStringCodec;
use leptos::prelude::*;
use leptos_router::{
    components::{A, Route, Router, Routes, RoutingProgress},
    hooks::use_navigate,
    path,
};

use crate::{
    auth::{
        AuthRoutes, LoginState,
        login::{handle_oauth_redirect, logout},
        use_auth,
    },
    components::{
        block::Block,
        button::ActionButton,
        card::{Card, ShowCards},
        header::HeaderBlock,
    },
};

pub mod auth;
pub mod components;

#[component]
pub fn Index() -> impl IntoView {
    let cards = (0..4)
        .map(|_| Card {
            title: "My Submission".into(),
            link: "https://example.com".into(),
            authors: vec!["meow".into()],
            description: Some("My submission description".into()),
            icon: Some("https://media.discordapp.net/stickers/1360302217017692373.webp?size=240&quality=lossless".into()),
        })
        .collect();

    let auth = use_auth();

    return view! {
        <main>
            <HeaderBlock title="Jamery".into()>
                <Show when=move || auth.logged_out()>
                    <ActionButton action=move || handle_oauth_redirect(use_navigate())>Login</ActionButton>
                </Show>
                <Show when=move || auth.logged_in()>
                    <ActionButton action=move || {
                        let auth = auth.clone();


                        async move {
                            logout().await;
                            auth.refresh();
                        }
                    }>Logout</ActionButton>
                </Show>
            </HeaderBlock>
            <Block>
                <p>meow</p>
            </Block>
            <ShowCards cards/>
            <Block> meow </Block>
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
