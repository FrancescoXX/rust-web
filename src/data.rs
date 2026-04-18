// Rust Web TCG — data module
// Card database, snippets, image URLs, and supporting types.

use std::collections::HashMap;

// ── Enums ───────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Mythic,
}

impl Rarity {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Common => "Common",
            Self::Uncommon => "Uncommon",
            Self::Rare => "Rare",
            Self::Mythic => "Mythic",
        }
    }
    pub fn gem_color(&self) -> &'static str {
        match self {
            Self::Common => "#6b5a44",
            Self::Uncommon => "#b89468",
            Self::Rare => "#e8b05a",
            Self::Mythic => "#ff7a1f",
        }
    }
    pub fn glow(&self) -> &'static str {
        match self {
            Self::Common | Self::Uncommon => "none",
            Self::Rare => "0 0 0 1px #8a5a1a inset, 0 0 16px -6px rgba(232,176,90,.4)",
            Self::Mythic => "0 0 0 1px #6a2d05 inset, 0 0 28px -6px rgba(255,122,31,.65)",
        }
    }
    pub fn is_holo(&self) -> bool {
        matches!(self, Self::Rare | Self::Mythic)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Lane {
    Frontend,
    Backend,
    Fullstack,
    Runtime,
    Data,
    Api,
    View,
    Core,
    Tools,
}

impl Lane {
    pub fn id(&self) -> &'static str {
        match self {
            Self::Frontend => "frontend",
            Self::Backend => "backend",
            Self::Fullstack => "fullstack",
            Self::Runtime => "runtime",
            Self::Data => "data",
            Self::Api => "api",
            Self::View => "view",
            Self::Core => "core",
            Self::Tools => "tools",
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Self::Frontend => "Frontend",
            Self::Backend => "Backend",
            Self::Fullstack => "Fullstack",
            Self::Runtime => "Runtime",
            Self::Data => "Data",
            Self::Api => "API",
            Self::View => "View",
            Self::Core => "Core",
            Self::Tools => "Tools",
        }
    }
    pub fn key(&self) -> char {
        match self {
            Self::Frontend => 'F',
            Self::Backend => 'B',
            Self::Fullstack => 'S',
            Self::Runtime => 'R',
            Self::Data => 'D',
            Self::Api => 'A',
            Self::View => 'V',
            Self::Core => 'C',
            Self::Tools => 'T',
        }
    }
    pub fn hex(&self) -> &'static str {
        match self {
            Self::Frontend => "#e8a04a",
            Self::Backend => "#e8602b",
            Self::Fullstack => "#f2c15a",
            Self::Runtime => "#c47a28",
            Self::Data => "#b85a20",
            Self::Api => "#d89050",
            Self::View => "#a65f22",
            Self::Core => "#d4421a",
            Self::Tools => "#f0bd5a",
        }
    }
    pub fn bg(&self) -> &'static str {
        match self {
            Self::Frontend => "linear-gradient(180deg,#1e0f04,#2a1606)",
            Self::Backend => "linear-gradient(180deg,#1f0a03,#2e1205)",
            Self::Fullstack => "linear-gradient(180deg,#22150a,#2e1e10)",
            Self::Runtime => "linear-gradient(180deg,#180a02,#241205)",
            Self::Data => "linear-gradient(180deg,#180803,#241004)",
            Self::Api => "linear-gradient(180deg,#1c1006,#2a1a0a)",
            Self::View => "linear-gradient(180deg,#15090a,#1f0e04)",
            Self::Core => "linear-gradient(180deg,#1a0804,#261004)",
            Self::Tools => "linear-gradient(180deg,#1f1408,#2c1c0c)",
        }
    }
    pub fn ink(&self) -> &'static str {
        match self {
            Self::Frontend => "#f4d9a8",
            Self::Backend => "#f4c9a0",
            Self::Fullstack => "#f7e2b5",
            Self::Runtime => "#e8c28a",
            Self::Data => "#e8b088",
            Self::Api => "#f0cd98",
            Self::View => "#d9a170",
            Self::Core => "#f0b494",
            Self::Tools => "#f7dba5",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CostKind {
    Cpu,
    Async,
    Net,
    Mem,
    Ui,
    Db,
}

impl CostKind {
    pub fn bg_color(&self) -> &'static str {
        match self {
            Self::Cpu => "#f2b866",
            Self::Async => "#e8852b",
            Self::Net => "#d89050",
            Self::Mem => "#b85a20",
            Self::Ui => "#f4d9a8",
            Self::Db => "#8a4a15",
        }
    }
    pub fn ink_color(&self) -> &'static str {
        match self {
            Self::Cpu => "#2a1606",
            Self::Async => "#1c0e04",
            Self::Net => "#1c0a03",
            Self::Mem => "#f4d9a8",
            Self::Ui => "#2a1606",
            Self::Db => "#f4d9a8",
        }
    }
    pub fn glyph(&self) -> &'static str {
        match self {
            Self::Cpu => "⚡",
            Self::Async => "◉",
            Self::Net => "⇄",
            Self::Mem => "▣",
            Self::Ui => "◐",
            Self::Db => "▤",
        }
    }
}

