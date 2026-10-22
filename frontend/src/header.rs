use leptos::prelude::*;

#[component]
pub fn HeaderBlock(title: String, #[prop(optional)] children: Option<Children>) -> impl IntoView {
    return view! {
        <div class="w-full px-4 pb-4 sm:px-0">
            <div class="
                mx-auto w-full sm:w-5/8
                bg-[#CE8EDE] px-4 py-2
                shadow-[8px_8px_0px_0px_rgba(0,0,0,0.25)]
            ">

                <div class="flex flex-wrap items-center justify-between gap-4">
                    <h1 class="text-4xl text-left">
                        {title}
                    </h1>

                    {children.map(|children| view! {
                        <div class="flex flex-wrap justify-start gap-2">
                            {children()}
                        </div>
                    })}
                </div>

            </div>
        </div>
    };
}
