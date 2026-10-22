use leptos::prelude::*;

#[component]
pub fn Block(children: Children) -> impl IntoView {
    return view! {
        <div class="w-full px-4 pb-4 sm:px-0">
            <div class="
                mx-auto w-full sm:w-5/8
                bg-[#AF76D4] px-4 py-2
                shadow-[8px_8px_0px_0px_rgba(0,0,0,0.25)]
                text-center space-y-4
            ">
                {children()}
            </div>
        </div>
    };
}
