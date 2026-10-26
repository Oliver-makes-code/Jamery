use leptos::prelude::*;

const TEXT_INPUT_CLASS: &str = "
    my-0 box-border w-full
    font-[inherit] leading-normal
    px-4 py-2 bg-[#EBB7D7] text-black
    shadow-[8px_8px_0px_0px_rgba(0,0,0,0.25)]
    placeholder:text-black/60
    hover:bg-[#F1DBE6]
    focus:bg-[#F1DBE6]
    focus:outline-none focus:ring focus:ring-inset focus:ring-black
    resize-none
    transition
";

#[component]
pub fn TextInput(
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    label: String,
    id: String,
    #[prop(default = String::new())] placeholder: String,
    #[prop(default = false)] required: bool,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    return view! {
        <div class="w-full">
            <label
                for=id.clone()
                class="block mb-2 font-medium text-black"
            >
                {label}
            </label>
            <input
                id=id
                type="text"
                class=TEXT_INPUT_CLASS
                prop:value=move || value.get()
                placeholder=placeholder
                required=required
                disabled=disabled
                on:input=move |ev| {
                    set_value.set(event_target_value(&ev));
                }
            />
        </div>
    };
}

#[component]
pub fn TextArea(
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    label: String,
    id: String,
    #[prop(default = String::new())] placeholder: String,
    #[prop(default = 6)] rows: u32,
    #[prop(default = false)] required: bool,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    return view! {
        <div class="w-full">
            <label
                for=id.clone()
                class="block mb-2 font-medium text-black"
            >
                {label}
            </label>
            <textarea
                id=id
                class=TEXT_INPUT_CLASS
                prop:value=move || value.get()
                placeholder=placeholder
                rows=rows
                required=required
                disabled=disabled
                on:input=move |ev| {
                    set_value.set(event_target_value(&ev));
                }
            />
        </div>
    };
}
