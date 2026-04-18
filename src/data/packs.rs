pub struct Pack {
    pub id: &'static str,
    pub name: &'static str,
    pub swatch: &'static str,
}

pub fn all_packs() -> Vec<Pack> {
    vec![
        Pack { id: "all",      name: "All Cards",  swatch: "#c9a34a" },
        Pack { id: "frontend", name: "Frontend",   swatch: "#f5ecd0" },
        Pack { id: "backend",  name: "Backend",    swatch: "#c83830" },
        Pack { id: "fullstack",name: "Fullstack",  swatch: "#3a78c8" },
        Pack { id: "runtime",  name: "Runtime",    swatch: "#4a9858" },
        Pack { id: "data",     name: "Data",       swatch: "#7a4d96" },
        Pack { id: "api",      name: "API",        swatch: "#22c8b0" },
        Pack { id: "view",     name: "View",       swatch: "#f09030" },
        Pack { id: "core",     name: "Core",       swatch: "#a0a0b0" },
        Pack { id: "tools",    name: "Tools",      swatch: "#f0c040" },
    ]
}
