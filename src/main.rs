use dioxus::prelude::*;

mod data;
mod components;

use data::*;
use components::*;

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(
            dioxus::fullstack::Config::new()
                .addr(std::net::SocketAddr::from(([0, 0, 0, 0], 8080)))
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    let mut filter = use_signal(|| "all".to_string());
    let mut search = use_signal(|| String::new());
    let mut zoomed_card: Signal<Option<Card>> = use_signal(|| None);
    let cards = use_hook(all_cards);
    let counts = use_hook(|| lane_counts(&all_cards()));

    let query = search().to_lowercase();
    let filtered: Vec<Card> = cards.iter().filter(|c| {
        let lane_ok = filter() == "all" || c.lane.id() == filter().as_str();
        let search_ok = query.is_empty()
            || c.name.to_lowercase().contains(&query)
            || c.card_type.to_lowercase().contains(&query)
            || c.lane.name().to_lowercase().contains(&query)
            || c.abilities.iter().any(|a| {
                a.label.to_lowercase().contains(&query)
                || a.text.as_deref().unwrap_or_default().to_lowercase().contains(&query)
            })
            || c.flavor.as_deref().unwrap_or_default().to_lowercase().contains(&query);
        lane_ok && search_ok
    }).cloned().collect();

    let is_open = zoomed_card.read().is_some();

    rsx! {
        document::Stylesheet { href: CSS }

        Nav {}
        HeroFerris {}

        header { class: "hero",
            h1 {
                "rust-web"
                span { class: "amp", "." }
                "com"
                span { class: "cursor", "\u{00a0}" }
            }
        }

        // Search bar
        div { class: "search-bar",
            input {
                r#type: "text",
                placeholder: "search cards…",
                value: "{search}",
                oninput: move |e| search.set(e.value()),
            }
        }

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
            button {
                class: "zoom-close",
                onclick: move |e: Event<MouseData>| { e.stop_propagation(); zoomed_card.set(None); },
                aria_label: "Back",
                "\u{2190}"
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
