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
}
