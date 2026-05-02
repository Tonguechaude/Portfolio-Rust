use leptos::prelude::*;

// Compétence 5 — Conduire
// Niveau 2 : Appliquer une démarche de suivi de projet en fonction des besoins
//            métiers des clients et des utilisateurs
//
// Apprentissages Critiques (AC) typiques niveau 2 :
//   AC2.01 — Identifier et décrire les processus métiers d'un client
//   AC2.02 — Formaliser les besoins du client et de l'utilisateur
//   AC2.03 — Identifier les critères de faisabilité d'un projet
//   AC2.04 — Définir et mettre en œuvre une démarche de suivi de projet
//
// Ressources mobilisées : R6.03, R6.04, projet alternance

#[component]
pub fn PortfolioConduirePage() -> impl IntoView {
    view! {
        <main class="max-w-5xl mx-auto px-6 py-12">

            <div class="mb-8">
                <a href="/portfolio" class="text-sm text-theme-secondary hover:underline mb-4 block">
                    "← Retour au portfolio"
                </a>
                <span class="text-xs font-bold uppercase text-blue-600">"Compétence 5"</span>
                <h1 class="text-4xl font-bold text-theme-primary mt-1 mb-2">"Conduire"</h1>
                <p class="text-xl text-theme-secondary italic">
                    "Niveau 2 — Appliquer une démarche de suivi de projet en fonction des besoins métiers"
                </p>
            </div>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Mise en contexte"</h2>
                <p class="text-theme-secondary">
                    // TODO : quel(s) projet(s) vous ont permis de travailler cette compétence ?
                    // (alternance, SAE, projet personnel...)
                    "[ À compléter : projets conduits, méthodes utilisées (Agile, Scrum, Kanban...) ]"
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.01 — Identifier et décrire les processus métiers d'un client"
                </h2>
                <p class="text-theme-secondary mb-4">
                    "[ À compléter : comment avez-vous recueilli et modélisé les besoins ? ]"
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.02 — Formaliser les besoins du client et de l'utilisateur"
                </h2>
                <p class="text-theme-secondary mb-4">
                    "[ À compléter : user stories, cahier des charges, maquettes... ]"
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.03 — Identifier les critères de faisabilité d'un projet"
                </h2>
                <p class="text-theme-secondary mb-4">
                    "[ À compléter : analyse des contraintes techniques, temporelles, humaines ]"
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.04 — Définir et mettre en œuvre une démarche de suivi de projet"
                </h2>
                <p class="text-theme-secondary mb-4">
                    "[ À compléter : outils utilisés (Jira, GitLab Issues, Trello...), rétrospectives, livrables ]"
                </p>
                // <img alt="board" src="/img/portfolio/conduire/board.png" class="rounded-lg border shadow-lg" />
            </section>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Traces et preuves"</h2>
                <ul class="list-disc list-inside space-y-2 text-theme-secondary">
                    <li>"[ Trace 1 : ex. tableau de bord projet, burndown chart ]"</li>
                    <li>"[ Trace 2 : ex. compte-rendu de réunion client ]"</li>
                    <li>"[ Trace 3 ]"</li>
                </ul>
            </section>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Ressources mobilisées"</h2>
                <ul class="list-disc list-inside space-y-1 text-theme-secondary">
                    <li>"R6.03 — Communication : organisation et diffusion de l'information"</li>
                    <li>"R6.04 — Projet personnel et professionnel"</li>
                </ul>
            </section>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Analyse réflexive"</h2>
                <p class="text-theme-secondary">
                    "[ À compléter : ce que cette compétence vous a apporté, difficultés de coordination, apprentissages sur la gestion de projet ]"
                </p>
            </section>

            <div class="flex items-center gap-3">
                <p class="text-lg font-semibold text-theme-success">"Niveau atteint : Niveau 2"</p>
                <span class="text-sm px-3 py-1 rounded-full bg-green-100 text-green-800">"[ À évaluer ]"</span>
            </div>

        </main>
    }
}