// ── Structs ─────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
pub struct Ability {
    pub label: &'static str,
    pub text: Option<&'static str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Card {
    pub no: u32,
    pub name: &'static str,
    pub lane: Lane,
    pub rarity: Rarity,
    pub cost: Vec<CostKind>,
    pub card_type: &'static str,
    pub power: &'static str,
    pub toughness: &'static str,
    pub abilities: Vec<Ability>,
    pub flavor: Option<&'static str>,
    pub flavor_by: Option<&'static str>,
    pub stars: Option<&'static str>,
}

#[derive(Clone, Debug)]
pub struct Snippet {
    pub lang: &'static str,
    pub title: &'static str,
    pub code: &'static str,
}

pub struct Pack {
    pub id: &'static str,
    pub name: &'static str,
    pub swatch: &'static str,
}

// ── Card data ───────────────────────────────────────────────────────────────

pub fn all_cards() -> Vec<Card> {
    use CostKind::*;
    use Lane::*;
    use Rarity::*;
    vec![
        // FRONTEND
        Card {
            no: 1, name: "Leptos", lane: Frontend, rarity: Mythic,
            cost: vec![Ui, Ui, Cpu],
            card_type: "Framework — Frontend · Fullstack",
            power: "4", toughness: "3",
            abilities: vec![
                Ability { label: "Fine-grained", text: Some("Reactive signals update only what changed — no virtual DOM diff.") },
                Ability { label: "Server Fn", text: Some("⚡: Declare a Server Function callable as if it were local.") },
            ],
            flavor: Some("It is not the framework that renders. It is the signal that remembers."),
            flavor_by: Some("Leptos Book"), stars: Some("17k"),
        },
        Card {
            no: 2, name: "Dioxus", lane: Frontend, rarity: Rare,
            cost: vec![Ui, Ui],
            card_type: "Framework — Frontend · Cross-Target",
            power: "3", toughness: "3",
            abilities: vec![
                Ability { label: "Hooks", text: Some("VDOM with hook-style state. Feels like React, compiles like Rust.") },
                Ability { label: "Portable", text: Some("◉: Choose one — Web, Desktop, iOS, Android, TUI.") },
            ],
            flavor: Some("One component, any surface."),
            flavor_by: None, stars: None,
        },
        Card {
            no: 3, name: "Yew", lane: Frontend, rarity: Rare,
            cost: vec![Ui, Ui],
            card_type: "Framework — Frontend · Veteran",
            power: "3", toughness: "4",
            abilities: vec![
                Ability { label: "Steady", text: Some("The elder of Rust UI. Component-based, battle-worn.") },
                Ability { label: "Mature", text: Some("+1/+1 while you control a stable ecosystem.") },
            ],
            flavor: Some("Before there was a scene, there was Yew."),
            flavor_by: None, stars: None,
        },
        Card {
            no: 4, name: "Sycamore", lane: Frontend, rarity: Uncommon,
            cost: vec![Ui],
            card_type: "Framework — Frontend · Lightweight",
            power: "2", toughness: "2",
            abilities: vec![
                Ability { label: "No VDOM", text: Some("Fine-grained reactivity, tiny bundles. Spiritually aligned with Solid.") },
            ],
            flavor: Some("Less wood. More grain."),
            flavor_by: None, stars: None,
        },
        // BACKEND
        Card {
            no: 5, name: "Axum", lane: Backend, rarity: Mythic,
            cost: vec![Net, Cpu, Cpu],
            card_type: "Framework — Backend · Tokio",
            power: "5", toughness: "5",
            abilities: vec![
                Ability { label: "Extractor", text: Some("Route handlers pull state, paths, headers and JSON from their own type signature.") },
                Ability { label: "Tower", text: Some("Middleware, rate-limit, trace — plug a Tower service and it just composes.") },
            ],
            flavor: Some("Why argue with the request? Ask types to unpack it."),
            flavor_by: Some("Tokio zulip"), stars: None,
        },
        Card {
            no: 6, name: "Actix Web", lane: Backend, rarity: Rare,
            cost: vec![Net, Cpu],
            card_type: "Framework — Backend · Fast",
            power: "5", toughness: "3",
            abilities: vec![
                Ability { label: "Throughput", text: Some("Historic perf leader. +2/+0 when benchmarks are involved.") },
                Ability { label: "Actors", text: Some("Optional actor runtime available when you need it.") },
            ],
            flavor: Some("Some services never sleep. This one never even blinks."),
            flavor_by: None, stars: None,
        },
        Card {
            no: 7, name: "Rocket", lane: Backend, rarity: Rare,
            cost: vec![Net, Cpu],
            card_type: "Framework — Backend · Declarative",
            power: "3", toughness: "4",
            abilities: vec![
                Ability { label: "Macros", text: Some("Routes, guards, forms and JSON — all declared with attributes.") },
                Ability { label: "Batteries", text: Some("Templating, config, testing and fairings ship in the box.") },
            ],
            flavor: Some("Point the compiler. Launch."),
            flavor_by: None, stars: None,
        },
        Card {
            no: 8, name: "Warp", lane: Backend, rarity: Uncommon,
            cost: vec![Net, Cpu],
            card_type: "Framework — Backend · Combinators",
            power: "3", toughness: "2",
            abilities: vec![
                Ability { label: "Filters", text: Some("Routes compose from filters. Elegant for small APIs.") },
                Ability { label: "Typed", text: Some("When compilation succeeds, the shape is already right.") },
            ],
            flavor: Some("One filter. Then another. Then, somehow, an API."),
            flavor_by: None, stars: None,
        },
        Card {
            no: 9, name: "Poem", lane: Backend, rarity: Uncommon,
            cost: vec![Net],
            card_type: "Framework — Backend · OpenAPI",
            power: "2", toughness: "3",
            abilities: vec![
                Ability { label: "Schema", text: Some("Generate an OpenAPI spec directly from your handler types.") },
            ],
            flavor: Some("A verse, a route, a schema."),
            flavor_by: None, stars: None,
        },
        // FULLSTACK
        Card {
            no: 10, name: "Loco", lane: Fullstack, rarity: Rare,
            cost: vec![Cpu, Net, Db],
            card_type: "Framework — Fullstack · MVC",
            power: "4", toughness: "4",
            abilities: vec![
                Ability { label: "Generate", text: Some("`loco generate scaffold` — models, controllers, migrations.") },
                Ability { label: "Workers", text: Some("Background jobs and mailers belong to the framework, not your glue code.") },
            ],
            flavor: Some("Not every track is new. Some are just well-laid."),
            flavor_by: None, stars: None,
        },
        Card {
            no: 11, name: "Perseus", lane: Fullstack, rarity: Uncommon,
            cost: vec![Ui, Cpu],
            card_type: "Framework — Fullstack · SSG",
            power: "3", toughness: "2",
            abilities: vec![
                Ability { label: "Hybrid", text: Some("Static, server-rendered, or incremental — pick per route.") },
                Ability { label: "Built on Sycamore.", text: None },
            ],
            flavor: Some("Prebuild what is knowable. Stream the rest."),
            flavor_by: None, stars: None,
        },
        // RUNTIME
        Card {
            no: 12, name: "Tokio", lane: Runtime, rarity: Mythic,
            cost: vec![Async, Async],
            card_type: "Land — Async Runtime",
            power: "∞", toughness: "∞",
            abilities: vec![
                Ability { label: "Tap", text: Some("◉: Add one ASYNC. This land is underneath every other card you play.") },
                Ability { label: "Work-steal", text: Some("Scales cores without you thinking about cores.") },
            ],
            flavor: Some("If your future wakes up, it wakes up here."),
            flavor_by: None, stars: None,
        },
        Card {
            no: 13, name: "async-std", lane: Runtime, rarity: Uncommon,
            cost: vec![Async],
            card_type: "Land — Runtime",
            power: "3", toughness: "3",
            abilities: vec![
                Ability { label: "Shape", text: Some("An async runtime modeled after `std`. Familiar, smaller world.") },
            ],
            flavor: Some("Like std, only it waits."),
            flavor_by: None, stars: None,
        },
        Card {
            no: 14, name: "smol", lane: Runtime, rarity: Uncommon,
            cost: vec![Async],
            card_type: "Land — Runtime",
            power: "2", toughness: "2",
            abilities: vec![
                Ability { label: "Minimal", text: Some("A small executor and a handful of friends. Bring your own world.") },
            ],
            flavor: Some("Big worlds begin with small executors."),
            flavor_by: None, stars: None,
        },
        // DATA
        Card {
            no: 15, name: "SQLx", lane: Data, rarity: Mythic,
            cost: vec![Db, Db],
            card_type: "Artifact — Data · Async",
            power: "4", toughness: "4",
            abilities: vec![
                Ability { label: "Compile-check", text: Some("`query!` connects to your database at build time and types the result.") },
                Ability { label: "No ORM", text: Some("Just SQL. Just rows. Just types.") },
            ],
            flavor: Some("If it compiles, the database already agreed."),
            flavor_by: None, stars: None,
        },
        Card {
            no: 16, name: "Diesel", lane: Data, rarity: Rare,
            cost: vec![Db, Cpu],
            card_type: "Artifact — ORM · Schema-first",
            power: "4", toughness: "5",
            abilities: vec![
                Ability { label: "Builder", text: Some("A strongly-typed query DSL with migrations and schema codegen.") },
            ],
            flavor: Some("Slower to start. Harder to break."),
            flavor_by: None, stars: None,
        },
        Card {
            no: 17, name: "SeaORM", lane: Data, rarity: Rare,
            cost: vec![Db, Cpu],
            card_type: "Artifact — ORM · Async",
            power: "3", toughness: "3",
            abilities: vec![
                Ability { label: "Entities", text: Some("ActiveRecord-style entities with codegen from your live schema.") },
            ],
            flavor: Some("For tides that change, but are still tides."),
            flavor_by: None, stars: None,
        },
        // API / RPC
        Card {
            no: 18, name: "tonic", lane: Api, rarity: Rare,
            cost: vec![Net, Net],
            card_type: "Framework — API · gRPC",
            power: "4", toughness: "3",
            abilities: vec![
                Ability { label: "Protobuf", text: Some("Codegen from `.proto`; runs on Hyper + Tower.") },
                Ability { label: "Stream", text: Some("Bidi streaming, first-class.") },
            ],
            flavor: Some("Messages of fixed shape, conversed at wire speed."),
            flavor_by: None, stars: None,
        },
        Card {
            no: 19, name: "async-graphql", lane: Api, rarity: Rare,
            cost: vec![Net, Cpu],
            card_type: "Framework — API · GraphQL",
            power: "3", toughness: "3",
            abilities: vec![
                Ability { label: "Derive", text: Some("Objects, inputs, and unions from derive macros.") },
                Ability { label: "Dataloader", text: Some("Batched loading and subscriptions included.") },
            ],
            flavor: Some("Ask for the shape you want. Receive exactly that."),
            flavor_by: None, stars: None,
        },
        Card {
            no: 20, name: "utoipa", lane: Api, rarity: Uncommon,
            cost: vec![Net],
            card_type: "Enchantment — OpenAPI",
            power: "2", toughness: "2",
            abilities: vec![
                Ability { label: "Derive", text: Some("Generate an OpenAPI 3 document from your Rust types.") },
            ],
            flavor: Some("What the endpoints said. Written down."),
            flavor_by: None, stars: None,
        },
        // VIEW / TEMPLATING
        Card {
            no: 21, name: "Askama", lane: View, rarity: Uncommon,
            cost: vec![Ui],
            card_type: "Enchantment — Templates",
            power: "2", toughness: "3",
            abilities: vec![
                Ability { label: "Compiled", text: Some("Jinja-style templates, type-checked and compiled to plain Rust.") },
            ],
            flavor: Some("A blessing on the render path."),
            flavor_by: None, stars: None,
        },
        Card {
            no: 22, name: "Maud", lane: View, rarity: Uncommon,
            cost: vec![Ui],
            card_type: "Enchantment — Inline HTML",
            power: "2", toughness: "2",
            abilities: vec![
                Ability { label: "html!", text: Some("An inline HTML macro — every tag statically checked.") },
            ],
            flavor: Some("HTML, but the compiler read it too."),
            flavor_by: None, stars: None,
        },
        Card {
            no: 23, name: "Tera", lane: View, rarity: Common,
            cost: vec![Ui],
            card_type: "Enchantment — Templates · Runtime",
            power: "2", toughness: "2",
            abilities: vec![
                Ability { label: "Runtime", text: Some("Edit templates without rebuilding. Inheritance and filters included.") },
            ],
            flavor: Some("When the compile loop is the wrong loop."),
            flavor_by: None, stars: None,
        },
        Card {
            no: 24, name: "reqwest", lane: Api, rarity: Common,
            cost: vec![Net],
            card_type: "Artifact — HTTP Client",
            power: "2", toughness: "2",
            abilities: vec![
                Ability { label: "Client", text: Some("The calm, obvious HTTP client the rest of your service will import.") },
            ],
            flavor: Some("Most requests are just requests."),
            flavor_by: None, stars: None,
        },
        // CORE
        Card {
            no: 25, name: "serde", lane: Core, rarity: Mythic,
            cost: vec![Cpu, Cpu],
            card_type: "Enchantment — Serialization",
            power: "∞", toughness: "∞",
            abilities: vec![
                Ability { label: "Derive", text: Some("#[derive(Serialize, Deserialize)] — every format, one trait.") },
                Ability { label: "Ubiquitous", text: Some("Half your Cargo.lock transitively depends on this card.") },
            ],
            flavor: Some("Every byte you ever touched passed through here."),
            flavor_by: Some("Every Rust project, ever"), stars: None,
        },
        Card {
            no: 26, name: "hyper", lane: Core, rarity: Mythic,
            cost: vec![Net, Net],
            card_type: "Artifact — HTTP Engine",
            power: "5", toughness: "5",
            abilities: vec![
                Ability { label: "Foundation", text: Some("The HTTP/1 + HTTP/2 implementation Axum, reqwest and tonic all stand on.") },
                Ability { label: "Low-level", text: Some("You can reach past your framework and touch the wire directly.") },
            ],
            flavor: Some("Somewhere under every Rust service, a hyper task is listening."),
            flavor_by: None, stars: None,
        },
        Card {
            no: 27, name: "tower", lane: Core, rarity: Rare,
            cost: vec![Cpu, Net],
            card_type: "Enchantment — Middleware",
            power: "3", toughness: "4",
            abilities: vec![
                Ability { label: "Service", text: Some("One trait — `Service<Request>` — composes rate-limits, retries, auth, tracing.") },
                Ability { label: "Reusable", text: Some("The same layers work in Axum, tonic, and any custom hyper stack.") },
            ],
            flavor: Some("Every request climbs the tower, and every response comes back down."),
            flavor_by: None, stars: None,
        },
        Card {
            no: 28, name: "tracing", lane: Core, rarity: Rare,
            cost: vec![Cpu],
            card_type: "Enchantment — Observability",
            power: "3", toughness: "3",
            abilities: vec![
                Ability { label: "Spans", text: Some("Structured, async-aware spans that survive `.await` points.") },
                Ability { label: "Subscribers", text: Some("Plug a subscriber — pretty logs, OpenTelemetry, Jaeger — same events.") },
            ],
            flavor: Some("println! for the first service. tracing for the tenth."),
            flavor_by: None, stars: None,
        },
        Card {
            no: 29, name: "anyhow", lane: Core, rarity: Uncommon,
            cost: vec![Cpu],
            card_type: "Artifact — Error · Application",
            power: "2", toughness: "3",
            abilities: vec![
                Ability { label: "Context", text: Some(".context(\"while reading config\") — errors that know where they were.") },
                Ability { label: "One Type", text: Some("`anyhow::Result` — one error, any source. Perfect for binaries.") },
            ],
            flavor: Some("For when you just need the error to tell you what happened."),
            flavor_by: None, stars: None,
        },
        Card {
            no: 30, name: "thiserror", lane: Core, rarity: Uncommon,
            cost: vec![Cpu],
            card_type: "Artifact — Error · Library",
            power: "2", toughness: "3",
            abilities: vec![
                Ability { label: "Derive", text: Some("#[derive(Error)] — zero-cost typed errors with Display and source().") },
                Ability { label: "Library", text: Some("The dual of anyhow. Use in crates you publish.") },
            ],
            flavor: Some("Anyhow for apps. This one, for the library you ship."),
            flavor_by: None, stars: None,
        },
        // TOOLS
        Card {
            no: 31, name: "cargo", lane: Tools, rarity: Mythic,
            cost: vec![Cpu],
            card_type: "Land — Package Manager",
            power: "∞", toughness: "∞",
            abilities: vec![
                Ability { label: "Tap", text: Some("◉: `cargo build`. ◉: `cargo test`. ◉: `cargo run`.") },
                Ability { label: "Workspaces", text: Some("One lockfile. Many crates. Shared target/.") },
            ],
            flavor: Some("Rust without cargo is just a compiler looking for a home."),
            flavor_by: None, stars: None,
        },
        Card {
            no: 32, name: "wasm-bindgen", lane: Tools, rarity: Rare,
            cost: vec![Ui, Cpu],
            card_type: "Enchantment — Rust ↔ JS Bridge",
            power: "3", toughness: "3",
            abilities: vec![
                Ability { label: "ABI", text: Some("Call JS from Rust. Call Rust from JS. Types cross the boundary intact.") },
                Ability { label: "Foundation", text: Some("Everything WASM on the web stands on this card.") },
            ],
            flavor: Some("The border crossing where Rust and JavaScript shake hands."),
            flavor_by: None, stars: None,
        },
        Card {
            no: 33, name: "trunk", lane: Tools, rarity: Uncommon,
            cost: vec![Ui],
            card_type: "Artifact — WASM Bundler",
            power: "3", toughness: "2",
            abilities: vec![
                Ability { label: "Index.html", text: Some("Point it at an HTML file; it compiles, bundles and serves the rest.") },
                Ability { label: "Live", text: Some("Hot-reload dev server for Leptos, Yew, Dioxus, Sycamore.") },
            ],
            flavor: Some("You write the index; trunk writes the pipeline."),
            flavor_by: None, stars: None,
        },
        Card {
            no: 34, name: "clap", lane: Tools, rarity: Rare,
            cost: vec![Cpu],
            card_type: "Artifact — CLI Builder",
            power: "3", toughness: "4",
            abilities: vec![
                Ability { label: "Derive", text: Some("#[derive(Parser)] — structs become argument parsers, flags and subcommands.") },
                Ability { label: "Help", text: Some("--help, shell completion, colored errors — all included.") },
            ],
            flavor: Some("Every Rust CLI you have ever loved starts the same way: use clap::Parser."),
            flavor_by: None, stars: None,
        },
    ]
}

