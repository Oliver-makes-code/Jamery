use jamery_common::jam::Jam;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::{
    auth::{
        login::{handle_oauth_redirect, logout},
        use_auth,
    },
    components::{
        button::{ActionButton, HamburgerToggleButton, LinkButton},
        card::{Card, ShowCards},
        header::HeaderBlock,
        markdown::MarkdownBlock,
    },
    fetch_jams,
};

#[component]
pub fn Home() -> impl IntoView {
    let auth = use_auth();

    let (get_navbar, set_navbar) = signal(false);

    let (get_jams, set_jams) = signal::<Vec<Card>>(Vec::new());

    leptos::task::spawn_local(async move {
        let mut jams = Vec::new();

        for jam in fetch_jams().await {
            let card = Card {
                title: Signal::stored(jam.title),
                link: Signal::stored(Some(format!("/jams/{}", jam.id))),
                authors: Signal::stored(vec![jam.created_by.to_string()]),
                start_date: Signal::stored(Some(jam.starts)),
                end_date: Signal::stored(Some(jam.ends)),
                summary: Signal::stored(jam.summary),
                icon: Signal::stored(None),
            };

            jams.push(card);
        }

        set_jams.set(jams);
    });

    return view! {
        <main>
            <HeaderBlock title="Jamery".into()>
                <HamburgerToggleButton value=get_navbar set_value=set_navbar />
                <Show when=move || get_navbar.get()>
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
                        <LinkButton href="/jam/create".into()>Create a Jam</LinkButton>
                    </Show>
                </Show>
            </HeaderBlock>
            <MarkdownBlock markdown=include_str!("./home.md").to_string() />
            <ShowCards cards=get_jams.into() />
        </main>
    };
}
