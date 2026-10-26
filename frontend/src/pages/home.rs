use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::{
    auth::{
        login::{handle_oauth_redirect, logout},
        use_auth,
    },
    components::{
        button::{ActionButton, HamburgerToggleButton, LinkButton},
        header::HeaderBlock,
        markdown::MarkdownBlock,
    },
};

#[component]
pub fn Home() -> impl IntoView {
    let auth = use_auth();

    let (get_navbar, set_navbar) = signal(false);

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
        </main>
    };
}
