use leptos::prelude::*;

const BUTTON_CLASS: &str = "
    my-0 box-border inline-flex items-center justify-center align-middle
    font-[inherit] leading-normal
    px-4 py-2 bg-[#EBB7D7] text-black
    shadow-[8px_8px_0px_0px_rgba(0,0,0,0.25)]
    hover:bg-[#F1CEE4] transition
";
const HAMBURGER_BUTTON_CLASS: &str = "
    my-0 box-border inline-flex items-center justify-center align-middle
    font-[inherit] leading-normal
    px-1 py-1 bg-[#EBB7D7] text-black
    shadow-[8px_8px_0px_0px_rgba(0,0,0,0.25)]
    hover:bg-[#F1CEE4] transition
";

#[component]
pub fn LinkButton(href: String, children: Children) -> impl IntoView {
    return view! {
        <a href=href class=BUTTON_CLASS>
            {children()}
        </a>
    };
}

#[component]
pub fn HamburgerToggleButton(
    value: ReadSignal<bool>,
    set_value: WriteSignal<bool>,
) -> impl IntoView {
    return view! {
        <button
            type="button"
            class=HAMBURGER_BUTTON_CLASS
            aria-pressed=move || value.get()
            on:click=move |_| {
                set_value.update(|value| *value = !*value);
            }
        >
            <img width="32" src="/site/hamburger_menu.svg" alt="Hamburger Menu" />
        </button>
    };
}

#[component]
pub fn ToggleButton(
    value: ReadSignal<bool>,
    set_value: WriteSignal<bool>,
    children: Children,
) -> impl IntoView {
    return view! {
        <button
            type="button"
            class=BUTTON_CLASS
            aria-pressed=move || value.get()
            on:click=move |_| {
                set_value.update(|value| *value = !*value);
            }
        >
            {children()}
        </button>
    };
}

#[component]
pub fn ActionButton<F, Fut>(action: F, children: Children) -> impl IntoView
where
    F: Fn() -> Fut + 'static,
    Fut: Future<Output = ()> + 'static,
{
    view! {
        <button
            type="button"
            class=BUTTON_CLASS
            on:click=move |_| {
                leptos::task::spawn_local_scoped(action());
            }
        >
            {children()}
        </button>
    }
}
