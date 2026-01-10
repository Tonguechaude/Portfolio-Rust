#![allow(dead_code)]

use crate::data::stack::{get_stack, TechCategory};
use leptos::prelude::*;

#[component]
pub fn StackView(
    _selected_tags: RwSignal<Vec<String>>,
    #[prop(into)] _on_tech_click: Callback<String>,
) -> impl IntoView {
    let stack = get_stack();

    let categories = vec![
        TechCategory::Language,
        TechCategory::Framework,
        TechCategory::DevOps,
        TechCategory::Tool,
    ];

    view! {
        <div class="space-y-8">
            {categories.into_iter().map(|category| {
                let techs: Vec<_> = stack.iter()
                    .filter(|t| t.category == category)
                    .cloned()
                    .collect();

                if techs.is_empty() {
                    return view! { <div></div> }.into_any();
                }

                view! {
                    <div>
                        <h3 class="text-xl font-bold text-theme-primary mb-4">{category.label()}</h3>
                        <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
                            {techs.into_iter().map(|tech| {
                                let tech_name = tech.name.to_string();
                                let tech_name_click = tech_name.clone();
                                let on_click = _on_tech_click.clone();

                                let is_selected = Memo::new(move |_| {
                                    _selected_tags.get().contains(&tech_name.clone())
                                });

                                view! {
                                    <button
                                        on:click=move |_| {
                                            on_click.run(tech_name_click.clone());
                                        }
                                        class=move || {
                                            let base = "group relative bg-theme-nav border rounded-2xl p-4 hover:shadow-2xl hover:scale-105 transition-all duration-300 overflow-hidden text-left w-full";
                                            let border = if is_selected.get() {
                                                "border-2 border-indigo-500 dark:border-indigo-400"
                                            } else {
                                                "border-zinc-200 dark:border-zinc-700"
                                            };
                                            format!("{} {}", base, border)
                                        }
                                    >
                                        <div class=format!("absolute inset-0 bg-gradient-to-br from-{}/10 to-{}/5 opacity-0 group-hover:opacity-100 transition-opacity duration-300", tech.color.split_whitespace().next().unwrap_or("gray-500"), tech.color.split_whitespace().next().unwrap_or("gray-500"))></div>
                                        <div class="relative z-10">
                                            <div class="flex items-center gap-3 mb-2">
                                                <div class=format!("p-2 {} rounded-xl", tech.bg_color)>
                                                    <span class="text-xl">{tech.icon}</span>
                                                </div>
                                                <div>
                                                    <h4 class=format!("font-bold {}", tech.color)>{tech.name}</h4>
                                                </div>
                                            </div>
                                            <p class="text-xs text-theme-secondary">{tech.description}</p>

                                            // Indicateur "cliquez pour voir les projets"
                                            <div class="mt-2 flex items-center gap-1 text-xs text-theme-secondary opacity-0 group-hover:opacity-100 transition-opacity">
                                                <span>"Voir les projets"</span>
                                                <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                                                </svg>
                                            </div>
                                        </div>
                                    </button>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                }.into_any()
            }).collect::<Vec<_>>()}
        </div>
    }
}
