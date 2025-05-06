#[derive(Clone, PartialEq)]
pub struct Project {
    pub title: &'static str,
    pub description: &'static str,
    pub tags: &'static [&'static str],
    pub image: &'static str,
    pub link: &'static str,
}

pub fn get_projects() -> Vec<Project> {
    vec![
        Project {
            title: "Cube.rs",
            description: "Juste un cube qui tourne en Rust",
            tags: &["Rust", "TUI"],
            image: "/img/cuberust.png",
            link: "https://github.com/Tonguechaude/cube.rs",
        },
        Project {
            title: "Jeu de la vie en Rust avec Bevy",
            description: "Implémentation du jeu de la vie de Conway en Rust avec Bevy et déployé sur WebAssembly",
            tags: &["Rust", "GUI", "WASM", "Jeu"],
            image: "/img/gol.png",
            link: "https://github.com/Tonguechaude/GOL.rs",
        },
        Project {
            title: "Portfolio Tonguechaude",
            description: "Un site perso pour montrer mes projets",
            tags: &["Rust", "Web", "Tailwind", "WASM"],
            image: "/img/portfolio-rust.png",
            link: "https://github.com/Tonguechaude/Portfolio-rust",
        },
        Project {
            title: "Aquatui",
            description: "Un clone personnalisé de asciiquarium pour se détendre en regardant les pwassonnnns !",
            tags: &["Rust", "TUI"],
            image: "/img/aquatui.png",
            link: "https://github.com/Tonguechaude/Aquatui",
        },
        Project {
            title: "Convertisseur Rust",
            description: "Convertisseur Hexa/Decimal/Octal/Binaire en Rust et dockerisée",
            tags: &["Rust", "Docker", "TUI"],
            image: "/img/convertisseur-rust.png",
            link: "https://github.com/Tonguechaude/Convertisseur-Rust",
        },
        Project {
            title: "2048 Java",
            description: "Un 2048 en Java avec une interface graphique",
            tags: &["Java", "GUI"],
            image: "/img/2048.png",
            link: "https://github.com/Tonguechaude/2048",
        },
    ]
}
