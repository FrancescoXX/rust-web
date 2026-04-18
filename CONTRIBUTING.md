# Contributing to rust-web-tcg

Thanks for your interest in contributing! This project is a fan-made trading card game celebrating the Rust web ecosystem, and contributions of all kinds are welcome — new cards, bug fixes, visual polish, accessibility improvements, documentation, or ideas.

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating you agree to uphold it. Report unacceptable behavior to the maintainers via a private GitHub issue or email.

## Ways to contribute

- 🃏 **Add or improve a card** — see [Adding a card](#adding-a-card) below
- 🐛 **Report a bug** — open an issue using the bug report template
- ✨ **Suggest a feature** — open an issue using the feature request template
- 🎨 **Polish visuals** — tweak the CSS in [`assets/main.css`](assets/main.css) or the card layout in [`src/components/card_view.rs`](src/components/card_view.rs)
- 📚 **Improve docs** — fix typos, clarify the README, add comments
- ♿ **Improve accessibility** — keyboard navigation, ARIA labels, color contrast

## Getting set up

Prerequisites:

- [Rust](https://rustup.rs/) (stable)
- The WASM target: `rustup target add wasm32-unknown-unknown`
- The Dioxus CLI: `cargo install dioxus-cli`

Then:

```bash
git clone https://github.com/<your-fork>/rust-web-tcg
cd rust-web-tcg
dx serve --platform web
```

Open <http://127.0.0.1:8080>. CSS hot-reloads; press `r` in the dev-server terminal to rebuild after editing Rust.

## Project layout

See [README.md](README.md#project-layout) for a tour of the directory structure.

## Adding a card

All cards live in [`data/cards.json`](data/cards.json). Append a new entry:

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

### Field reference

| Field         | Type     | Notes                                                                 |
| ------------- | -------- | --------------------------------------------------------------------- |
| `no`          | `u32`    | Collector number, must be unique                                      |
| `name`        | string   | Card title                                                            |
| `lane`        | enum     | One of: `Frontend`, `Backend`, `Fullstack`, `Runtime`, `Data`, `Api`, `View`, `Core`, `Tools` |
| `rarity`      | enum     | `Common`, `Uncommon`, `Rare`, `Mythic`                                |
| `cost`        | array    | Mana cost — `Cpu`, `Async`, `Net`, `Mem`, `Ui`, `Db`                  |
| `card_type`   | string   | Must start with `Framework`, `Land`, `Artifact`, or `Enchantment`     |
| `power`       | string   | Only shown for `Framework` cards                                      |
| `toughness`   | string   | Only shown for `Framework` cards                                      |
| `abilities`   | array    | `{ "label": "...", "text": "..." }` entries                           |
| `flavor`      | string?  | Italic flavor text                                                    |
| `flavor_by`   | string?  | Author attribution for the flavor                                     |
| `image_url`   | string?  | URL to logo / artwork                                                 |
| `github_repo` | string?  | `owner/repo` — used as fallback art and zoom-modal link               |
| `is_logo_card`| bool     | If `true`, contains the logo (uses light bg, contained sizing)        |
| `card_bg`     | string?  | CSS background override for the art window                            |

### Card-type rules (MTG conventions)

- **Framework** → Creature: shows power/toughness
- **Land** → mana source: no P/T
- **Artifact** → tool/library: no P/T
- **Enchantment** → middleware/protocol: no P/T

## Coding style

- Follow `cargo fmt` defaults — run `cargo fmt --all` before committing
- Run `cargo clippy --all-targets --all-features -- -D warnings` and fix any new lints
- Keep changes focused — one feature/fix per PR
- Match the existing CSS style in [`assets/main.css`](assets/main.css) (kebab-case classes, no preprocessor)
- Inline styles in `rsx!` are fine for one-off cards; promote to a CSS class when reused

## Commit messages

Follow [Conventional Commits](https://www.conventionalcommits.org/) where practical:

```
feat: add Salvo to the Backend pack
fix(zoom): keep close button sticky on iOS
docs: expand contributing guide
chore(deps): bump dioxus to 0.7.6
```

## Pull request process

1. Fork the repo and create a feature branch from `dioxus`
2. Make your changes, run `cargo fmt`, `cargo clippy`, and verify the app builds with `dx serve --platform web`
3. Open a PR against `dioxus` using the [PR template](.github/PULL_REQUEST_TEMPLATE.md)
4. Describe what you changed and why; attach screenshots/GIFs for visual changes
5. A maintainer will review — be prepared for revision requests

## Reporting security issues

See [SECURITY.md](SECURITY.md) — please **do not** open a public issue for vulnerabilities.

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
