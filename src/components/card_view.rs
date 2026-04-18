use dioxus::prelude::*;
use crate::data::{Card, CostKind};
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
    let _ink = lane.ink();
    let _bg = lane.bg();
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
    let foil_a = tilt_px() * 60.0;
    let foil_b = foil_a + 10.0;
    let foil_c = foil_a + 18.0;
    let foil_d = foil_a + 30.0;
    let no_str = format!("{:03}", card.no);
    // In MTG, only Creatures have power/toughness. Frameworks are our creatures.
    let has_pt = card.card_type.starts_with("Framework");

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
                style: "transform:rotateX({tx}deg) rotateY({ty}deg) translateZ(0); transition:{transition}; background:linear-gradient(160deg, {hex} 0%, {hex}cc 25%, #1a0f06 50%, {hex}cc 75%, {hex} 100%); box-shadow:0 18px 40px -14px rgba(0,0,0,.9), 0 0 0 1px rgba(0,0,0,.85), inset 0 1px 0 rgba(255,255,255,.35), 0 0 24px -6px {hex}88, {glow};",

                // Metallic sheen overlay (gives lane color a foil-like depth)
                div {
                    style: "position:absolute; inset:0; border-radius:inherit; pointer-events:none; background:linear-gradient(135deg, rgba(255,255,255,.28) 0%, transparent 25%, transparent 70%, rgba(0,0,0,.35) 100%); mix-blend-mode:overlay;",
                }

                div {
                    class: "tcg-card-inner",
                    style: "background:linear-gradient(180deg, #1a1208, #0a0604);",

                    // Title bar - tinted with lane color
                    div {
                        class: "tcg-bar",
                        style: "display:flex; align-items:center; justify-content:space-between; padding:5px 9px; background:linear-gradient(180deg, {hex} 0%, {hex}dd 50%, {hex}aa 100%); border-bottom:1px solid rgba(0,0,0,.5); box-shadow:inset 0 1px 0 rgba(255,255,255,.4), inset 0 -1px 0 rgba(0,0,0,.3), 0 1px 0 rgba(0,0,0,.4);",
                        div {
                            style: "font-family:'VT323','JetBrains Mono',monospace; font-weight:400; font-size:21px; letter-spacing:0.02em; color:#0a0604; text-shadow:0 1px 0 rgba(255,255,255,.35); white-space:nowrap; overflow:hidden; text-overflow:ellipsis; line-height:1;",
                            "{card_inner.name}"
                        }
                        div { style: "display:flex; gap:3px; flex-shrink:0; margin-left:6px;",
                            for cost in card_inner.cost.iter() {
                                CostPip { kind: *cost, size: 18.0 }
                            }
                        }
                    }

                    // Art window with thin gold border
                    div {
                        class: "tcg-art-window",
                        style: "height:148px; position:relative; overflow:hidden; background:#0b0906; margin:6px 8px 0; box-shadow:inset 0 0 0 1px #6a4a20, 0 0 0 1px rgba(0,0,0,.6), 0 2px 4px rgba(0,0,0,.5);",
                        if card_inner.image_url.is_some() {
                            ImageArt {
                                src: card_inner.image_url.clone().unwrap(),
                                fallback: card_inner.github_repo.clone(),
                                lane: lane,
                                is_repo_card: card_inner.is_logo_card,
                                card_bg: card_inner.card_bg.clone(),
                            }
                        } else {
                            CardArtSvg { seed: card_inner.name.clone(), lane: lane }
                        }
                        if holo {
                            div {
                                style: "position:absolute; inset:0; background:radial-gradient(280px 140px at {holo_px}% {holo_py}%, rgba(255,255,255,.22), transparent 55%); mix-blend-mode:screen; pointer-events:none;",
                            }
                        }
                    }

                    // Type line - tinted with lane color
                    div {
                        class: "tcg-bar",
                        style: "display:flex; align-items:center; justify-content:space-between; padding:4px 9px; margin:6px 8px 0; background:linear-gradient(180deg, {hex}dd 0%, {hex}99 100%); box-shadow:inset 0 0 0 1px rgba(0,0,0,.5), inset 0 1px 0 rgba(255,255,255,.35); font-family:'JetBrains Mono',monospace; font-size:10px; letter-spacing:0.06em; color:#0a0604; text-transform:uppercase; font-weight:700; text-shadow:0 1px 0 rgba(255,255,255,.25);",
                        span { style: "white-space:nowrap; overflow:hidden; text-overflow:ellipsis;", "{card_inner.card_type}" }
                        span {
                            style: "display:inline-block; width:11px; height:11px; border-radius:2px; background:radial-gradient(circle at 30% 30%, {gem}, {hex} 70%, #1a0f06); box-shadow:inset 0 0 0 1px rgba(0,0,0,.6), inset 0 1px 1px rgba(255,255,255,.6), 0 0 6px {gem}88; transform:rotate(45deg); flex-shrink:0; margin-left:6px;",
                        }
                    }

                    // Parchment text box (white/cream with dark text)
                    div {
                        class: "tcg-textbox",
                        style: "flex:1; padding:9px 11px; margin:5px 8px 0; box-shadow:inset 0 0 0 1px #6a4a20; font-family:'JetBrains Mono',monospace; font-size:10.5px; line-height:1.5; display:flex; flex-direction:column; gap:7px; overflow:hidden; position:relative;",
                        for ability in card_inner.abilities.iter() {
                            div { style: "display:flex; gap:6px; position:relative; z-index:1;",
                                span {
                                    style: "font-family:'JetBrains Mono',monospace; font-size:9px; letter-spacing:0.1em; font-weight:700; color:#5a3a18; text-transform:uppercase; flex-shrink:0; padding-top:1px;",
                                    "{ability.label}"
                                }
                                if let Some(ref text) = ability.text {
                                    span { style: "flex:1; color:#1a1208;", "{text}" }
                                }
                            }
                        }
                        if let Some(ref flavor) = card_inner.flavor {
                            div {
                                style: "margin-top:auto; padding-top:7px; border-top:1px solid rgba(106,74,32,.35); color:#4a3418; font-size:10px; line-height:1.45; font-style:italic; position:relative; z-index:1;",
                                "{flavor}"
                                if let Some(ref by) = card_inner.flavor_by {
                                    div {
                                        style: "text-align:right; font-size:9px; color:#6a4a20; margin-top:2px; opacity:.9; font-style:normal;",
                                        "— {by}"
                                    }
                                }
                            }
                        }
                    }

                    // Bottom info strip - collector info + P/T badge on the right
                    div {
                        style: "display:flex; align-items:center; justify-content:space-between; padding:3px 9px 4px; margin:5px 8px 0; font-family:'JetBrains Mono',monospace; font-size:8px; letter-spacing:0.08em; font-weight:600; color:#f4ead0; text-shadow:0 1px 0 rgba(0,0,0,.7); gap:6px;",
                        span { style: "display:inline-flex; align-items:center; gap:4px; white-space:nowrap; overflow:hidden; text-overflow:ellipsis;",
                            "{no_str}/034 · {rar_label} · rust-web"
                        }
                        if has_pt {
                            span {
                                class: "tcg-pt-badge",
                                style: "background:linear-gradient(160deg, {hex} 0%, {hex}cc 50%, {hex}88 100%); color:#0a0604; text-shadow:0 1px 0 rgba(255,255,255,.35); border:1px solid rgba(0,0,0,.6);",
                                "{card_inner.power}/{card_inner.toughness}"
                            }
                        }
                    }

                    // Foil shimmer overlay for rare/mythic
                    if holo {
                        div {
                            class: "tcg-foil",
                            style: "background:linear-gradient(115deg, transparent {foil_a}%, rgba(255,255,255,.22) {foil_b}%, rgba(255,200,180,.14) {foil_c}%, rgba(180,200,255,.16) {foil_d}%, transparent {foil_d}%);",
                        }
                    }
                }
            }
        }
    }
}
