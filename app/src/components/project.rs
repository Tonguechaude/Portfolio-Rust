use crate::data::projects::get_projects;
use leptos::prelude::*;

#[component]
pub fn Projects(selected_tags: RwSignal<Vec<String>>) -> impl IntoView {
    let projects = get_projects();

    let filtered_projects = Memo::new(move |_| {
        let selected = selected_tags.get();
        if selected.is_empty() {
            projects.clone()
        } else {
            projects
                .clone()
                .into_iter()
                .filter(|p| selected.iter().any(|tag| p.tags.contains(&tag.as_str())))
                .collect()
        }
    });

    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {move || filtered_projects.get().into_iter().map(|p| {
                let gradient_class = format!(
                    "absolute inset-0 bg-gradient-to-br {} {} opacity-0 group-hover:opacity-100 transition-opacity duration-300",
                    p.gradient_from, p.gradient_to
                );

                view! {
                    <a
                        href=p.link
                        target="_blank"
                        class="group relative bg-theme-nav border border-zinc-200 dark:border-zinc-700 rounded-2xl overflow-hidden hover:shadow-2xl hover:scale-105 transition-all duration-300"
                    >
                        <div class=gradient_class></div>

                        <div class="relative z-10">
                            <div class="relative overflow-hidden">
                                <img
                                    src=p.image
                                    alt=p.title
                                    class="w-full h-48 object-cover transition-transform duration-300 group-hover:scale-110"
                                />
                                <div class="absolute inset-0 bg-gradient-to-t from-black/20 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                            </div>

                            <div class="p-6 text-center">
                                <h2 class="text-xl font-bold mb-3 text-theme-primary group-hover:text-indigo-500 dark:group-hover:text-indigo-400 transition-colors">
                                    {p.title}
                                </h2>
                                <p class="text-theme-secondary text-sm mb-4 line-clamp-2">
                                    {p.description}
                                </p>
                                <div class="flex flex-wrap gap-2 justify-center">
                                    {
                                        p.tags.iter().map(|tag| {
                                            view! {
                                                <span class="text-xs bg-zinc-100 dark:bg-zinc-800 text-theme-secondary px-3 py-1 rounded-full border border-zinc-200 dark:border-zinc-700">
                                                    {*tag}
                                                </span>
                                            }
                                        }).collect::<Vec<_>>()
                                    }
                                </div>

                                <div class="mt-4 flex items-center justify-center gap-1 text-sm text-indigo-500 dark:text-indigo-400 opacity-0 group-hover:opacity-100 transition-opacity">
                                    <span>"Voir le projet"</span>
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"/>
                                    </svg>
                                </div>
                            </div>
                        </div>
                    </a>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
