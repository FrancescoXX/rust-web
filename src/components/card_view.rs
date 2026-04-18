use dioxus::prelude::*;
use crate::data::{Card, CostKind, Rarity};
use super::card_art::{CardArtSvg, ImageArt};

// ── Cost Pip ────────────────────────────────────────────────────────────────

#[component]
pub fn CostPip(kind: CostKind, size: f64) -> Element {
    let bg = kind.bg_color();
    let ink = kind.ink_color();
    let glyph = kind.glyph();
    rsx! {
        span {
            style: "width:{size}px; height:{size}px; border-radius:50%; background:{bg}; color:{ink}; display:inline-flex; align-items:center; justify-content:center; font-family:'JetBrains Mono',monospace; font-size:{size * 0.55}px; font-weight:700; box-shadow:inset 0 -2px 3px rgba(0,0,0,.2), inset 0 1px 1px rgba(255,255,255,.6), 0 1px 1px rgba(0,0,0,.4); border:1px solid rgba(0,0,0,.35); line-height:1;",
            "{glyph}"
        }
    }
}

// ── Card View ───────────────────────────────────────────────────────────────

#[component]
pub fn CardView(card: Card, on_click: EventHandler<Card>) -> Element {
    let mut tilt_x = use_signal(|| 0.0f64);
    let mut tilt_y = use_signal(|| 0.0f64);
    let mut tilt_on = use_signal(|| false);
    let mut tilt_px = use_signal(|| 0.5f64);
    let mut tilt_py = use_signal(|| 0.5f64);

    let lane = card.lane;
    let rar = card.rarity;
    let hex = lane.hex();
    let ink = lane.ink();
    let bg = lane.bg();
    let gem = rar.gem_color();
    let glow = rar.glow();
    let holo = rar.is_holo();
    let rar_label = rar.short();

    let tx = tilt_x();
    let ty = tilt_y();
    let transition = if tilt_on() {
        "transform 60ms linear"
    } else {
        "transform 220ms cubic-bezier(.2,.7,.2,1)"
    };

    let card_inner = card.clone();
    let card_click = card.clone();

    let holo_px = tilt_px() * 100.0;
    let holo_py = tilt_py() * 100.0;
    let no_str = format!("{:03}", card.no);

    rsx! {
        div {
            class: "tcg-card",
            onmousemove: move |e| {
                let coords = e.element_coordinates();
                let px = (coords.x / 280.0).clamp(0.0, 1.0);
                let py = (coords.y / 400.0).clamp(0.0, 1.0);
                tilt_x.set((py - 0.5) * -8.0);
                tilt_y.set((px - 0.5) * 10.0);
                tilt_on.set(true);
                tilt_px.set(px);
                tilt_py.set(py);
            },
            onmouseleave: move |_| {
                tilt_x.set(0.0);
                tilt_y.set(0.0);
                tilt_on.set(false);
                tilt_px.set(0.5);
                tilt_py.set(0.5);
            },
            onclick: move |_| on_click.call(card_click.clone()),

            div {
                class: "tcg-card-body",
                style: "transform:rotateX({tx}deg) rotateY({ty}deg) translateZ(0); transition:{transition}; background:{bg}; box-shadow:0 1px 0 rgba(255,180,100,.1) inset, 0 -2px 0 rgba(0,0,0,.5) inset, 0 14px 32px -12px rgba(0,0,0,.85), {glow};",

                div {
                    class: "tcg-card-inner",
                    style: "border-color:{ink}22;",

                    // Title bar
                    div {
                        style: "display:flex; align-items:center; justify-content:space-between; padding:6px 8px; background:linear-gradient(180deg, {hex}55, {hex}15); border-bottom:1px solid {hex}55;",
                        div {
                            style: "font-family:'VT323','JetBrains Mono',monospace; font-weight:400; font-size:22px; letter-spacing:0.02em; color:{ink}; text-shadow:0 0 8px {hex}66, 0 1px 0 rgba(0,0,0,.6); white-space:nowrap; overflow:hidden; text-overflow:ellipsis; line-height:1;",
                            "{card_inner.name}"
                        }
                        div { style: "display:flex; gap:3px;",
                            for cost in card_inner.cost.iter() {
                                CostPip { kind: *cost, size: 18.0 }
                            }
                        }
                    }

                    // Art
                    div {
                        style: "height:150px; position:relative; border-bottom:1px solid {ink}22; overflow:hidden; background:#0b0906;",
                        if card_inner.image_url.is_some() {
                            ImageArt {
                                src: card_inner.image_url.clone().unwrap(),
                                fallback: card_inner.github_repo.clone(),
                                lane: lane,
                                is_repo_card: card_inner.is_logo_card,
                            }
                        } else {
                            CardArtSvg { seed: card_inner.name.clone(), lane: lane }
                        }
                        if holo {
                            div {
                                style: "position:absolute; inset:0; background:radial-gradient(240px 120px at {holo_px}% {holo_py}%, rgba(255,255,255,.18), transparent 60%); mix-blend-mode:screen; pointer-events:none;",
                            }
                        }
                    }

                    // Type line
                    div {
                        style: "display:flex; align-items:center; justify-content:space-between; padding:5px 8px; background:linear-gradient(180deg, {hex}33, {hex}0d); border-bottom:1px solid {hex}44; font-family:'JetBrains Mono',monospace; font-size:10px; letter-spacing:0.08em; color:{ink}; text-transform:uppercase; font-weight:600;",
                        span { "{card_inner.card_type}" }
                        span {
                            style: "display:inline-block; width:10px; height:10px; border-radius:2px; background:{gem}; box-shadow:inset 0 0 0 1px rgba(0,0,0,.5), inset 0 1px 1px rgba(255,255,255,.4); transform:rotate(45deg);",
                        }
                    }

                    // Text box
                    div {
                        style: "flex:1; padding:8px 10px; background:linear-gradient(180deg, #1a0f07, #0f0804); color:#e8c896; font-family:'JetBrains Mono',monospace; font-size:10.5px; line-height:1.45; display:flex; flex-direction:column; gap:6px; overflow:hidden; border-top:1px solid {hex}22;",
                        for ability in card_inner.abilities.iter() {
                            div { style: "display:flex; gap:6px;",
                                span {
                                    style: "font-family:'JetBrains Mono',monospace; font-size:9px; letter-spacing:0.1em; font-weight:700; color:{hex}; text-transform:uppercase; flex-shrink:0; padding-top:1px;",
                                    "{ability.label}"
                                }
                                if let Some(ref text) = ability.text {
                                    span { style: "flex:1;", "{text}" }
                                }
                            }
                        }
                        if let Some(ref flavor) = card_inner.flavor {
                            div {
                                style: "margin-top:auto; padding-top:6px; border-top:1px dashed {hex}44; color:#a07448; font-size:10px; line-height:1.4;",
                                span { style: "color:{hex}; opacity:.7;", ">" }
                                " {flavor}"
                                if let Some(ref by) = card_inner.flavor_by {
                                    div {
                                        style: "text-align:right; font-size:9px; color:#7a5a2c; margin-top:2px; opacity:.75;",
                                        "— {by}"
                                    }
                                }
                            }
                        }
                    }

                    // Stat bar
                    div {
                        style: "display:flex; align-items:center; justify-content:space-between; padding:4px 8px; background:#050301; border-top:1px solid {hex}44; font-family:'JetBrains Mono',monospace; font-size:9.5px; letter-spacing:0.1em; font-weight:500; color:#8a6a42;",
                        span { style: "display:inline-flex; align-items:center; gap:5px;",
                            svg { width: "11", height: "7", view_box: "0 0 100 64", style: "flex-shrink:0; opacity:.75;",
                                ellipse { cx: "50", cy: "36", rx: "32", ry: "18", fill: "{hex}", stroke: "#2a1606", stroke_width: "3" }
                                rect { x: "4", y: "28", width: "14", height: "10", fill: "{hex}", stroke: "#2a1606", stroke_width: "3" }
                                rect { x: "82", y: "28", width: "14", height: "10", fill: "{hex}", stroke: "#2a1606", stroke_width: "3" }
                                circle { cx: "42", cy: "28", r: "3", fill: "#fff" }
                                circle { cx: "58", cy: "28", r: "3", fill: "#fff" }
                                circle { cx: "42", cy: "28", r: "1.2", fill: "#2a1606" }
                                circle { cx: "58", cy: "28", r: "1.2", fill: "#2a1606" }
                            }
                            "rust-web.com · {no_str}/034 · {rar_label}"
                        }
                        span {
                            style: "background:linear-gradient(180deg, {hex}, {hex}99); color:#0a0603; font-weight:700; padding:2px 8px; border-radius:2px; font-size:11px; letter-spacing:0.02em; box-shadow:inset 0 -1px 1px rgba(0,0,0,.35), inset 0 1px 0 rgba(255,220,180,.4), 0 0 10px {hex}55; font-family:'VT323',monospace;",
                            "{card_inner.power}/{card_inner.toughness}"
                        }
                    }
                }
            }
        }
    }
}
