use leptos::prelude::*;

// Compétence 2 — Optimiser
// Niveau 2 : Sélectionner les algorithmes adéquats pour répondre à un problème donné
//
// Apprentissages Critiques (AC) typiques niveau 2 :
//   AC2.01 — Analyser un problème et choisir une structure de données adaptée
//   AC2.02 — Comparer des solutions algorithmiques selon leur complexité
//   AC2.03 — S'assurer de la sécurité des données et du code
//
// Ressources mobilisées : cours d'algorithmique, R6.Deploi.05

#[component]
pub fn PortfolioOptimiserPage() -> impl IntoView {
    view! {
        <main class="max-w-5xl mx-auto px-6 py-12">

            <div class="mb-8">
                <a href="/portfolio" class="text-sm text-theme-secondary hover:underline mb-4 block">
                    "← Retour au portfolio"
                </a>
                <span class="text-xs font-bold uppercase text-orange-400">"Compétence 2"</span>
                <h1 class="text-4xl font-bold text-theme-primary mt-1 mb-2">"Optimiser"</h1>
                <p class="text-xl text-theme-secondary italic">
                    "Niveau 2 — Sélectionner les algorithmes adéquats pour répondre à un problème donné"
                </p>
            </div>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Mise en contexte"</h2>
                <p class="text-theme-secondary">
                    "[ À compléter : dans quels projets avez-vous eu à choisir ou optimiser des algorithmes ? ]"
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.01 — Analyser un problème et choisir une structure de données adaptée"
                </h2>
                <p class="text-theme-secondary mb-4">
                    "[ À compléter : situation, choix de structure, justification ]"
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.02 — Comparer des solutions selon leur complexité"
                </h2>
                <p class="text-theme-secondary mb-4">
                    "[ À compléter : exemple d'analyse de complexité, benchmark, comparaison de solutions ]"
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.03 — S'assurer de la sécurité des données et du code"
                </h2>
                <p class="text-theme-secondary mb-4">
                    "[ À compléter : mesures de sécurité algorithmique prises ]"
                </p>
            </section>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Traces et preuves"</h2>
                <ul class="list-disc list-inside space-y-2 text-theme-secondary">
                    <li>"[ Trace 1 ]"</li>
                    <li>"[ Trace 2 ]"</li>
                </ul>
            </section>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Ressources mobilisées"</h2>
                <ul class="list-disc list-inside space-y-1 text-theme-secondary">
                    <li>"R6.Deploi.05 — Optimisation des services complexes"</li>
                    // TODO : autres ressources
                </ul>
            </section>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Analyse réflexive"</h2>
                <p class="text-theme-secondary">
                    "[ À compléter ]"
                </p>
            </section>

            <div class="flex items-center gap-3">
                <p class="text-lg font-semibold text-theme-success">"Niveau atteint : Niveau 2"</p>
                <span class="text-sm px-3 py-1 rounded-full bg-green-100 text-green-800">"[ À évaluer ]"</span>
            </div>

        </main>
    }
}
