#[derive(Clone, PartialEq)]
pub struct Project {
    pub title: &'static str,
    pub description: &'static str,
    pub tags: &'static [&'static str],
    pub image: &'static str,
    pub link: &'static str,
    pub gradient_from: &'static str,
    pub gradient_to: &'static str,
}

pub fn get_projects() -> Vec<Project> {
    vec![
        Project {
            title: "Cube.rs",
            description: "Juste un cube qui tourne en Rust",
            tags: &["Rust", "TUI"],
            image: "/img/cuberust.png",
            link: "https://github.com/Tonguechaude/cube.rs",
            gradient_from: "from-emerald-500/10",
            gradient_to: "to-cyan-500/10",
        },
        Project {
            title: "Jeu de la vie en Rust avec Bevy",
            description: "Implémentation du jeu de la vie de Conway en Rust avec Bevy et déployé sur WebAssembly",
            tags: &["Rust", "GUI", "WASM", "Jeu", "⭐"],
            image: "/img/gol.png",
            link: "https://github.com/Tonguechaude/GOL.rs",
            gradient_from: "from-orange-500/10",
            gradient_to: "to-rose-500/10",
        },
        Project {
            title: "Portfolio Tonguechaude",
            description: "Un site perso pour montrer mes projets",
            tags: &["Rust", "Web", "Tailwind", "WASM", "⭐"],
            image: "/img/portfolio-rust.png",
            link: "https://github.com/Tonguechaude/Portfolio-rust",
            gradient_from: "from-violet-500/10",
            gradient_to: "to-purple-500/10",
        },
        Project {
            title: "Aquatui",
            description: "Un clone personnalisé de asciiquarium pour se détendre en regardant les pwassonnnns !",
            tags: &["Rust", "TUI", "⭐"],
            image: "/img/aquatui.png",
            link: "https://github.com/Tonguechaude/Aquatui",
            gradient_from: "from-cyan-500/10",
            gradient_to: "to-blue-500/10",
        },
        Project {
            title: "Convertisseur Rust",
            description: "Convertisseur Hexa/Decimal/Octal/Binaire en Rust et dockerisée",
            tags: &["Rust", "Docker", "TUI"],
            image: "/img/convertisseur-rust.png",
            link: "https://github.com/Tonguechaude/Convertisseur-Rust",
            gradient_from: "from-blue-500/10",
            gradient_to: "to-orange-500/10",
        },
        Project {
            title: "2048 Java",
            description: "Un 2048 en Java avec une interface graphique",
            tags: &["Java", "GUI"],
            image: "/img/2048.png",
            link: "https://github.com/Tonguechaude/2048",
            gradient_from: "from-amber-500/10",
            gradient_to: "to-orange-500/10",
        },
    ]
}