// ── Packs ───────────────────────────────────────────────────────────────────

pub fn all_packs() -> Vec<Pack> {
    vec![
        Pack { id: "all", name: "All Cards", swatch: "#c9a34a" },
        Pack { id: "frontend", name: "Frontend", swatch: "#e8a04a" },
        Pack { id: "backend", name: "Backend", swatch: "#e8602b" },
        Pack { id: "fullstack", name: "Fullstack", swatch: "#f2c15a" },
        Pack { id: "runtime", name: "Runtime", swatch: "#c47a28" },
        Pack { id: "data", name: "Data", swatch: "#b85a20" },
        Pack { id: "api", name: "API", swatch: "#d89050" },
        Pack { id: "view", name: "View", swatch: "#a65f22" },
        Pack { id: "core", name: "Core", swatch: "#d4421a" },
        Pack { id: "tools", name: "Tools", swatch: "#f0bd5a" },
    ]
}

// ── Card image URLs ─────────────────────────────────────────────────────────

pub fn card_image_url(name: &str) -> Option<&'static str> {
    match name {
        "Leptos" => Some("https://repository-images.githubusercontent.com/519882617/3d86477e-90b1-4be2-a401-ff414013529e"),
        "Dioxus" => Some("https://raw.githubusercontent.com/DioxusLabs/dioxus/main/packages/docs-router/src/doc_examples/dog_app_assets/dioxus_logo.png"),
        "Yew" => Some("https://yew.rs/img/logo.svg"),
        "Sycamore" => Some("https://raw.githubusercontent.com/sycamore-rs/sycamore/master/assets/logo.png"),
        "Axum" => Some("https://raw.githubusercontent.com/tokio-rs/axum/main/axum/src/docs/logo.png"),
        "Actix Web" => Some("https://actix.rs/img/logo.png"),
        "Rocket" => Some("https://rocket.rs/images/logo.svg"),
        "Warp" => Some("https://raw.githubusercontent.com/seanmonstar/warp/master/assets/logo.svg"),
        "Poem" => Some("https://raw.githubusercontent.com/poem-web/poem/master/logo.png"),
        "Loco" => Some("https://loco.rs/icon.svg"),
        "Perseus" => Some("https://raw.githubusercontent.com/framesurge/perseus/main/docs/assets/perseus-logo.svg"),
        "Tokio" => Some("https://tokio.rs/img/tokio-horizontal.svg"),
        "async-std" => Some("https://raw.githubusercontent.com/async-rs/async-std/main/assets/async-std-logo.png"),
        "smol" => Some("https://raw.githubusercontent.com/smol-rs/smol/master/assets/smol.png"),
        "SQLx" => Some("https://raw.githubusercontent.com/launchbadge/sqlx/main/assets/logo.png"),
        "Diesel" => Some("https://diesel.rs/assets/images/diesel_logo_stacked_black.png"),
        "SeaORM" => Some("https://www.sea-ql.org/SeaORM/img/SeaQL%20logo%20dual.png"),
        "tonic" => Some("https://raw.githubusercontent.com/hyperium/tonic/master/.github/assets/tonic.png"),
        "async-graphql" => Some("https://raw.githubusercontent.com/async-graphql/async-graphql/master/logo.svg"),
        "utoipa" => Some("https://raw.githubusercontent.com/juhaku/utoipa/master/docs/utoipa.png"),
        "Askama" => Some("https://raw.githubusercontent.com/rinja-rs/askama/main/book/src/logo.svg"),
        "Maud" => Some("https://raw.githubusercontent.com/lambda-fairy/maud/main/logo.svg"),
        "Tera" => Some("https://keats.github.io/tera/img/logo.png"),
        "reqwest" => Some("https://avatars.githubusercontent.com/u/62226355"),
        "serde" => Some("https://serde.rs/img/serde.svg"),
        "hyper" => Some("https://hyper.rs/hyper-logo.svg"),
        "tower" => Some("https://raw.githubusercontent.com/tower-rs/tower/master/assets/tower.png"),
        "tracing" => Some("https://raw.githubusercontent.com/tokio-rs/tracing/master/assets/logo.svg"),
        "anyhow" => Some("https://avatars.githubusercontent.com/u/1940490"),
        "thiserror" => Some("https://avatars.githubusercontent.com/u/1940490"),
        "cargo" => Some("https://www.rust-lang.org/static/images/cargo.png"),
        "wasm-bindgen" => Some("https://rustwasm.github.io/wasm-bindgen/_static/wasm-ferris.png"),
        "trunk" => Some("https://trunkrs.dev/assets/trunk-logo.svg"),
        "clap" => Some("https://raw.githubusercontent.com/clap-rs/clap/master/assets/clap.png"),
        _ => None,
    }
}

