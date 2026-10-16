use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <main>
            <h1>"Jamery!"</h1>
            <p>"It's me, Jamery!"</p>
        </main>
    }
}

fn main() {
    mount_to_body(App);
}
