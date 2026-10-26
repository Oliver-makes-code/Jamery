use leptos::prelude::*;

use crate::components::to_kebab_case;

const FILE_INPUT_CLASS: &str = "
    my-0 box-border w-full
    font-[inherit] leading-normal
    bg-[#EBB7D7] text-black
    file:mr-4 file:border-0
    file:px-4 file:py-2
    file:bg-transparent file:text-black
    file:transition
    hover:bg-[#F1DBE6]
    focus:bg-[#F1DBE6]
    focus:outline-none
    focus-visible:outline-2 focus-visible:outline-offset-2
    focus-visible:outline-black
    disabled:opacity-50
    transition
";

const FILE_INPUT_WRAPPER_CLASS: &str = "w-full shadow-[8px_8px_0px_0px_rgba(0,0,0,0.25)]";

#[component]
fn FilePicker(
    id: String,
    label: String,
    #[prop(default = false)] required: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] multiple: bool,
    #[prop(default = String::new())] accept: String,
    on_change: Callback<Vec<web_sys::File>>,
) -> impl IntoView {
    return view! {
        <div class="w-full">
            <label
                for=id.clone()
                class="block mb-2 font-medium text-black"
            >
                {label}
            </label>

            <div class=FILE_INPUT_WRAPPER_CLASS>
                <input
                    id=id
                    type="file"
                    class=FILE_INPUT_CLASS
                    required=required
                    disabled=disabled
                    multiple=multiple
                    accept=accept
                    on:change=move |ev| {
                        let input: web_sys::HtmlInputElement = event_target(&ev);
                        let mut files = Vec::new();

                        if let Some(file_list) = input.files() {
                            for index in 0..file_list.length() {
                                if let Some(file) = file_list.get(index) {
                                    files.push(file);
                                }
                            }
                        }

                        on_change.run(files);
                    }
                />
            </div>
        </div>
    };
}

#[component]
pub fn SingleFileInput(
    set_value: WriteSignal<Option<web_sys::File>>,
    label: String,
    #[prop(default = false)] required: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] accept: String,
) -> impl IntoView {
    let input_id = to_kebab_case(&label);

    return view! {
        <FilePicker
            id=input_id
            label=label
            required=required
            disabled=disabled
            accept=accept
            on_change=Callback::new(move |files: Vec<web_sys::File>| {
                set_value.set(files.into_iter().next());
            })
        />
    };
}

#[component]
pub fn MultiFileInput(
    set_value: WriteSignal<Vec<web_sys::File>>,
    label: String,
    #[prop(default = false)] required: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] accept: String,
) -> impl IntoView {
    let input_id = to_kebab_case(&label);

    return view! {
        <FilePicker
            id=input_id
            label=label
            required=required
            disabled=disabled
            multiple=true
            accept=accept
            on_change=Callback::new(move |files| {
                set_value.set(files);
            })
        />
    };
}