pub fn og_fallback(name: &str) -> Option<&'static str> {
    match name {
        "Leptos" => Some("leptos-rs/leptos"),
        "Dioxus" => Some("DioxusLabs/dioxus"),
        "Yew" => Some("yewstack/yew"),
        "Sycamore" => Some("sycamore-rs/sycamore"),
        "Axum" => Some("tokio-rs/axum"),
        "Actix Web" => Some("actix/actix-web"),
        "Rocket" => Some("rwf2/Rocket"),
        "Warp" => Some("seanmonstar/warp"),
        "Poem" => Some("poem-web/poem"),
        "Loco" => Some("loco-rs/loco"),
        "Perseus" => Some("framesurge/perseus"),
        "Tokio" => Some("tokio-rs/tokio"),
        "async-std" => Some("async-rs/async-std"),
        "smol" => Some("smol-rs/smol"),
        "SQLx" => Some("launchbadge/sqlx"),
        "Diesel" => Some("diesel-rs/diesel"),
        "SeaORM" => Some("SeaQL/sea-orm"),
        "tonic" => Some("hyperium/tonic"),
        "async-graphql" => Some("async-graphql/async-graphql"),
        "utoipa" => Some("juhaku/utoipa"),
        "Askama" => Some("rinja-rs/askama"),
        "Maud" => Some("lambda-fairy/maud"),
        "Tera" => Some("Keats/tera"),
        "reqwest" => Some("seanmonstar/reqwest"),
        "serde" => Some("serde-rs/serde"),
        "hyper" => Some("hyperium/hyper"),
        "tower" => Some("tower-rs/tower"),
        "tracing" => Some("tokio-rs/tracing"),
        "anyhow" => Some("dtolnay/anyhow"),
        "thiserror" => Some("dtolnay/thiserror"),
        "cargo" => Some("rust-lang/cargo"),
        "wasm-bindgen" => Some("rustwasm/wasm-bindgen"),
        "trunk" => Some("trunk-rs/trunk"),
        "clap" => Some("clap-rs/clap"),
        _ => None,
    }
}

