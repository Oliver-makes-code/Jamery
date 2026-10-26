use leptos::prelude::*;

const TEXT_INPUT_CLASS: &str = "
    my-0 box-border w-full
    font-[inherit] leading-normal
    px-4 py-2 bg-[#EBB7D7] text-black
    shadow-[8px_8px_0px_0px_rgba(0,0,0,0.25)]
    placeholder:text-black/60
    hover:bg-[#F1DBE6]
    focus:bg-[#F1DBE6]
    focus:outline-none focus:ring-2 focus:ring-inset focus:ring-black
    resize-none
    transition
";

fn to_kebab_case(value: &str) -> String {
    let mut result = String::new();

    for (index, ch) in value.chars().enumerate() {
        if ch.is_ascii_alphanumeric() {
            if ch.is_uppercase() && index > 0 {
                result.push('-');
            }

            result.extend(ch.to_lowercase());
        } else if !result.is_empty() && !result.ends_with('-') {
            result.push('-');
        }
    }

    return result.trim_matches('-').to_string();
}

#[component]
pub fn TextInput(
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    label: String,
    #[prop(default = String::new())] placeholder: String,
    #[prop(default = false)] required: bool,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let id = to_kebab_case(&label);

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
    #[prop(default = String::new())] placeholder: String,
    #[prop(default = 6)] rows: u32,
    #[prop(default = false)] required: bool,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let id = to_kebab_case(&label);

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
