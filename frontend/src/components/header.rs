use leptos::prelude::*;

#[component]
pub fn HeaderBlock(title: String, #[prop(optional)] children: Option<Children>) -> impl IntoView {
    return view! {
        <div class="w-full px-4 pb-4 mt-2 sm:px-0">
            <div class="
                mx-auto w-full sm:w-5/8
                bg-[#CE8EDE] px-4 py-2
                shadow-[8px_8px_0px_0px_rgba(0,0,0,0.25)]
            ">
                <div class="flex items-start gap-4">
                    <h1 class="min-w-0 flex-1 break-words text-4xl text-left">
                        {title}
                    </h1>

                    {children.map(|children| view! {
                        <div class="flex shrink-0 flex-col items-end justify-end gap-4">
                            {children()}
                        </div>
                    })}
                </div>
            </div>
        </div>
    };
}