// ── Snippets ────────────────────────────────────────────────────────────────

pub fn snippet_for(name: &str) -> Option<Snippet> {
    match name {
        "Leptos" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <button on:click=move |_| set_count.update(|n| *n += 1)>
            "Clicked " {count} " times"
        </button>
    }
}

fn main() { mount_to_body(App) }"# }),

        "Dioxus" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use dioxus::prelude::*;

fn App() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        button { onclick: move |_| count += 1, "Clicked {count} times" }
    }
}

fn main() { dioxus::launch(App); }"# }),

        "Yew" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let count = use_state(|| 0);
    let onclick = { let count = count.clone();
        Callback::from(move |_| count.set(*count + 1)) };
    html! { <button {onclick}>{ format!("Clicked {} times", *count) }</button> }
}

fn main() { yew::Renderer::<App>::new().render(); }"# }),

        "Sycamore" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| {
        let count = create_signal(cx, 0);
        view! { cx,
            button(on:click=move |_| count.set(*count.get() + 1)) {
                "Clicked " (count.get()) " times"
            }
        }
    });
}"# }),

        "Axum" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "hello, world!" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}"# }),

        "Actix Web" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder { "hello, world!" }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 3000))?.run().await
}"# }),

        "Rocket" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str { "hello, world!" }

