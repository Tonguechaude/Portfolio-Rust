#[derive(Clone, PartialEq)]
pub struct Tech {
    pub name: &'static str,
    pub icon: &'static str,
    pub color: &'static str,
    pub bg_color: &'static str,
    pub description: &'static str,
    pub category: TechCategory,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TechCategory {
    Language,
    Framework,
    DevOps,
    Tool,
}

#[allow(dead_code)]
impl TechCategory {
    pub fn label(&self) -> &'static str {
        match self {
            TechCategory::Language => "Langages",
            TechCategory::Framework => "Frameworks & Libs",
            TechCategory::DevOps => "DevOps",
            TechCategory::Tool => "Outils",
        }
    }
}

pub fn get_stack() -> Vec<Tech> {
    vec![
        // Langages
        Tech {
            name: "Rust",
            icon: "🦀",
            color: "text-orange-500 dark:text-orange-400",
            bg_color: "bg-orange-100 dark:bg-orange-900/30",
            description: "Mon langage de prédilection",
            category: TechCategory::Language,
        },
        Tech {
            name: "Java",
            icon: "☕",
            color: "text-red-500 dark:text-red-400",
            bg_color: "bg-red-100 dark:bg-red-900/30",
            description: "POO et applications robustes",
            category: TechCategory::Language,
        },
        Tech {
            name: "Shell",
            icon: "🐚",
            color: "text-green-500 dark:text-green-400",
            bg_color: "bg-green-100 dark:bg-green-900/30",
            description: "Automatisation et scripts",
            category: TechCategory::Language,
        },
        Tech {
            name: "SQL",
            icon: "🗃️",
            color: "text-blue-500 dark:text-blue-400",
            bg_color: "bg-blue-100 dark:bg-blue-900/30",
            description: "Bases de données relationnelles",
            category: TechCategory::Language,
        },
        // Frameworks
        Tech {
            name: "WASM",
            icon: "🌐",
            color: "text-purple-500 dark:text-purple-400",
            bg_color: "bg-purple-100 dark:bg-purple-900/30",
            description: "WebAssembly pour le web",
            category: TechCategory::Framework,
        },
        Tech {
            name: "Tailwind",
            icon: "🎨",
            color: "text-cyan-500 dark:text-cyan-400",
            bg_color: "bg-cyan-100 dark:bg-cyan-900/30",
            description: "CSS utility-first",
            category: TechCategory::Framework,
        },
        Tech {
            name: "GUI",
            icon: "🖼️",
            color: "text-pink-500 dark:text-pink-400",
            bg_color: "bg-pink-100 dark:bg-pink-900/30",
            description: "Interfaces graphiques",
            category: TechCategory::Framework,
        },
        Tech {
            name: "TUI",
            icon: "💻",
            color: "text-emerald-500 dark:text-emerald-400",
            bg_color: "bg-emerald-100 dark:bg-emerald-900/30",
            description: "Terminal User Interfaces",
            category: TechCategory::Framework,
        },
        // DevOps
        Tech {
            name: "Docker",
            icon: "🐳",
            color: "text-blue-500 dark:text-blue-400",
            bg_color: "bg-blue-100 dark:bg-blue-900/30",
            description: "Conteneurisation",
            category: TechCategory::DevOps,
        },
        Tech {
            name: "Puppet",
            icon: "🎭",
            color: "text-amber-500 dark:text-amber-400",
            bg_color: "bg-amber-100 dark:bg-amber-900/30",
            description: "Configuration as Code",
            category: TechCategory::DevOps,
        },
        Tech {
            name: "Openvox",
            icon: "🦊",
            color: "text-orange-600 dark:text-orange-400",
            bg_color: "bg-orange-100 dark:bg-orange-900/30",
            description: "Fork communautaire de Puppet",
            category: TechCategory::DevOps,
        },
        Tech {
            name: "NFtables",
            icon: "🔥",
            color: "text-red-500 dark:text-red-400",
            bg_color: "bg-red-100 dark:bg-red-900/30",
            description: "Firewall Linux moderne",
            category: TechCategory::DevOps,
        },
        Tech {
            name: "Restic",
            icon: "💾",
            color: "text-teal-500 dark:text-teal-400",
            bg_color: "bg-teal-100 dark:bg-teal-900/30",
            description: "Backup sécurisé et déduplication",
            category: TechCategory::DevOps,
        },
        // Outils
        Tech {
            name: "Jeu",
            icon: "🎮",
            color: "text-violet-500 dark:text-violet-400",
            bg_color: "bg-violet-100 dark:bg-violet-900/30",
            description: "Développement de jeux",
            category: TechCategory::Tool,
        },
    ]
}
