use leptos::prelude::*;

// Compétence 4 — Gérer
// Niveau 2 : Optimiser une base de données, interagir avec une application
//            et mettre en oeuvre la sécurité
//
// Apprentissages Critiques (AC) typiques niveau 2 :
//   AC2.01 — Optimiser les modèles de données de l'entreprise
//   AC2.02 — Assurer la confidentialité des données (RGPD, chiffrement)
//   AC2.03 — Organiser la restitution de données à travers la programmation et la visualisation
//   AC2.04 — Manipuler des données hétérogènes

#[component]
pub fn PortfolioGererPage() -> impl IntoView {
    view! {
        <main class="max-w-5xl mx-auto px-6 py-12">

            <div class="mb-8">
                <a href="/portfolio" class="text-sm text-theme-secondary hover:underline mb-4 block">
                    "← Retour au portfolio"
                </a>
                <span class="text-xs font-bold uppercase text-green-600">"Compétence 4"</span>
                <h1 class="text-4xl font-bold text-theme-primary mt-1 mb-2">"Gérer"</h1>
                <p class="text-xl text-theme-secondary italic">
                    "Niveau 2 — Optimiser une base de données, interagir avec une application et mettre en oeuvre la sécurité"
                </p>
            </div>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Mise en contexte"</h2>
                <p class="text-theme-secondary">
                    "[ À compléter : quels systèmes de données avez-vous gérés ? (SQL, NoSQL, fichiers...) ]"
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.01 — Optimiser les modèles de données"
                </h2>
                <p class="text-theme-secondary mb-4">
                    "[ À compléter : modélisation, indexation, requêtes optimisées ]"
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.02 — Assurer la confidentialité des données"
                </h2>
                <p class="text-theme-secondary mb-4">
                    "[ À compléter : RGPD, chiffrement, gestion des droits d'accès ]"
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.03 — Organiser la restitution des données"
                </h2>
                <p class="text-theme-secondary mb-4">
                    "[ À compléter : API, visualisations, rapports ]"
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.04 — Manipuler des données hétérogènes"
                </h2>
                <p class="text-theme-secondary mb-4">
                    "[ À compléter : formats différents, intégration de sources multiples ]"
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
                    // TODO : quelles ressources du semestre ont contribué à cette compétence ?
                    <li>"[ Ressource(s) concernée(s) ]"</li>
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
