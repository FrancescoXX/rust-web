# rust-web-tcg

> A Magic-the-Gathering-inspired trading card game for the Rust web ecosystem, built with [Dioxus](https://dioxuslabs.com/) and compiled to WebAssembly.

Every major Rust web library is reimagined as a collectible card — frameworks become creatures, runtimes become lands, ORMs and HTTP clients become artifacts, middleware becomes enchantments. Each card has a mana cost, abilities, flavor text, and (for creatures) power/toughness, all driven by JSON data and rendered with hand-crafted CSS to evoke the look and feel of a real MTG card.

## What's in the deck

Cards are organised by **lane** (the equivalent of MTG colors), each with its own visual identity:

| Lane          | MTG analogue      | Examples                         |
| ------------- | ----------------- | -------------------------------- |
| **Frontend**  | White (Plains)    | Yew, Sycamore, Leptos            |
| **Backend**   | Red (Mountain)    | Actix, Axum, Rocket, Warp        |
| **Fullstack** | Blue (Island)     | Leptos, Dioxus, Loco             |
| **Runtime**   | Green (Forest)    | Tokio, async-std, smol           |
| **Data**      | Black (Swamp)     | SQLx, Diesel, SeaORM             |
| **API**       | Azorius teal      | Tonic, Async-GraphQL             |
| **View**      | Boros orange      | Askama, Maud, Tera, Minijinja    |
| **Core**      | Colorless         | reqwest, serde, hyper            |
| **Tools**     | Gold (multicolor) | Cargo, wasm-bindgen, Trunk, Dioxus CLI |

Card types follow MTG semantics:
- **Framework** → Creature (has power/toughness)
- **Land** → mana source (no P/T)
- **Artifact** → tool / library
- **Enchantment** → middleware / protocol layer

## Features

- **Authentic card layout** — gilded multicolor frame, cream title/type bars, parchment textbox, lane-tinted P/T pill, faceted set-symbol gem.
- **Interactive 3D tilt** — cards rotate to follow your cursor, with a foil shimmer that tracks the highlight on rare and mythic cards.
- **Click-to-zoom modal** — open any card to see a larger view with full ability text, flavor, and a back arrow that stays sticky on mobile.
- **Full-text search** — filter cards by name, type, lane, ability text, or flavor.
- **Pack filter bar** — quick-filter by lane via colored swatches.
- **Responsive** — works on desktop and mobile, with an icon-only navbar on small screens.

## Stack

- [**Dioxus 0.7**](https://dioxuslabs.com/) — Rust UI framework, compiled to WASM
- [**serde**](https://serde.rs/) + JSON — card data lives in [`data/cards.json`](data/cards.json)
- Hand-written CSS in [`assets/main.css`](assets/main.css) — no Tailwind, no component library

## Project layout

```
src/
  main.rs              — App root, search & filter logic, zoom overlay
  components/
    card_view.rs       — The TCG card itself (frame, art, textbox, P/T)
    card_art.rs        — Generative SVG art + image loader for repo logos
    zoom.rs            — Modal view for the selected card
    hero.rs            — Header with the Ferris mascot
    nav.rs             — Top navigation
  data/
    types.rs           — Lane, Rarity, CostKind enums + Card struct
    packs.rs           — Pack/lane filter definitions
data/
  cards.json           — All card definitions (name, cost, abilities, flavor, etc.)
assets/
  main.css             — All styling
```

## Running locally

You'll need [Rust](https://rustup.rs/) and the [Dioxus CLI](https://dioxuslabs.com/learn/0.7/getting_started):

```bash
cargo install dioxus-cli
rustup target add wasm32-unknown-unknown
```

Then from the project root:

```bash
dx serve --platform web
```

The app will be available at <http://127.0.0.1:8080>. The dev server hot-reloads CSS automatically; press `r` in the terminal to rebuild after editing Rust source.

## Adding a card

Append an entry to [`data/cards.json`](data/cards.json):

```json
{
  "no": 35,
  "name": "MyCrate",
  "lane": "Backend",
  "rarity": "Rare",
  "cost": ["Cpu", "Async"],
  "card_type": "Framework — Backend · Tokio",
  "power": "3",
  "toughness": "4",
  "abilities": [
    { "label": "Fast", "text": "Cast spells with 0 allocation overhead." }
  ],
  "flavor": "Built on solid foundations.",
  "github_repo": "owner/repo"
}
```

`card_type` controls game-rule behavior: cards starting with `Framework` show power/toughness; `Land`, `Artifact`, and `Enchantment` cards do not (matching MTG).

## Disclaimer

Not affiliated with Wizards of the Coast or any of the Rust crates featured. All trademarks belong to their respective owners. This is a fan project celebrating the Rust web ecosystem.

## Open Source

This repository is structured as a standard open-source project:

- [License](LICENSE)
- [Contributing guide](CONTRIBUTING.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)
- [Security policy](SECURITY.md)
- [Governance](GOVERNANCE.md)
- [Changelog](CHANGELOG.md)
