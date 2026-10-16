use leptos::prelude::*;
use leptos_router::{
    components::{A, Route, Router, Routes, RoutingProgress},
    path,
};

use crate::auth::AuthRoutes;

pub mod auth;

#[component]
pub fn Index() -> impl IntoView {
    return view! {
        <nav>
            <A href="/auth/login">"Log in with FluxFur"</A>
        </nav>

        <main>
            <p>meow</p>
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