#[launch]
fn rocket() -> _ { rocket::build().mount("/", routes![index]) }"# }),

        "Warp" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path::end().map(|| "hello, world!");
    warp::serve(hello).run(([0, 0, 0, 0], 3000)).await;
}"# }),

        "Poem" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use poem::{get, handler, listener::TcpListener, Route, Server};

#[handler] fn hello() -> &'static str { "hello, world!" }

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/", get(hello));
    Server::new(TcpListener::bind("0.0.0.0:3000")).run(app).await
}"# }),

        "Loco" => Some(Snippet { lang: "bash", title: "shell", code: r#"# Scaffold a full MVC app — models, controllers, migrations, workers
$ cargo install loco-cli
$ loco new -n blog
$ cd blog
$ cargo loco generate scaffold post title:string body:text
$ cargo loco start"# }),

        "Perseus" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use perseus::prelude::*;
use sycamore::prelude::*;

#[perseus::main(perseus_warp::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new().template(
        Template::build("index").view(|cx| view! { cx, h1 { "hello, world!" } })
    )
}"# }),

        "Tokio" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (a, b) = tokio::join!(work("A"), work("B"));
    println!("done: {a} / {b}");
}

async fn work(name: &str) -> &str {
    sleep(Duration::from_millis(100)).await;
    name
}"# }),

        "async-std" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use async_std::task;

