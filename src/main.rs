use dioxus::prelude::*;

mod data;
mod components;

use data::*;
use components::*;

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut filter = use_signal(|| "all".to_string());
    let mut zoomed_card: Signal<Option<Card>> = use_signal(|| None);
    let cards = use_hook(all_cards);
    let counts = use_hook(|| lane_counts(&all_cards()));

    let filtered: Vec<Card> = if filter() == "all" {
        cards.clone()
    } else {
        cards.iter().filter(|c| c.lane.id() == filter().as_str()).cloned().collect()
    };

    let is_open = zoomed_card.read().is_some();

    rsx! {
        document::Stylesheet { href: CSS }

        Nav {}
        HeroFerris {}
        HeroHeader {}

        // Packs filter bar
        div { class: "packs", id: "packs",
            for pack in all_packs() {
                {
                    let id = pack.id.to_string();
                    let is_selected = filter() == id;
                    let count = counts.get(&id).copied().unwrap_or(0);
                    let id_clone = id.clone();
                    rsx! {
                        button {
                            class: "pack",
                            aria_selected: "{is_selected}",
                            onclick: move |_| filter.set(id_clone.clone()),
                            span { class: "swatch", style: "background: {pack.swatch};" }
                            "{pack.name}"
                            span { class: "n", "{count}" }
                        }
                    }
                }
            }
        }

        // Card board
        main {
            div { class: "board", id: "board",
                for card in filtered.iter() {
                    {
                        let mut zoomed = zoomed_card;
                        rsx! {
                            CardView {
                                key: "{card.no}",
                                card: card.clone(),
                                on_click: move |c: Card| zoomed.set(Some(c)),
                            }
                        }
                    }
                }
            }
        }

        // Zoom overlay
        div {
            class: if is_open { "zoom-backdrop open" } else { "zoom-backdrop" },
            id: "zoom",
            onclick: move |_| zoomed_card.set(None),
            if let Some(ref card) = *zoomed_card.read() {
                ZoomWrap { card: card.clone(), on_close: move |_| zoomed_card.set(None) }
            }
        }
        if is_open {
            div { class: "close-hint", id: "closeHint",
                kbd { "ESC" }
                " · click away to close"
            }
        }

        footer {
            div { class: "line", "> NOT_AFFILIATED · FAN_MADE · DRAFT_FORMAT_LEGAL <" }
        }
    }
}
