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
