// Rust Web TCG — Dioxus App
// A faithful port of the React/JSX static site to Dioxus 0.7 web.

use dioxus::prelude::*;

mod data;
use data::*;

static CSS: Asset = asset!("/assets/main.css");
static FERRIS_IMG: Asset = asset!("/assets/ferris-web.png");

fn main() {
    dioxus::launch(App);
}

// ── App ─────────────────────────────────────────────────────────────────────

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

// ── Nav ─────────────────────────────────────────────────────────────────────

#[component]
fn Nav() -> Element {
    rsx! {
        nav { class: "nav",
            a { class: "brand", href: "#",
                FerrisIcon { width: 34.0, height: 22.0 }
                "rust-web"
                span { class: "dot", "." }
                "com"
            }
            div { class: "nav-links",
                a { href: "#", class: "active", "[ Cards ]" }
                a { href: "#", "[ Learn ]" }
                a { href: "#", "[ Draft ]" }
                a { href: "https://crates.io", target: "_blank", rel: "noreferrer", "[ Crates ↗ ]" }
            }
        }
    }
}

// ── Ferris SVG icon ─────────────────────────────────────────────────────────

#[component]
fn FerrisIcon(width: f64, height: f64) -> Element {
    rsx! {
        svg {
            class: "ferris",
            width: "{width}",
            height: "{height}",
            view_box: "0 0 100 64",
            // shadow
            ellipse { cx: "50", cy: "58", rx: "30", ry: "3", fill: "#000", opacity: ".45" }
            // left claw
            g {
                rect { x: "4", y: "28", width: "14", height: "10", rx: "2", fill: "#e8852b", stroke: "#2a1606", stroke_width: "1.5" }
                polygon { points: "4,28 10,22 14,28", fill: "#f2b866", stroke: "#2a1606", stroke_width: "1.5" }
                polygon { points: "4,38 10,44 14,38", fill: "#b35a1a", stroke: "#2a1606", stroke_width: "1.5" }
            }
            // right claw
            g {
                rect { x: "82", y: "28", width: "14", height: "10", rx: "2", fill: "#e8852b", stroke: "#2a1606", stroke_width: "1.5" }
                polygon { points: "82,28 86,22 96,28", fill: "#f2b866", stroke: "#2a1606", stroke_width: "1.5" }
                polygon { points: "82,38 86,44 96,38", fill: "#b35a1a", stroke: "#2a1606", stroke_width: "1.5" }
            }
            // legs
            g { stroke: "#2a1606", stroke_width: "1.5", fill: "#c26b20",
                rect { x: "22", y: "42", width: "4", height: "10" }
                rect { x: "32", y: "44", width: "4", height: "10" }
                rect { x: "42", y: "46", width: "4", height: "10" }
                rect { x: "54", y: "46", width: "4", height: "10" }
                rect { x: "64", y: "44", width: "4", height: "10" }
                rect { x: "74", y: "42", width: "4", height: "10" }
            }
            // body
            ellipse { cx: "50", cy: "36", rx: "32", ry: "18", fill: "#e8852b", stroke: "#2a1606", stroke_width: "2" }
            ellipse { cx: "50", cy: "32", rx: "28", ry: "12", fill: "#f2b866", opacity: ".35" }
            // eye stalks
            line { x1: "42", y1: "20", x2: "42", y2: "14", stroke: "#2a1606", stroke_width: "2" }
            line { x1: "58", y1: "20", x2: "58", y2: "14", stroke: "#2a1606", stroke_width: "2" }
            circle { cx: "42", cy: "12", r: "3.5", fill: "#fff", stroke: "#2a1606", stroke_width: "1.5" }
            circle { cx: "58", cy: "12", r: "3.5", fill: "#fff", stroke: "#2a1606", stroke_width: "1.5" }
            circle { cx: "42", cy: "12", r: "1.4", fill: "#2a1606" }
            circle { cx: "58", cy: "12", r: "1.4", fill: "#2a1606" }
            // mouth
            path { d: "M 44 38 Q 50 42 56 38", fill: "none", stroke: "#2a1606", stroke_width: "1.8", stroke_linecap: "round" }
            // highlight
            ellipse { cx: "40", cy: "28", rx: "6", ry: "3", fill: "#fff", opacity: ".4" }
        }
    }
}

// ── Hero ────────────────────────────────────────────────────────────────────

