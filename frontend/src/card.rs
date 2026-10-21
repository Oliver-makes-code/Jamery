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
    view! {
        <a
            href=card.link
            class="flex flex-col items-center gap-3
                bg-[#73A1D6] p-4 transition
                hover:bg-[#93bedb] min-h-[256px] min-w-[256px]
                shadow-[8px_8px_0px_0px_rgba(0,0,0,0.25)]
                text-center"
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
                <img src=icon alt="" class="w-full h-40 object-contain" />
            })}

            {(!card.authors.is_empty()).then(|| view! {
                <p class="mt-auto text-sm">
                    {format!("by {}", card.authors.join(", "))}
                </p>
            })}
        </a>
    }
}

#[component]
pub fn ShowCards(cards: Vec<Card>) -> impl IntoView {
    view! {
        <div class="justify-center flex flex-wrap items-stretch gap-4">
            {cards.into_iter().map(|card| view! {
                <ShowCard card />
            }).collect_view()}
        </div>
    }
}
