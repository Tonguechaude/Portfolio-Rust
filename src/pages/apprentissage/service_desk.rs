use leptos::prelude::*;

#[component]
pub fn TicketingPage() -> impl IntoView {
    view! {
        <main class="max-w-5xl mx-auto px-6 py-12">
            <h1 class="text-4xl font-bold mb-4">"Outil de ticketing — Alternance ADULLACT"</h1>
            <p class="mb-4 italic">"Projet professionnel — 6 mois"</p>
            <p class="mb-2">
                "Développement en solo avec un chef de projet. J’ai géré la conception, l’architecture, le back et front-end, ainsi que les tests."
            </p>
            <p class="mb-2">
                "Méthode agile, suivis réguliers, démonstrations, prise en compte de retours, et documentation complète."
            </p>
        </main>
    }
}