#[component]
fn HeroFerris() -> Element {
    rsx! {
        div { class: "hero-ferris", aria_hidden: "true",
            img { src: FERRIS_IMG, alt: "" }
        }
    }
}

#[component]
fn HeroHeader() -> Element {
    rsx! {
        header { class: "hero",
            h1 {
                "rust-web"
                span { class: "amp", "." }
                "com"
                span { class: "cursor", "\u{00a0}" }
            }
            p { class: "sub",
                "thirty-four frameworks, runtimes & libraries — printed onto collectible cards."
                br {}
                "pick a pack, hover to tilt, click for the full spec sheet."
            }
        }
    }
}

// ── Cost Pip ────────────────────────────────────────────────────────────────

#[component]
fn CostPip(kind: CostKind, size: f64) -> Element {
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

// ── Card Art (procedural SVG) ───────────────────────────────────────────────

fn det_rand(seed: &str) -> impl FnMut() -> f64 {
    let mut s: u32 = 0;
    for ch in seed.chars() {
        s = s.wrapping_mul(31).wrapping_add(ch as u32);
    }
    move || {
        s = s.wrapping_mul(1664525).wrapping_add(1013904223);
        ((s & 0xffffff) as f64) / (0xffffff as f64)
    }
}

#[component]
fn CardArtSvg(seed: String, lane: Lane) -> Element {
    let mut rand = det_rand(&seed);
    let hex = lane.hex();
    let ink = lane.ink();
    let key = lane.key();
    let palette = [hex, "#f2b866", "#0a0603", ink, "#e8852b"];
    let n = 14 + (rand() * 10.0) as usize;

    let mut shapes: Vec<(String, String)> = Vec::new(); // (tag_type, attributes)
    for i in 0..n {
        let cx = 20.0 + rand() * 260.0;
        let cy = 20.0 + rand() * 140.0;
        let r = 6.0 + rand() * 26.0;
        let kind = (rand() * 4.0) as usize;
        let col = palette[(rand() * palette.len() as f64) as usize % palette.len()];
        let op = 0.35 + rand() * 0.55;
        shapes.push((kind.to_string(), format!("{i},{cx},{cy},{r},{col},{op}")));
    }

    rsx! {
        svg {
            view_box: "0 0 300 180",
            preserve_aspect_ratio: "none",
            style: "display:block; width:100%; height:100%;",
            defs {
                linearGradient { id: "bg-{seed}", x1: "0", x2: "0", y1: "0", y2: "1",
                    stop { offset: "0", stop_color: "{hex}", stop_opacity: "0.35" }
                    stop { offset: "1", stop_color: "#0b0906", stop_opacity: "0.9" }
                }
            }
            rect { width: "300", height: "180", fill: "#0a0603" }
            rect { width: "300", height: "180", fill: "url(#bg-{seed})" }
            // shapes
            for (_idx, (kind_str, data)) in shapes.iter().enumerate() {
                {
                    let parts: Vec<&str> = data.split(',').collect();
                    let cx: f64 = parts[1].parse().unwrap_or(0.0);
                    let cy: f64 = parts[2].parse().unwrap_or(0.0);
                    let r: f64 = parts[3].parse().unwrap_or(0.0);
                    let col = parts[4];
                    let op = parts[5];
                    let kind: usize = kind_str.parse().unwrap_or(0);
                    match kind {
                        0 => rsx! { circle { cx: "{cx}", cy: "{cy}", r: "{r}", fill: "{col}", opacity: "{op}" } },
                        1 => rsx! { rect { x: "{cx - r}", y: "{cy - r}", width: "{r * 2.0}", height: "{r * 2.0}", fill: "{col}", opacity: "{op}" } },
                        2 => {
                            let pts = format!("{cx},{}", cy - r) + &format!(" {},{}", cx + r, cy + r) + &format!(" {},{}", cx - r, cy + r);
                            rsx! { polygon { points: "{pts}", fill: "{col}", opacity: "{op}" } }
                        },
                        _ => rsx! { line { x1: "{cx - r}", y1: "{cy}", x2: "{cx + r}", y2: "{cy}", stroke: "{col}", stroke_width: "2", opacity: "{op}" } },
                    }
                }
            }
            // corner sigil
            g { opacity: "0.9", transform: "translate(252, 150)",
                circle { r: "14", fill: "none", stroke: "{ink}", stroke_opacity: ".6", stroke_width: "1" }
                text {
                    text_anchor: "middle",
                    dominant_baseline: "central",
                    font_family: "JetBrains Mono, monospace",
                    font_size: "12",
                    font_weight: "700",
                    fill: "{ink}",
                    "{key}"
                }
            }
        }
    }
}

// ── Image Art ───────────────────────────────────────────────────────────────

#[component]
fn ImageArt(src: String, fallback: Option<String>, lane: Lane, is_repo_card: bool) -> Element {
    let mut errored = use_signal(|| false);
    let hex = lane.hex();

    let url = if errored() {
        if let Some(ref fb) = fallback {
            format!("https://opengraph.githubassets.com/1/{fb}")
        } else {
            src.clone()
        }
    } else {
        src.clone()
    };

    let is_cover = errored() || is_repo_card;

    let img_style = if is_cover {
        "width:100%; height:100%; object-fit:cover; display:block; filter:saturate(1.05) contrast(1.02);"
    } else {
        "max-width:72%; max-height:72%; width:auto; height:auto; object-fit:contain; display:block; filter:drop-shadow(0 2px 8px rgba(0,0,0,.6));"
    };

    rsx! {
        div {
            style: "position:relative; width:100%; height:100%; background:radial-gradient(circle at 50% 50%, {hex}22, #0a0603 80%); display:flex; align-items:center; justify-content:center; overflow:hidden;",
            img {
                src: "{url}",
                alt: "",
                loading: "lazy",
                onerror: move |_| errored.set(true),
                style: "{img_style}",
            }
            div {
                style: "position:absolute; inset:0; background:radial-gradient(140% 90% at 50% 50%, transparent 55%, rgba(10,6,3,.45) 100%); pointer-events:none;",
            }
        }
    }
}

// ── Card Component ──────────────────────────────────────────────────────────

#[component]
fn CardView(card: Card, on_click: EventHandler<Card>) -> Element {
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
    let rar_label = rar.label().to_uppercase();

    let tx = tilt_x();
    let ty = tilt_y();
    let transition = if tilt_on() {
        "transform 60ms linear"
    } else {
        "transform 220ms cubic-bezier(.2,.7,.2,1)"
    };

    let card_inner = card.clone();
    let card_click = card.clone();

    // Holo position
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

            // Card body with 3D transform
            div {
                style: "width:100%; height:100%; transform:rotateX({tx}deg) rotateY({ty}deg) translateZ(0); transition:{transition}; background:{bg}; padding:10px; position:relative; box-shadow:0 1px 0 rgba(255,180,100,.1) inset, 0 -2px 0 rgba(0,0,0,.5) inset, 0 14px 32px -12px rgba(0,0,0,.85), {glow}; border:1px solid #000; will-change:transform;",

                // Inner frame
                div {
                    style: "position:relative; height:100%; border:1px solid {ink}22; background:linear-gradient(180deg, rgba(255,255,255,.04), rgba(0,0,0,.2)); display:flex; flex-direction:column; overflow:hidden;",

                    // TITLE BAR
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

                    // ART
                    div {
                        style: "height:150px; position:relative; border-bottom:1px solid {ink}22; overflow:hidden; background:#0b0906;",
                        if card_image_url(card_inner.name).is_some() {
                            ImageArt {
                                src: card_image_url(card_inner.name).unwrap().to_string(),
                                fallback: og_fallback(card_inner.name).map(|s| s.to_string()),
                                lane: lane,
                                is_repo_card: card_inner.name == "Leptos",
                            }
                        } else {
                            CardArtSvg { seed: card_inner.name.to_string(), lane: lane }
                        }
                        // Holo sheen
                        if holo {
                            div {
                                style: "position:absolute; inset:0; background:radial-gradient(240px 120px at {holo_px}% {holo_py}%, rgba(255,255,255,.18), transparent 60%); mix-blend-mode:screen; pointer-events:none;",
                            }
                        }
                    }

                    // TYPE LINE
                    div {
                        style: "display:flex; align-items:center; justify-content:space-between; padding:5px 8px; background:linear-gradient(180deg, {hex}33, {hex}0d); border-bottom:1px solid {hex}44; font-family:'JetBrains Mono',monospace; font-size:10px; letter-spacing:0.08em; color:{ink}; text-transform:uppercase; font-weight:600;",
                        span { "{card_inner.card_type}" }
                        span {
                            style: "display:inline-block; width:10px; height:10px; border-radius:2px; background:{gem}; box-shadow:inset 0 0 0 1px rgba(0,0,0,.5), inset 0 1px 1px rgba(255,255,255,.4); transform:rotate(45deg);",
                        }
                    }

                    // TEXT BOX
                    div {
                        style: "flex:1; padding:8px 10px; background:linear-gradient(180deg, #1a0f07, #0f0804); color:#e8c896; font-family:'JetBrains Mono',monospace; font-size:10.5px; line-height:1.45; display:flex; flex-direction:column; gap:6px; overflow:hidden; border-top:1px solid {hex}22;",
                        for ability in card_inner.abilities.iter() {
                            div { style: "display:flex; gap:6px;",
                                span {
                                    style: "font-family:'JetBrains Mono',monospace; font-size:9px; letter-spacing:0.1em; font-weight:700; color:{hex}; text-transform:uppercase; flex-shrink:0; padding-top:1px;",
                                    "{ability.label}"
                                }
                                if let Some(text) = ability.text {
                                    span { style: "flex:1;", "{text}" }
                                }
                            }
                        }
                        if let Some(flavor) = card_inner.flavor {
                            div {
                                style: "margin-top:auto; padding-top:6px; border-top:1px dashed {hex}44; color:#a07448; font-size:10px; line-height:1.4;",
                                span { style: "color:{hex}; opacity:.7;", ">" }
                                " {flavor}"
                                if let Some(by) = card_inner.flavor_by {
                                    div {
                                        style: "text-align:right; font-size:9px; color:#7a5a2c; margin-top:2px; opacity:.75;",
                                        "— {by}"
                                    }
                                }
                            }
                        }
                    }

                    // STAT BAR
                    div {
                        style: "display:flex; align-items:center; justify-content:space-between; padding:4px 8px; background:#050301; border-top:1px solid {hex}44; font-family:'JetBrains Mono',monospace; font-size:9.5px; letter-spacing:0.1em; font-weight:500; color:#8a6a42;",
                        span { style: "display:inline-flex; align-items:center; gap:5px;",
                            // Mini ferris
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

// ── Zoom Overlay ────────────────────────────────────────────────────────────

#[component]
fn ZoomWrap(card: Card, on_close: EventHandler<()>) -> Element {
    let lane = card.lane;
    let _hex = lane.hex();
    let _ink = lane.ink();

    rsx! {
        div {
            class: "zoom-wrap",
            onclick: move |e| e.stop_propagation(),

            // Scaled card
            div {
                style: "transform:scale(1.4); transform-origin:center; margin:40px 80px 40px 0;",
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
                            if let Some(text) = ability.text {
                                span { "{text}" }
                            }
                        }
                    }
                }
                // Code snippet
                if let Some(snippet) = snippet_for(card.name) {
                    div { class: "lore-label", "Hello, world" }
                    div { class: "code-block",
                        div { class: "code-head",
                            span { class: "dot r" }
                            span { class: "dot y" }
                            span { class: "dot g" }
                            span { class: "file", "{snippet.title}" }
                            span { class: "lang", "{snippet.lang}" }
                        }
                        pre {
                            code { "{snippet.code}" }
                        }
                    }
                }
                // Flavor
                if let Some(flavor) = card.flavor {
                    div { class: "lore-label", "Flavor" }
                    div { style: "font-style:italic; color:#cbbf95; font-size:14px;",
                        "\"{flavor}\""
                        if let Some(by) = card.flavor_by {
                            div {
                                style: "font-style:normal; font-size:11px; color:#9fb1a6; margin-top:4px;",
                                "— {by}"
                            }
                        }
                    }
                }
                // Deck stats
                if card.stars.is_some() {
                    div { class: "lore-label", "Deck Stats" }
                    div { class: "traits",
                        if let Some(stars) = card.stars {
                            span { class: "chip", "★ {stars} on GitHub" }
                        }
                        span { class: "chip", "Rarity · {card.rarity.label()}" }
                        {
                            let cost_str = card.cost.iter().map(|c| format!("{:?}", c)).collect::<Vec<_>>().join(" · ");
                            rsx! { span { class: "chip", "Cost · {cost_str}" } }
                        }
                    }
                }
            }
        }
    }
}