#[async_std::main]
async fn main() {
    task::spawn(async { println!("hello from a task") }).await;
}"# }),

        "smol" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"fn main() {
    smol::block_on(async {
        let task = smol::spawn(async { 1 + 1 });
        println!("{}", task.await);
    });
}"# }),

        "SQLx" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use sqlx::postgres::PgPool;

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    let pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;
    // query! verifies this against the live DB at compile time
    let rec = sqlx::query!("SELECT name FROM users WHERE id = $1", 1i64)
        .fetch_one(&pool).await?;
    println!("hello, {}", rec.name);
    Ok(())
}"# }),

        "Diesel" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use diesel::prelude::*;
use crate::schema::users::dsl::*;

fn main() {
    let mut conn = PgConnection::establish(&std::env::var("DATABASE_URL").unwrap()).unwrap();
    let names: Vec<String> = users.select(name).limit(5).load(&mut conn).unwrap();
    for n in names { println!("hello, {n}"); }
}"# }),

        "SeaORM" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use sea_orm::*;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = Database::connect("postgres://user:pass@localhost/db").await?;
    let users: Vec<user::Model> = user::Entity::find().all(&db).await?;
    for u in users { println!("hello, {}", u.name); }
    Ok(())
}"# }),

        "tonic" => Some(Snippet { lang: "rust", title: "src/server.rs", code: r#"use tonic::{transport::Server, Request, Response, Status};
use hello::{greeter_server::{Greeter, GreeterServer}, HelloReply, HelloRequest};

#[derive(Default)] pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, req: Request<HelloRequest>)
        -> Result<Response<HelloReply>, Status> {
        Ok(Response::new(HelloReply { message: format!("hello, {}", req.into_inner().name) }))
    }
}"# }),

        "async-graphql" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use async_graphql::*;

struct Query;

#[Object]
impl Query {
    async fn hello(&self, #[graphql(default = "world")] name: String) -> String {
        format!("hello, {name}")
    }
}

fn main() {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    println!("{}", schema.sdl());
}"# }),

        "utoipa" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use utoipa::{OpenApi, ToSchema};

#[derive(ToSchema)] struct User { id: i64, name: String }

#[utoipa::path(get, path = "/users/{id}", responses(
    (status = 200, body = User)
))]
async fn get_user() {}

