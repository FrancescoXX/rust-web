use dioxus::prelude::*;
use crate::data::Lane;

// ── Deterministic PRNG ──────────────────────────────────────────────────────

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

// ── Procedural SVG Art ──────────────────────────────────────────────────────

#[component]
pub fn CardArtSvg(seed: String, lane: Lane) -> Element {
    let mut rand = det_rand(&seed);
    let hex = lane.hex();
    let ink = lane.ink();
    let key = lane.key();
    let palette = [hex, "#f2b866", "#0a0603", ink, "#e8852b"];
    let n = 14 + (rand() * 10.0) as usize;

    let mut shapes: Vec<(String, String)> = Vec::new();
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
pub fn ImageArt(src: String, fallback: Option<String>, lane: Lane, is_repo_card: bool) -> Element {
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

    let is_cover = errored();

    let img_style = if is_cover {
        "width:100%; height:100%; object-fit:cover; display:block; filter:saturate(1.05) contrast(1.02);"
    } else {
        "max-width:72%; max-height:72%; width:auto; height:auto; object-fit:contain; display:block; filter:drop-shadow(0 2px 8px rgba(0,0,0,.6));"
    };

    let bg = if is_repo_card {
        "position:relative; width:100%; height:100%; background:#fff; display:flex; align-items:center; justify-content:center; overflow:hidden;".to_string()
    } else {
        format!("position:relative; width:100%; height:100%; background:radial-gradient(circle at 50% 50%, {hex}22, #0a0603 80%); display:flex; align-items:center; justify-content:center; overflow:hidden;")
    };

    rsx! {
        div {
            style: "{bg}",
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
