use leptos::prelude::*;

pub enum ButtonBehavior {
    Link(String),
    Toggle(WriteSignal<bool>),
}

#[component]
pub fn BlockButton(behavior: ButtonBehavior, children: Children) -> impl IntoView {
    let class = "
        my-0 box-border inline-flex items-center justify-center align-middle
        font-[inherit] leading-normal
        px-4 py-2 bg-[#EBB7D7] text-black
        shadow-[8px_8px_0px_0px_rgba(0,0,0,0.25)]
        hover:bg-[#F1CEE4] transition
        [&:not(:first-child)]:ml-4
    ";

    match behavior {
        ButtonBehavior::Link(href) => {
            return view! {
                <a href=href class=class>
                    {children()}
                </a>
            }
            .into_any();
        }

        ButtonBehavior::Toggle(set_value) => {
            return view! {
                <button type="button" class=class
                    on:click=move |_| {
                        set_value.update(|value| *value = !*value);
                    }
                >
                    {children()}
                </button>
            }
            .into_any();
        }
    }
}
