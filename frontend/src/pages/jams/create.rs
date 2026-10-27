use std::ops::Not;

use chrono::{DateTime, Utc};
use jamery_common::jam::{Jam, JamCreate};
use leptos::{html::summary, prelude::*};
use leptos_router::{NavigateOptions, hooks::use_navigate};

use crate::{
    auth::{
        Auth, LoginState,
        login::{handle_oauth_redirect, logout},
        use_auth,
    },
    components::{
        block::Block,
        button::{ActionButton, HamburgerToggleButton, LinkButton},
        card::{Card, ShowCards},
        header::HeaderBlock,
        markdown::MarkdownBlock,
        text::{TextArea, TextInput},
        time::TimeInput,
    },
};

async fn submit(
    title: String,
    summary: String,
    description: String,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    navigate: impl Fn(&str, NavigateOptions) + Clone,
) {
    let title = title.trim().to_string();
    let summary = summary.trim().to_string();
    let description = description.trim().to_string();

    if title.is_empty() {
        return;
    }

    let summary = (!summary.is_empty()).then_some(summary);
    let description = (!description.is_empty()).then_some(description);

    let create = JamCreate {
        title,
        summary,
        description,
        starts: start_time,
        ends: end_time,
    };

    let jam = gloo_net::http::Request::post("/api/jams")
        .json(&create)
        .unwrap()
        .send()
        .await
        .unwrap()
        .json::<Jam>()
        .await
        .unwrap();

    navigate(&format!("/jams/{}", jam.id), NavigateOptions::default())
}

#[component]
pub fn Create() -> impl IntoView {
    let auth = use_auth();

    let (get_navbar, set_navbar) = signal(false);

    let (get_title, set_title) = signal(String::new());
    let (get_summary, set_summary) = signal(String::new());
    let (get_desc, set_desc) = signal(String::new());
    let (get_start_time, set_start_time) = signal(Utc::now());
    let (get_end_time, set_end_time) = signal(Utc::now());

    let title_error = Signal::derive(move || get_title.get().trim().is_empty());

    let time_error = Signal::derive(move || get_end_time.get() < get_start_time.get());

    let card = Card {
        title: get_title.into(),
        link: Signal::derive(|| None),
        authors: Signal::derive(move || {
            let auth = auth.clone();

            let LoginState::LoggedIn(user) = auth.state.get() else {
                return vec![];
            };

            let username = user.display_name.unwrap_or(user.username);

            vec![username]
        }),
        icon: Signal::derive(|| None),
        summary: Signal::derive(move || Some(get_summary.get())),
        start_date: Signal::derive(move || Some(get_start_time.get())),
        end_date: Signal::derive(move || Some(get_end_time.get())),
    };

    return view! {
        <HeaderBlock title="Create a Jam".into()>
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
                </Show>
                <LinkButton href="/".into()>Go Home</LinkButton>
            </Show>
        </HeaderBlock>
        <Block>
            <Show when=move || auth.logged_out()>
                You must be logged in to create a jam.
            </Show>
            <Show when=move || auth.logged_in()>
                <TextInput id="jam-title".into() value=get_title set_value=set_title label="Jam Title".into() placeholder="Write jam title...".into() />
                <Show when=move || title_error.get()><p>Title is required.</p></Show>
                <TextInput id="jam-summary".into() value=get_summary set_value=set_summary label="Jam Summary".into() placeholder="Write jam summary...".into() />
                <TextArea id="jam-desc".into() value=get_desc set_value=set_desc label="Jam Description".into() placeholder="Write jam description...".into() />
                <TimeInput id="jam-start".into() value=get_start_time set_value=set_start_time label="Jam Start Date".into() />
                <TimeInput id="jam-end".into() value=get_end_time set_value=set_end_time label="Jam End Date".into() />
                <Show when=move || time_error.get()><p>Start date must be before end date.</p></Show>
                <ActionButton action=move || {
                    let get_title = get_title.clone();
                    let get_summary = get_summary.clone();
                    let get_desc = get_desc.clone();
                    let get_start_time = get_start_time.clone();
                    let get_end_time = get_end_time.clone();

                    let navigate = use_navigate();

                    async move {
                        if time_error.get() || title_error.get() {
                            return;
                        }

                        submit(get_title.get(), get_summary.get(), get_desc.get(), get_start_time.get(), get_end_time.get(), navigate).await;
                    }
                }>Create the jam!</ActionButton>
                <br/>
                <p>
                    You can view the previews for the card and description below.
                    The description supports markdown!
                </p>
            </Show>
        </Block>
        <ShowCards cards=Signal::stored(vec![card]) />
        {move || {
            let desc = get_desc.get();

            view! {
                <MarkdownBlock markdown=desc />
            }
        }}
    };
}
