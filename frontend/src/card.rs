use leptos::prelude::*;

pub struct Card {
    pub title: String,
    pub link: String,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
}

#[component]
pub fn ShowCard(card: Card) -> impl IntoView {
    return view! {
        <a
            href=card.link
            class="
                flex flex-col items-center gap-3
                w-full min-h-[256px] flex-[1_1_256px] max-w-none
                bg-[#73A1D6] px-4 py-2
                shadow-[8px_8px_0px_0px_rgba(0,0,0,0.25)]
                text-center
                transition hover:bg-[#93BEDB]
            "
        >
            <h2 class="text-lg font-bold">
                {card.title}
            </h2>

            {card.description.map(|description| view! {
                <p class="text-sm">
                    {description}
                </p>
            })}

            {card.icon.map(|icon| view! {
                <img src=icon alt="" class="h-[192px] object-contain aspect-square" />
            })}

            {(!card.authors.is_empty()).then(|| view! {
                <p class="mt-auto text-sm">
                    {format!("by {}", card.authors.join(", "))}
                </p>
            })}
        </a>
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
