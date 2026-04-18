pub struct Pack {
    pub id: &'static str,
    pub name: &'static str,
    pub swatch: &'static str,
}

pub fn all_packs() -> Vec<Pack> {
    vec![
        Pack { id: "all",      name: "All Cards",  swatch: "#c9a34a" },
        Pack { id: "frontend", name: "Frontend",   swatch: "#e8a04a" },
        Pack { id: "backend",  name: "Backend",    swatch: "#e8602b" },
        Pack { id: "fullstack",name: "Fullstack",  swatch: "#f2c15a" },
        Pack { id: "runtime",  name: "Runtime",    swatch: "#c47a28" },
        Pack { id: "data",     name: "Data",       swatch: "#b85a20" },
        Pack { id: "api",      name: "API",        swatch: "#d89050" },
        Pack { id: "view",     name: "View",       swatch: "#a65f22" },
        Pack { id: "core",     name: "Core",       swatch: "#d4421a" },
        Pack { id: "tools",    name: "Tools",      swatch: "#f0bd5a" },
    ]
}
