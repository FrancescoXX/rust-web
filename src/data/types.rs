use serde::Deserialize;

// ── Rarity ──────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize)]
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

    pub fn short(&self) -> &'static str {
        match self {
            Self::Common => "C",
            Self::Uncommon => "U",
            Self::Rare => "R",
            Self::Mythic => "M",
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

// ── Lane ────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize)]
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
            Self::Frontend  => "#f5ecd0", // White (Plains) - parchment
            Self::Backend   => "#c83830", // Red (Mountain) - crimson
            Self::Fullstack => "#3a78c8", // Blue (Island) - sapphire
            Self::Runtime   => "#4a9858", // Green (Forest) - moss
            Self::Data      => "#7a4d96", // Black/Purple (Swamp) - amethyst
            Self::Api       => "#3aa0a8", // Azorius teal
            Self::View      => "#d87830", // Boros orange
            Self::Core      => "#9098a4", // Colorless - pewter
            Self::Tools     => "#d4af37", // Gold (multicolor)
        }
    }

    pub fn bg(&self) -> &'static str {
        match self {
            Self::Frontend  => "linear-gradient(180deg,#f0e6c8,#dccfa8)",
            Self::Backend   => "linear-gradient(180deg,#2a0a08,#3a100c)",
            Self::Fullstack => "linear-gradient(180deg,#0a1424,#101e38)",
            Self::Runtime   => "linear-gradient(180deg,#0c1e10,#143018)",
            Self::Data      => "linear-gradient(180deg,#180c20,#241432)",
            Self::Api       => "linear-gradient(180deg,#0a201e,#102e2c)",
            Self::View      => "linear-gradient(180deg,#241408,#341e0c)",
            Self::Core      => "linear-gradient(180deg,#14141a,#1c1c24)",
            Self::Tools     => "linear-gradient(180deg,#241c08,#34280c)",
        }
    }

    pub fn ink(&self) -> &'static str {
        match self {
            Self::Frontend  => "#2a2418",
            Self::Backend   => "#f4b8a8",
            Self::Fullstack => "#b8d0f0",
            Self::Runtime   => "#b8e0c0",
            Self::Data      => "#d8b8f0",
            Self::Api       => "#b0e8d8",
            Self::View      => "#f4d0a0",
            Self::Core      => "#d0d4dc",
            Self::Tools     => "#f0d890",
        }
    }
}

// ── CostKind ────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
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

// ── Card & Ability ──────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Ability {
    pub label: String,
    pub text: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Card {
    pub no: u32,
    pub name: String,
    pub lane: Lane,
    pub rarity: Rarity,
    pub cost: Vec<CostKind>,
    pub card_type: String,
    pub power: String,
    pub toughness: String,
    pub abilities: Vec<Ability>,
    pub flavor: Option<String>,
    pub flavor_by: Option<String>,
    pub stars: Option<String>,
    pub image_url: Option<String>,
    pub github_repo: Option<String>,
    #[serde(default)]
    pub is_logo_card: bool,
    pub card_bg: Option<String>,
}
