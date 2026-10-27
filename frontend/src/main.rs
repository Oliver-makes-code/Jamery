use chrono::Utc;
use jamery_common::{jam::Jam, user::User};
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes, RoutingProgress},
    hooks::use_navigate,
    path,
};
use ulid::Ulid;

use crate::{
    auth::{
        AuthRoutes,
        login::{handle_oauth_redirect, logout},
        use_auth,
    },
    components::{
        block::Block,
        button::{ActionButton, HamburgerToggleButton, LinkButton},
        card::{Card, ShowCards},
        file::{MultiFileInput, SingleFileInput},
        header::HeaderBlock,
        markdown::MarkdownBlock,
        text::{TextArea, TextInput},
        time::TimeInput,
    },
    pages::{home::Home, jams::JamRoutes},
};

pub mod auth;
pub mod components;
pub mod pages;

const MARKDOWN_TEST: &str = r#"
# Waow! Markdown !

```
console.log("A codeblock");
```

> Quotes,

- A
- List

`inline code`, **bold**, *ital*, and ~~strike~~! 

## Smaller heading

paragraph,
pafsgf

table|a|b
-|-|-
1|2|3
4|5|6

***
***
***
"#;

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
pub fn Index() -> impl IntoView {
    let cards = (0..4)
        .map(|_| {
            let (title, _) = signal("My Submission".to_string());
            let (link, _) = signal(Some("https://example.com".to_string()));
            let (authors, _) = signal(vec!["meow".to_string()]);
            let (summary, _) =
                signal(Some("My submission summary".to_string()));
            let (icon, _) = signal(Some(
                "https://media.discordapp.net/stickers/1360302217017692373.webp?size=240&quality=lossless"
                    .to_string(),
            ));
            let (time, _) = signal(None);

            return Card {
                title: title.into(),
                link: link.into(),
                authors: authors.into(),
                summary: summary.into(),
                icon: icon.into(),
                start_date: time.into(),
                end_date: time.into()
            }
        })
        .collect::<Vec<_>>();

    let auth = use_auth();

    let (get_navbar, set_navbar) = signal(false);

    let (get_input, set_input) = signal(String::new());
    let (get_area, set_area) = signal(String::new());
    let (get_time, set_time) = signal(Utc::now());
    let (_get_file, set_file) = signal::<Option<web_sys::File>>(None);
    let (_get_files, set_files) = signal::<Vec<web_sys::File>>(Vec::new());

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
                    </Show>
                </Show>
            </HeaderBlock>
            <MarkdownBlock markdown=MARKDOWN_TEST.to_owned() />
            <ShowCards cards=Signal::stored(cards)/>
            <Block>
                <TextInput id="text-input".into() value=get_input set_value=set_input label="First Input".into() placeholder="test".into() />
                <TextArea id="text-area".into() value=get_area set_value=set_area label="Second Input".into() placeholder="test2".into() />
                <TimeInput id="time".into() value=get_time set_value=set_time label="Time input".into() />

                <SingleFileInput
                    set_value=set_file
                    id="single".into()
                    label="Upload Images".into()
                    accept="image/*".into()
                />

                <MultiFileInput
                    set_value=set_files
                    id="multiple".into()
                    label="Attachments".to_string()
                    accept="image/*".into()
                />
            </Block>
        </main>
    };
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