#[derive(OpenApi)]
#[openapi(paths(get_user), components(schemas(User)))]
struct ApiDoc;

fn main() { println!("{}", ApiDoc::openapi().to_pretty_json().unwrap()); }"# }),

        "Askama" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use askama::Template;

#[derive(Template)]
#[template(path = "hello.html")] // templates/hello.html: Hello, {{ name }}!
struct HelloTpl<'a> { name: &'a str }

fn main() {
    let html = HelloTpl { name: "world" }.render().unwrap();
    println!("{html}");
}"# }),

        "Maud" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use maud::{html, Markup};

fn page(name: &str) -> Markup {
    html! {
        h1 { "hello, " (name) "!" }
    }
}

fn main() { println!("{}", page("world").into_string()); }"# }),

        "Tera" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use tera::{Context, Tera};

fn main() {
    let tera = Tera::new("templates/**/*").unwrap();
    let mut ctx = Context::new();
    ctx.insert("name", "world");
    println!("{}", tera.render("hello.html", &ctx).unwrap());
}"# }),

        "reqwest" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;
    println!("{}", &body[..200]);
    Ok(())
}"# }),

        "serde" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct User { id: u64, name: String }

fn main() -> serde_json::Result<()> {
    let u = User { id: 1, name: "ferris".into() };
    let json = serde_json::to_string(&u)?;
    let back: User = serde_json::from_str(&json)?;
    println!("{json}  ->  {back:?}");
    Ok(())
}"# }),

        "hyper" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use http_body_util::Full;
use bytes::Bytes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            hyper::server::conn::http1::Builder::new()
                .serve_connection(TokioIo::new(stream), service_fn(|_req| async {
                    Ok::<_, hyper::Error>(hyper::Response::new(Full::new(Bytes::from("hello"))))
                })).await.ok();
        });
    }
}"# }),

        "tower" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use tower::{ServiceBuilder, ServiceExt, service_fn};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let svc = ServiceBuilder::new()
        .timeout(Duration::from_secs(1))
        .service(service_fn(|req: &str| async move { Ok::<_, ()>(format!("hello, {req}")) }));

    let reply = svc.oneshot("world").await.unwrap();
    println!("{reply}");
}"# }),

        "tracing" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use tracing::{info, instrument};

#[instrument]
fn greet(name: &str) {
    info!(target: "app", "hello, {name}");
}

fn main() {
    tracing_subscriber::fmt().init();
    greet("world");
}"# }),

        "anyhow" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use anyhow::{Context, Result};

fn read_config() -> Result<String> {
    std::fs::read_to_string("config.toml")
        .context("while reading config.toml")
}

fn main() -> Result<()> {
    let cfg = read_config()?;
    println!("{cfg}");
    Ok(())
}"# }),

        "thiserror" => Some(Snippet { lang: "rust", title: "src/error.rs", code: r#"use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("io: {0}")]      Io(#[from] std::io::Error),
    #[error("not found: {0}")] NotFound(String),
}

pub fn load(name: &str) -> Result<String, AppError> {
    std::fs::read_to_string(name).map_err(Into::into)
}"# }),

        "cargo" => Some(Snippet { lang: "bash", title: "shell", code: r#"$ cargo new hello
$ cd hello
$ cat src/main.rs
fn main() { println!("hello, world!"); }

$ cargo add serde --features derive
$ cargo build --release
$ cargo test"# }),

        "wasm-bindgen" => Some(Snippet { lang: "rust", title: "src/lib.rs", code: r#"use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    log(&format!("hello, {name}"));
}"# }),

        "trunk" => Some(Snippet { lang: "bash", title: "shell", code: r#"# index.html  ->  <link data-trunk rel="rust" />
$ trunk serve --open
# live reload, wasm-bindgen + wasm-opt, CSS/SCSS/Tailwind — all wired.
$ trunk build --release"# }),

        "clap" => Some(Snippet { lang: "rust", title: "src/main.rs", code: r#"use clap::Parser;

#[derive(Parser)]
#[command(name = "hello", version, about)]
struct Cli {
    /// who to greet
    #[arg(short, long, default_value = "world")]
    name: String,
}

fn main() {
    let cli = Cli::parse();
    println!("hello, {}!", cli.name);
}"# }),

        _ => None,
    }
}

// ── Lane counts helper ──────────────────────────────────────────────────────

pub fn lane_counts(cards: &[Card]) -> HashMap<String, usize> {
    let mut m = HashMap::new();
    m.insert("all".to_string(), cards.len());
    for c in cards {
        *m.entry(c.lane.id().to_string()).or_insert(0) += 1;
    }
    m
}
