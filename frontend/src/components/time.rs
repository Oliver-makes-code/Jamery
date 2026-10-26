use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use leptos::prelude::*;

use crate::components::to_kebab_case;

const TIME_INPUT_CLASS: &str = "
    my-0 box-border w-full
    font-[inherit] leading-normal
    px-4 py-2 bg-[#EBB7D7] text-black
    shadow-[8px_8px_0px_0px_rgba(0,0,0,0.25)]
    hover:bg-[#F1DBE6]
    focus:bg-[#F1DBE6]
    focus:outline-none focus:ring focus:ring-inset focus:ring-black
    disabled:opacity-50
    transition
";

#[component]
pub fn TimeInput(
    value: ReadSignal<DateTime<Utc>>,
    set_value: WriteSignal<DateTime<Utc>>,
    label: String,
    #[prop(default = false)] required: bool,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let input_id = to_kebab_case(&label);

    return view! {
        <div class="w-full">
            <label
                for=input_id.clone()
                class="block mb-2 font-medium text-black"
            >
                {label}
            </label>

            <input
                id=input_id
                type="datetime-local"
                step="1"
                class=TIME_INPUT_CLASS
                prop:value=move || {
                    value
                        .get()
                        .with_timezone(&Local)
                        .format("%Y-%m-%dT%H:%M:%S")
                        .to_string()
                }
                required=required
                disabled=disabled
                on:input=move |ev| {
                    let input_value = event_target_value(&ev);

                    let parsed = NaiveDateTime::parse_from_str(
                        &input_value,
                        "%Y-%m-%dT%H:%M:%S",
                    )
                    .or_else(|_| {
                        NaiveDateTime::parse_from_str(
                            &input_value,
                            "%Y-%m-%dT%H:%M",
                        )
                    });

                    if let Ok(naive) = parsed {
                        if let Some(local) = Local.from_local_datetime(&naive).single() {
                            set_value.set(local.with_timezone(&Utc));
                        }
                    }
                }
            />
        </div>
    };
}
