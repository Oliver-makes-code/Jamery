use leptos::prelude::*;
use leptos_router::{
    MatchNestedRoutes,
    any_nested_route::IntoAnyNestedRoute,
    components::{Outlet, ParentRoute, Route},
    path,
};

use crate::pages::jams::create::Create;

mod create;

#[component]
pub fn JamRoutes() -> impl MatchNestedRoutes + Clone + IntoAnyNestedRoute {
    return view! {
        <ParentRoute path=path!("/jam") view=Outlet>
            <Route path=path!("/create") view=Create />
        </ParentRoute>
    }
    .into_inner();
}
