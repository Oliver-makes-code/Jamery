use chrono::Utc;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes, RoutingProgress},
    hooks::use_navigate,
    path,
};

use crate::{
    auth::{
        AuthRoutes,
        login::{handle_oauth_redirect, logout},
        use_auth,
    },
    components::{
        block::Block,
        button::{ActionButton, HamburgerToggleButton},
        card::{Card, ShowCards},
        file::{MultiFileInput, SingleFileInput},
        header::HeaderBlock,
        markdown::MarkdownBlock,
        text::{TextArea, TextInput},
        time::TimeInput,
    },
};

pub mod auth;
pub mod components;

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

    let (get_active, set_active) = signal(false);

    let (get_input, set_input) = signal(String::new());
    let (get_area, set_area) = signal(String::new());
    let (get_time, set_time) = signal(Utc::now());
    let (_get_file, set_file) = signal::<Option<web_sys::File>>(None);
    let (_get_files, set_files) = signal::<Vec<web_sys::File>>(Vec::new());

    return view! {
        <main>
            <HeaderBlock title="Jamery".into()>
                <HamburgerToggleButton value=get_active set_value=set_active />
                <Show when=move || get_active.get()>
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
            <ShowCards cards/>
            <Block>
                <TextInput value=get_input set_value=set_input label="First Input".into() placeholder="test".into() />
                <TextArea value=get_area set_value=set_area label="Second Input".into() placeholder="test2".into() />
                <TimeInput value=get_time set_value=set_time label="Time input".into() />

                <SingleFileInput
                    set_value=set_file
                    label="Upload Images".into()
                    accept="image/*".into()
                />

                <MultiFileInput
                    set_value=set_files
                    label="Attachments".to_string()
                    accept="image/*".into()
                />
            </Block>
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
