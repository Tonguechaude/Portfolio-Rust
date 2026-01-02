use crate::components::graph_view::GraphView;
use crate::components::project::Projects;
use crate::components::tagfilter::TagFilter;
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum ViewMode {
    Projects,
    Stack,
}

#[component]
pub fn ProjectPage() -> impl IntoView {
    let selected_tags = RwSignal::new(vec![]);
    let view_mode = RwSignal::new(ViewMode::Projects);

    let all_tags = vec![
        "Rust", "TUI", "GUI", "WASM", "Tailwind", "Docker", "Java", "Jeu", "⭐",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect::<Vec<_>>();

    let on_tech_click = Callback::new(move |tech_name: String| {
        let current = selected_tags.get();
        if current.contains(&tech_name) {
            selected_tags.set(current.into_iter().filter(|t| t != &tech_name).collect());
        } else {
            selected_tags.set(vec![tech_name]);
        }
        view_mode.set(ViewMode::Projects);
    });

    view! {
        <div class="bg-theme-primary min-h-screen py-12 px-6">
            <div class="max-w-5xl mx-auto text-center mb-6">
                <h1 class="text-4xl md:text-5xl font-extrabold text-theme-primary mb-4">
                    "Creusons dans mon atelier!"
                </h1>
                <p class="text-lg text-theme-secondary mb-6">
                    {move || match view_mode.get() {
                        ViewMode::Projects => "Voici quelques projets sur lesquels j'ai travaillé. Filtrez-les par technologie !",
                        ViewMode::Stack => "Ma stack technique. Cliquez sur une techno pour voir les projets associés !",
                    }}
                </p>

                // Toggle buttons
                <div class="flex justify-center gap-2 mb-6">
                    <button
                        on:click=move |_| view_mode.set(ViewMode::Projects)
                        class=move || {
                            let base = "px-4 py-2 rounded-lg font-medium transition-all duration-200";
                            if view_mode.get() == ViewMode::Projects {
                                format!("{} bg-indigo-500 text-white shadow-lg", base)
                            } else {
                                format!("{} bg-theme-nav text-theme-secondary hover:bg-theme-card", base)
                            }
                        }
                    >
                        <span class="flex items-center gap-2">
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"/>
                            </svg>
                            "Projets"
                        </span>
                    </button>
                    <button
                        on:click=move |_| view_mode.set(ViewMode::Stack)
                        class=move || {
                            let base = "px-4 py-2 rounded-lg font-medium transition-all duration-200";
                            if view_mode.get() == ViewMode::Stack {
                                format!("{} bg-indigo-500 text-white shadow-lg", base)
                            } else {
                                format!("{} bg-theme-nav text-theme-secondary hover:bg-theme-card", base)
                            }
                        }
                    >
                        <span class="flex items-center gap-2">
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/>
                            </svg>
                            "Stack"
                        </span>
                    </button>
                </div>

                // Tag filter (only in Projects view)
                <div class=move || if view_mode.get() == ViewMode::Projects { "" } else { "hidden" }>
                    <TagFilter tags=all_tags.clone() selected_tags=selected_tags />
                </div>
            </div>

            <div class="max-w-6xl mx-auto">
                {move || match view_mode.get() {
                    ViewMode::Projects => view! {
                        <Projects selected_tags=selected_tags />
                    }.into_any(),
                    ViewMode::Stack => view! {
                        <GraphView selected_tags=selected_tags on_tech_click=on_tech_click />
                    }.into_any(),
                }}
            </div>
        </div>
    }
}
