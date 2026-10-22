use leptos::prelude::*;
use leptos_router::{
    components::{A, Route, Router, Routes, RoutingProgress},
    path,
};

use crate::{
    auth::AuthRoutes,
    block::Block,
    button::{BlockButton, ButtonBehavior},
    card::{Card, ShowCards},
    header::HeaderBlock,
};

pub mod auth;
pub mod block;
pub mod button;
pub mod card;
pub mod header;

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

    let (get, set) = signal(false);

    return view! {
        <nav>
            <A href="/auth/login">"Log in with FluxFur"</A>
        </nav>

        <main>
            <HeaderBlock title="Jamery".into()> <BlockButton behavior=ButtonBehavior::Toggle(set)>waow</BlockButton> </HeaderBlock>
            <Block>
                <p>meow</p>
                <p>
                    <BlockButton behavior=ButtonBehavior::Link("https://example.com/".into())> wawa </BlockButton>
                    <BlockButton behavior=ButtonBehavior::Link("https://example.com/".into())> wawa </BlockButton>
                </p>
                <Show when=move || get.get()>
                    <p>"waow"</p>
                </Show>
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
