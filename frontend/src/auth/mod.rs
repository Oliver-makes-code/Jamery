use gloo_net::http::Request;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoginState {
    LoggedIn,
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
        return self.state.get() == LoginState::LoggedIn;
    }

    pub fn logged_out(&self) -> bool {
        return self.state.get() != LoginState::LoggedIn;
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
                Ok(response) if response.ok() => LoginState::LoggedIn,
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
