use chrono::{DateTime, Utc};
use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct Card {
    pub title: Signal<String>,
    pub link: Signal<Option<String>>,
    pub authors: Signal<Vec<String>>,
    pub summary: Signal<Option<String>>,
    pub start_date: Signal<Option<DateTime<Utc>>>,
    pub end_date: Signal<Option<DateTime<Utc>>>,
    pub icon: Signal<Option<String>>,
}

const CARD_CLASS: &str = "
    flex flex-col items-center gap-3
    w-full min-h-[256px] flex-[1_1_256px] max-w-none
    h-auto
    bg-[#73A1D6] px-4 py-2
    shadow-[8px_8px_0px_0px_rgba(0,0,0,0.25)]
    text-center
    transition hover:bg-[#93BEDB]
    wrap-anywhere
";

#[component]
pub fn ShowCard(card: Card) -> impl IntoView {
    let children = view! {
        <h2 class="text-lg font-bold">
            {move || card.title.get()}
        </h2>

        {move || card.summary.get().map(|description| {
            view! {
                <p class="text-sm">
                    {description}
                </p>
            }
        })}

        {move || card.icon.get().map(|icon| {
            view! {
                <img
                    src=icon
                    alt=""
                    class="h-[192px] object-contain aspect-square"
                />
            }
        })}

        {move || {
            let authors = card.authors.get();
            let start_date = card.start_date.get();
            let end_date = card.end_date.get();

            (!authors.is_empty() || start_date.is_some() || end_date.is_some()).then(|| {
                view! {
                    <div class="mt-auto text-sm">
                        {(!authors.is_empty()).then(|| view! {
                            <p>
                                {format!("by {}", authors.join(", "))}
                            </p>
                        })}

                        {start_date.map(|date| view! {
                            <p>
                                {format!("Begins {}", date.with_timezone(&chrono::Local).format("%b %d, %Y"))}
                            </p>
                        })}

                        {end_date.map(|date| view! {
                            <p>
                                {format!("Ends {}", date.with_timezone(&chrono::Local).format("%b %d, %Y"))}
                            </p>
                        })}
                    </div>
                }
            })
        }}
    };

    return view! {
        {move || match card.link.get() {
            Some(link) => view! {
                <a
                    href=link
                    class=CARD_CLASS
                >
                    {children.clone()}
                </a>
            }.into_any(),
            None => view! {
                <div class=CARD_CLASS>
                    {children.clone()}
                </div>
            }.into_any(),
        }}
    };
}

#[component]
pub fn ShowCards(cards: Vec<Card>) -> impl IntoView {
    return view! {
        <div class="w-full px-4 pb-4 sm:w-5/8 sm:px-0 mx-auto">
            <div class="flex flex-wrap justify-center gap-4">
                {cards.into_iter().map(|card| view! {
                    <div class="w-full sm:w-[calc(50%-0.5rem)]">
                        <ShowCard card />
                    </div>
                }).collect_view()}
            </div>
        </div>
    };
}
