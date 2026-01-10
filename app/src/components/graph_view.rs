use crate::data::stack::{get_stack, TechCategory};
use leptos::prelude::*;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Clone)]
struct Node {
    name: String,
    x: f32,
    y: f32,
    icon: String,
}

#[derive(Clone)]
struct Edge {
    from: String,
    to: String,
}

// Connexions manuelles basées sur les relations logiques entre technos
fn get_connections() -> Vec<(&'static str, &'static str)> {
    vec![
        // Rust et son écosystème
        ("Rust", "WASM"),
        ("Rust", "TUI"),
        ("Rust", "GUI"),
        ("Rust", "Docker"),
        // DevOps chain
        ("Puppet", "Openvox"),
        ("Puppet", "Docker"),
        ("Docker", "NFtables"),
        ("Restic", "Docker"),
        ("NFtables", "Restic"),
        // Web stack
        ("WASM", "Tailwind"),
        ("Tailwind", "GUI"),
        // Languages connexions
        ("Java", "GUI"),
        ("Shell", "Docker"),
        ("Shell", "Puppet"),
        ("SQL", "Docker"),
        // Jeux
        ("GUI", "Jeu"),
        ("Rust", "Jeu"),
    ]
}

#[component]
pub fn GraphView(
    selected_tags: RwSignal<Vec<String>>,
    #[prop(into)] on_tech_click: Callback<String>,
) -> impl IntoView {
    let stack = get_stack();
    let edges: Vec<Edge> = get_connections()
        .into_iter()
        .map(|(from, to)| Edge {
            from: from.to_string(),
            to: to.to_string(),
        })
        .collect();

    // Grouper par catégorie et positionner
    let mut nodes: Vec<Node> = Vec::new();

    // Positions par catégorie (en forme de clusters)
    let category_positions: HashMap<&str, (f32, f32, f32)> = [
        ("Language", (200.0, 200.0, 80.0)),    // gauche haut
        ("Framework", (600.0, 200.0, 80.0)),   // droite haut
        ("DevOps", (400.0, 450.0, 100.0)),     // centre bas
        ("Tool", (400.0, 150.0, 50.0)),        // centre haut
    ]
    .into_iter()
    .collect();

    for category in [TechCategory::Language, TechCategory::Framework, TechCategory::DevOps, TechCategory::Tool] {
        let techs_in_cat: Vec<_> = stack.iter().filter(|t| t.category == category).collect();
        let cat_key = match category {
            TechCategory::Language => "Language",
            TechCategory::Framework => "Framework",
            TechCategory::DevOps => "DevOps",
            TechCategory::Tool => "Tool",
        };

        if let Some(&(cx, cy, radius)) = category_positions.get(cat_key) {
            let count = techs_in_cat.len();
            for (i, tech) in techs_in_cat.into_iter().enumerate() {
                let angle = (i as f32 / count as f32) * 2.0 * std::f32::consts::PI - std::f32::consts::PI / 2.0;
                nodes.push(Node {
                    name: tech.name.to_string(),
                    x: cx + radius * angle.cos(),
                    y: cy + radius * angle.sin(),
                    icon: tech.icon.to_string(),
                });
            }
        }
    }

    let node_map: HashMap<String, Node> = nodes.iter().map(|n| (n.name.clone(), n.clone())).collect();

    view! {
        <div class="flex flex-col items-center">
            <svg
                viewBox="0 0 800 600"
                class="w-full max-w-4xl h-auto"
                style="min-height: 500px;"
            >
                // Définitions pour les effets
                <defs>
                    <filter id="glow">
                        <feGaussianBlur stdDeviation="3" result="coloredBlur"/>
                        <feMerge>
                            <feMergeNode in="coloredBlur"/>
                            <feMergeNode in="SourceGraphic"/>
                        </feMerge>
                    </filter>
                </defs>

                // Dessiner les connexions (edges)
                {edges.iter().filter_map(|edge| {
                    let from_node = node_map.get(&edge.from)?;
                    let to_node = node_map.get(&edge.to)?;

                    Some(view! {
                        <line
                            x1=from_node.x
                            y1=from_node.y
                            x2=to_node.x
                            y2=to_node.y
                            stroke="currentColor"
                            class="text-indigo-400 dark:text-indigo-500"
                            stroke-width="1.5"
                            opacity="0.4"
                        />
                    })
                }).collect::<Vec<_>>()}

                // Dessiner les nœuds
                {nodes.iter().map(|node| {
                    let node_name = node.name.clone();
                    let node_name_click = node.name.clone();
                    let node_name_selected = node.name.clone();
                    let on_click = on_tech_click.clone();
                    let icon = node.icon.clone();
                    let x = node.x;
                    let y = node.y;

                    let is_selected = Memo::new(move |_| {
                        selected_tags.get().contains(&node_name_selected)
                    });

                    view! {
                        <g
                            class="cursor-pointer transition-transform duration-200 hover:scale-110"
                            style="transform-origin: center; transform-box: fill-box;"
                            on:click=move |_| {
                                on_click.run(node_name_click.clone());
                            }
                        >
                            // Cercle de fond avec glow si sélectionné
                            <circle
                                cx=x
                                cy=y
                                r=move || if is_selected.get() { 38 } else { 35 }
                                class=move || {
                                    if is_selected.get() {
                                        "fill-indigo-500 dark:fill-indigo-600"
                                    } else {
                                        "fill-zinc-100 dark:fill-zinc-800"
                                    }
                                }
                                stroke=move || if is_selected.get() { "#6366f1" } else { "#a1a1aa" }
                                stroke-width=move || if is_selected.get() { 3 } else { 2 }
                                filter=move || if is_selected.get() { "url(#glow)" } else { "" }
                            />

                            // Icône emoji
                            <text
                                x=x
                                y=y - 5.0
                                text-anchor="middle"
                                dominant-baseline="middle"
                                class="text-2xl pointer-events-none select-none"
                            >
                                {icon}
                            </text>

                            // Nom de la techno
                            <text
                                x=x
                                y=y + 18.0
                                text-anchor="middle"
                                dominant-baseline="middle"
                                class=move || {
                                    if is_selected.get() {
                                        "text-xs font-bold fill-white pointer-events-none select-none"
                                    } else {
                                        "text-xs font-medium fill-zinc-600 dark:fill-zinc-300 pointer-events-none select-none"
                                    }
                                }
                            >
                                {node_name.clone()}
                            </text>
                        </g>
                    }
                }).collect::<Vec<_>>()}
            </svg>

            // Légende
            <div class="mt-4 text-center text-sm text-theme-secondary">
                <p>"Ma stack technique et les relations entre les technos"</p>
                <p class="text-xs mt-1 opacity-70">"Cliquez sur une techno pour filtrer les projets"</p>
            </div>
        </div>
    }
}
