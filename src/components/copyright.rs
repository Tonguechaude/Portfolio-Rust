use leptos::prelude::*;

#[component]
pub fn Copyright(year: usize) -> impl IntoView {
    view! {
        <footer class="bg-gray-800 text-white text-center py-4">
            <p>
                {format!("© {} Evan Tonguechaude. Tous droits réservés.", year.to_string())}
            </p>
        </footer>
    }
}
