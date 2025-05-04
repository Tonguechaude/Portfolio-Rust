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
            tags: &["Rust", "Make", "Cargo"],
            image: "/img/cuberust.png",
            link: "https://github.com/Tonguechaude/cube.rs",
        },
        Project {
            title: "Jeu de la vie en Rust avec Bevy",
            description: "Implémentation du jeu de la vie de Conway en Rust avec Bevy et déployé sur WebAssembly",
            tags: &["Rust", "Bevy", "WASM"],
            image: "/img/gol.png",
            link: "https://github.com/Tonguechaude/GOL.rs",
        },
        Project {
            title: "Portfolio Tonguechaude",
            description: "Un site perso pour montrer mes projets",
            tags: &["Rust", "Tailwind", "Leptos", "WASM"],
            image: "/img/portfolio.png",
            link: "https://github.com/Tonguechaude/Portfolio-vue",
        },
    ]
}
