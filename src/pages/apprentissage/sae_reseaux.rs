use leptos::prelude::*;

#[component]
pub fn SaePage() -> impl IntoView {
    view! {
        <main class="max-w-5xl mx-auto px-6 py-12">
            <h1 class="text-4xl font-bold mb-4">"SAE : Logiciel de sondage en réseau"</h1>
            <p class="mb-4 italic">"2ᵉ année BUT Informatique — DACS"</p>
            <p class="mb-2">
                "Travail en équipe de 5 personnes en méthode AGILE. Développement d’un logiciel de sondage avec un focus sur la cryptographie."
            </p>
            <p class="mb-2">
                "J’ai pu y approfondir mes compétences en réseau, en sécurité, et en travail collaboratif sous pression."
            </p>
        </main>
    }
}
