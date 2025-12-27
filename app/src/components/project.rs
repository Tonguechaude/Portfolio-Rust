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
                view! {
                    <a href=p.link target="_blank" class="block bg-white p-4 rounded shadow hover:shadow-lg transition">
                        <img src=p.image alt=p.title class="w-full h-48 object-cover rounded mb-4" />
                        <h2 class="text-xl font-bold mb-2">{p.title}</h2>
                        <p class="text-gray-600 mb-2">{p.description}</p>
                        <div class="flex flex-wrap gap-1 mt-2">
                            {
                                p.tags.iter().map(|tag| {
                                    view! {
                                        <span class="text-xs bg-gray-200 text-gray-700 px-2 py-1 rounded-full">
                                            {*tag}
                                        </span>
                                    }
                                }).collect::<Vec<_>>()
                            }
                        </div>
                    </a>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
