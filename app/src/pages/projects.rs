use crate::components::project::Projects;
use crate::components::tagfilter::TagFilter;
use leptos::prelude::*;

#[component]
pub fn ProjectPage() -> impl IntoView {
    let selected_tags = RwSignal::new(vec![]);

    let all_tags = vec![
        "Rust", "TUI", "GUI", "WASM", "Tailwind", "Docker", "Java", "Jeu", "⭐",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect::<Vec<_>>();

    view! {
        <div class="bg-gray-50 min-h-screen py-12 px-6">
            <div class="max-w-5xl mx-auto text-center mb-6">
                <h1 class="text-4xl md:text-5xl font-extrabold text-gray-800 mb-4">
                    "Creusons dans mon atelier!"
                </h1>
                <p class="text-lg text-gray-600 mb-6">
                    "Voici quelques projets sur lesquels j'ai travaillé. Filtrez-les par technologie !"
                </p>

                <TagFilter tags=all_tags selected_tags=selected_tags />
            </div>

            <div class="max-w-6xl mx-auto">
                <Projects selected_tags=selected_tags />
            </div>
        </div>
    }
}
