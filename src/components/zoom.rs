use dioxus::prelude::*;
use crate::data::{Card, snippet_for};
use super::card_view::CardView;

#[component]
pub fn ZoomWrap(card: Card, on_close: EventHandler<()>) -> Element {
    let lane = card.lane;
    let _hex = lane.hex();
    let _ink = lane.ink();

    rsx! {
        div {
            class: "zoom-wrap",
            onclick: move |e| e.stop_propagation(),

            // Scaled card — links to GitHub repo
            a {
                class: "zoom-card-link",
                href: card.github_repo.as_ref().map(|r| format!("https://github.com/{r}")).unwrap_or_default(),
                target: "_blank",
                rel: "noreferrer",
                style: "display:block; text-decoration:none; color:inherit;",
                CardView { card: card.clone(), on_click: |_: Card| {} }
            }

            // Side panel
            div { class: "zoom-side",
                div { class: "lore-label", style: "margin-top:0;", "{lane.name()}" }
                h2 { "{card.name}" }
                div {
                    style: "font-family:'JetBrains Mono',monospace; font-size:11px; letter-spacing:0.06em; color:#9fb1a6; margin-bottom:14px;",
                    "{card.card_type}"
                }
                div {
                    for ability in card.abilities.iter() {
                        div { style: "margin-bottom:10px;",
                            span {
                                style: "font-family:'JetBrains Mono',monospace; font-size:10px; letter-spacing:0.1em; color:#c9a34a; text-transform:uppercase; margin-right:8px;",
                                "{ability.label}"
                            }
                            if let Some(ref text) = ability.text {
                                span { "{text}" }
                            }
                        }
                    }
                }
                // Code snippet
                {
                    let snippet = snippet_for(&card.name);
                    let mut copied = use_signal(|| false);
                    rsx! {
                        if let Some(snippet) = snippet.as_ref() {
                            div { class: "lore-label", "Hello, world" }
                            div { class: "code-block",
                                div { class: "code-head",
                                    span { class: "dot r" }
                                    span { class: "dot y" }
                                    span { class: "dot g" }
                                    span { class: "file", "{snippet.title}" }
                                    button {
                                        class: "copy-btn",
                                        onclick: {
                                            let code = snippet.code.to_string();
                                            move |_| {
                                                let c = code.clone();
                                                spawn(async move {
                                                    let js = format!(
                                                        "navigator.clipboard.writeText({})",
                                                        serde_json::to_string(&c).unwrap_or_default()
                                                    );
                                                    let _ = document::eval(&js).await;
                                                    copied.set(true);
                                                    #[cfg(target_arch = "wasm32")]
                                                    {
                                                        gloo_timers::future::TimeoutFuture::new(1500).await;
                                                        copied.set(false);
                                                    }
                                                });
                                            }
                                        },
                                        if copied() { "Copied!" } else { "Copy" }
                                    }
                                }
                                pre {
                                    code { "{snippet.code}" }
                                }
                            }
                        }
                    }
                }
                // Flavor
                if let Some(ref flavor) = card.flavor {
                    div { class: "lore-label", "Flavor" }
                    div { style: "font-style:italic; color:#cbbf95; font-size:14px;",
                        "\"{flavor}\""
                        if let Some(ref by) = card.flavor_by {
                            div {
                                style: "font-style:normal; font-size:11px; color:#9fb1a6; margin-top:4px;",
                                "— {by}"
                            }
                        }
                    }
                }
                // GitHub info
                if let Some(ref repo) = card.github_repo {
                    div { class: "lore-label", "GitHub" }
                    div { class: "traits",
                        if let Some(ref stars) = card.stars {
                            span { class: "chip", "★ {stars}" }
                        }
                        a {
                            class: "chip",
                            href: "https://github.com/{repo}",
                            target: "_blank",
                            rel: "noreferrer",
                            style: "text-decoration:none; color:inherit;",
                            "{repo} ↗"
                        }
                    }
                }
            }
        }
    }
}
