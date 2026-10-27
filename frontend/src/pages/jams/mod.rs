use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::{
    MatchNestedRoutes,
    any_nested_route::IntoAnyNestedRoute,
    components::{Outlet, ParentRoute, Route},
    hooks::{use_navigate, use_params_map},
    path,
};
use ulid::Ulid;

use crate::{
    NotFound,
    auth::{
        login::{handle_oauth_redirect, logout},
        use_auth,
    },
    components::{
        button::{ActionButton, HamburgerToggleButton, LinkButton},
        header::HeaderBlock,
        markdown::MarkdownBlock,
    },
    fetch_jam,
    pages::jams::create::Create,
};

mod create;

#[component]
pub fn Jam() -> impl IntoView {
    let auth = use_auth();

    let params = use_params_map();

    let jam_id =
        move || params.with(|params| params.get("id").and_then(|id| id.parse::<Ulid>().ok()));

    let jam = LocalResource::new(move || async move {
        match jam_id() {
            Some(id) => fetch_jam(id).await,
            None => None,
        }
    });

    let (get_navbar, set_navbar) = signal(false);

    return view! {
        {move || Suspend::new(async move {
            match jam.await {
                Some(jam) => {
                    let title = jam.title.clone();

                    view! {
                        <Title text=format!("{} | Jamery", title) />
                        <main>
                            <HeaderBlock title=title.into()>
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

                            {
                                jam.description
                                    .clone()
                                    .filter(|description| !description.trim().is_empty())
                                    .map(|description| {
                                        view! {
                                            <MarkdownBlock markdown=description />
                                        }
                                    })
                            }
                        </main>
                    }.into_any()
                }
                None => view! {
                    <NotFound />
                }.into_any(),
            }
        })}
    };
}

#[component]
pub fn JamRoutes() -> impl MatchNestedRoutes + Clone + IntoAnyNestedRoute {
    return view! {
        <ParentRoute path=path!("/jams") view=Outlet>
            <Route path=path!("/create") view=Create />
            <Route path=path!("/:id") view=Jam />
        </ParentRoute>
    }
    .into_inner();
}
