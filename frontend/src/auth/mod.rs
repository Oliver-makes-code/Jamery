use gloo_net::http::Request;
use jamery_common::user::User;
use leptos::prelude::*;
use leptos_router::{
    MatchNestedRoutes,
    any_nested_route::IntoAnyNestedRoute,
    components::{Outlet, ParentRoute, Route},
    path,
};

use crate::auth::login::Login;

pub mod login;

#[component]
pub fn AuthRoutes() -> impl MatchNestedRoutes + Clone + IntoAnyNestedRoute {
    return view! {
        <ParentRoute path=path!("/auth") view=Outlet>
            <Route path=path!("/login") view=Login />
        </ParentRoute>
    }
    .into_inner();
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoginState {
    LoggedIn(User),
    LoggedOut,
    Error,
}

#[derive(Clone, Copy)]
pub struct Auth {
    pub state: ReadSignal<LoginState>,
    pub refresh: Callback<()>,
}

impl Auth {
    pub fn logged_in(&self) -> bool {
        return matches!(self.state.get(), LoginState::LoggedIn(_));
    }

    pub fn logged_out(&self) -> bool {
        return matches!(self.state.get(), LoginState::LoggedOut | LoginState::Error);
    }

    pub fn refresh(&self) {
        self.refresh.run(());
    }
}

pub fn use_auth() -> Auth {
    let (login_state, set_login_state) = signal(LoginState::LoggedOut);

    let refresh = Callback::new(move |_| {
        let set_login_state = set_login_state;

        leptos::task::spawn_local(async move {
            let result = Request::get("/api/users/@me").send().await;

            set_login_state.set(match result {
                Ok(response) if response.status() == 401 => LoginState::LoggedOut,
                Ok(response) if response.ok() => {
                    LoginState::LoggedIn(response.json().await.unwrap_or(User::default()))
                }
                _ => LoginState::Error,
            });
        });
    });

    refresh.run(());

    return Auth {
        state: login_state,
        refresh,
    };
}
