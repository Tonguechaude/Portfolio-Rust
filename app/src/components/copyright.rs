use leptos::prelude::*;

#[component]
pub fn Copyright(year: usize) -> impl IntoView {
    view! {
        <footer class="bg-theme-nav text-theme-primary text-center py-4">
            <p>
                {format!("© {} Evan Challias. Tous droits réservés.", year)}
            </p>
        </footer>
    }
}
