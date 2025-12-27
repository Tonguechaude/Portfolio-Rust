use leptos::prelude::*;

#[component]
pub fn TagFilter(tags: Vec<String>, selected_tags: RwSignal<Vec<String>>) -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-2 justify-center mb-8">
            {tags.into_iter().map(move |tag| {
                let tag_clone1 = tag.clone(); // pour is_selected
                let tag_clone2 = tag.clone(); // pour onclick

                let is_selected = move || selected_tags.get().contains(&tag_clone1);
                let onclick = move |_| {
                    let mut current = selected_tags.get();
                    if current.contains(&tag_clone2) {
                        current.retain(|t| t != &tag_clone2);
                    } else {
                        current.push(tag_clone2.clone());
                    }
                    selected_tags.set(current);
                };

                view! {
                    <button
                        class=move || {
                            if is_selected() {
                                "px-4 py-2 rounded-full bg-blue-600 text-white border border-blue-600 transition"
                            } else {
                                "px-4 py-2 rounded-full bg-white text-gray-800 border border-gray-300 transition hover:bg-gray-100"
                            }
                        }
                        on:click=onclick
                    >
                        {tag}
                    </button>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
