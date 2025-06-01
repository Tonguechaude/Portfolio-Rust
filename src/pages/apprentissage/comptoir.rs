use leptos::prelude::*;

#[component]
pub fn ComptoirPage() -> impl IntoView {
    view! {
        <main class="max-w-5xl mx-auto px-6 py-12">
            <h1 class="text-4xl font-bold mb-4">"Audit automatisé — Comptoir du Libre"</h1>
            <p class="mb-4 italic">"Projet d'alternance — 2 semaines"</p>
            <p class="mb-2">
                "Projet en autonomie à la demande du développeur principal. Scan de tous les logiciels via API pour identifier ceux obsolètes ou non maintenus."
            </p>
            <p class="mb-2">
                "Déploiement rapide, tests unitaires, interprétation des résultats, et documentation à destination du Comptoir."
            </p>
        </main>
    }
}
