use dioxus::prelude::*;

static FERRIS_IMG: Asset = asset!("/assets/ferris-web.png");

#[component]
pub fn FerrisIcon(width: f64, height: f64) -> Element {
    rsx! {
        svg {
            class: "ferris",
            width: "{width}",
            height: "{height}",
            view_box: "0 0 100 64",
            ellipse { cx: "50", cy: "58", rx: "30", ry: "3", fill: "#000", opacity: ".45" }
            g {
                rect { x: "4", y: "28", width: "14", height: "10", rx: "2", fill: "#e8852b", stroke: "#2a1606", stroke_width: "1.5" }
                polygon { points: "4,28 10,22 14,28", fill: "#f2b866", stroke: "#2a1606", stroke_width: "1.5" }
                polygon { points: "4,38 10,44 14,38", fill: "#b35a1a", stroke: "#2a1606", stroke_width: "1.5" }
            }
            g {
                rect { x: "82", y: "28", width: "14", height: "10", rx: "2", fill: "#e8852b", stroke: "#2a1606", stroke_width: "1.5" }
                polygon { points: "82,28 86,22 96,28", fill: "#f2b866", stroke: "#2a1606", stroke_width: "1.5" }
                polygon { points: "82,38 86,44 96,38", fill: "#b35a1a", stroke: "#2a1606", stroke_width: "1.5" }
            }
            g { stroke: "#2a1606", stroke_width: "1.5", fill: "#c26b20",
                rect { x: "22", y: "42", width: "4", height: "10" }
                rect { x: "32", y: "44", width: "4", height: "10" }
                rect { x: "42", y: "46", width: "4", height: "10" }
                rect { x: "54", y: "46", width: "4", height: "10" }
                rect { x: "64", y: "44", width: "4", height: "10" }
                rect { x: "74", y: "42", width: "4", height: "10" }
            }
            ellipse { cx: "50", cy: "36", rx: "32", ry: "18", fill: "#e8852b", stroke: "#2a1606", stroke_width: "2" }
            ellipse { cx: "50", cy: "32", rx: "28", ry: "12", fill: "#f2b866", opacity: ".35" }
            line { x1: "42", y1: "20", x2: "42", y2: "14", stroke: "#2a1606", stroke_width: "2" }
            line { x1: "58", y1: "20", x2: "58", y2: "14", stroke: "#2a1606", stroke_width: "2" }
            circle { cx: "42", cy: "12", r: "3.5", fill: "#fff", stroke: "#2a1606", stroke_width: "1.5" }
            circle { cx: "58", cy: "12", r: "3.5", fill: "#fff", stroke: "#2a1606", stroke_width: "1.5" }
            circle { cx: "42", cy: "12", r: "1.4", fill: "#2a1606" }
            circle { cx: "58", cy: "12", r: "1.4", fill: "#2a1606" }
            path { d: "M 44 38 Q 50 42 56 38", fill: "none", stroke: "#2a1606", stroke_width: "1.8", stroke_linecap: "round" }
            ellipse { cx: "40", cy: "28", rx: "6", ry: "3", fill: "#fff", opacity: ".4" }
        }
    }
}

#[component]
pub fn HeroFerris() -> Element {
    rsx! {
        div { class: "hero-ferris", aria_hidden: "true",
            img { src: FERRIS_IMG, alt: "" }
        }
    }
}

#[component]
pub fn HeroHeader() -> Element {
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
