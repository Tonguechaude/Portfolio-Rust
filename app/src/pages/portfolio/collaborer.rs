use leptos::prelude::*;

// Compétence 6 — Collaborer
// Niveau 2 : Situer son rôle et ses missions au sein d'une équipe informatique
//
// Apprentissages Critiques (AC) typiques niveau 2 :
//   AC2.01 — Comprendre la diversité et les rôles dans une équipe informatique
//   AC2.02 — Appliquer une démarche pour intégrer une équipe
//   AC2.03 — Mobiliser les compétences interpersonnelles (communication, écoute, partage)
//   AC2.04 — Rendre compte de son activité professionnelle
//
// Ressources mobilisées : R6.01, R6.03, R6.04, alternance

#[component]
pub fn PortfolioCollaborerPage() -> impl IntoView {
    view! {
        <main class="max-w-5xl mx-auto px-6 py-12">

            <div class="mb-8">
                <a href="/portfolio" class="text-sm text-theme-secondary hover:underline mb-4 block">
                    "← Retour au portfolio"
                </a>
                <span class="text-xs font-bold uppercase text-gray-500">"Compétence 6"</span>
                <h1 class="text-4xl font-bold text-theme-primary mt-1 mb-2">"Collaborer"</h1>
                <p class="text-xl text-theme-secondary italic">
                    "Niveau 2 — Situer son rôle et ses missions au sein d'une équipe informatique"
                </p>
            </div>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Mise en contexte"</h2>
                <p class="text-theme-secondary">
                    // TODO : décrire votre positionnement dans les équipes (alternance ADULLACT,
                    // équipes pédagogiques, projets open source...).
                    "[ À compléter : équipes dans lesquelles vous avez évolué et votre rôle ]"
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.01 — Comprendre la diversité des rôles dans une équipe informatique"
                </h2>
                <p class="text-theme-secondary mb-4">
                    "[ À compléter : quels rôles avez-vous côtoyés ? Comment interagissent-ils ? ]"
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.02 — S'intégrer dans une équipe et y trouver sa place"
                </h2>
                <p class="text-theme-secondary mb-4">
                    "[ À compléter : comment vous êtes-vous intégré en alternance / dans des projets collectifs ? ]"
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.03 — Mobiliser les compétences interpersonnelles"
                </h2>
                <p class="text-theme-secondary mb-4">
                    "[ À compléter : communication, documentation partagée, code review, pair programming... ]"
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.04 — Rendre compte de son activité professionnelle"
                </h2>
                <p class="text-theme-secondary mb-4">
                    "[ À compléter : rapports, comptes-rendus, présentations, documentation technique ]"
                </p>
            </section>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Traces et preuves"</h2>
                <ul class="list-disc list-inside space-y-2 text-theme-secondary">
                    <li>"[ Trace 1 : ex. MR/PR avec revues de code ]"</li>
                    <li>"[ Trace 2 : ex. rapport d'alternance, documentation rédigée ]"</li>
                    <li>"[ Trace 3 ]"</li>
                </ul>
            </section>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Ressources mobilisées"</h2>
                <ul class="list-disc list-inside space-y-1 text-theme-secondary">
                    <li>"R6.01 — Initiation à l'entrepreneuriat"</li>
                    <li>"R6.03 — Communication : organisation et diffusion de l'information"</li>
                    <li>"R6.04 — Projet personnel et professionnel"</li>
                </ul>
            </section>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Analyse réflexive"</h2>
                <p class="text-theme-secondary">
                    "[ À compléter : comment votre posture dans une équipe a évolué, ce que vous avez appris sur le travail collaboratif ]"
                </p>
            </section>

            <div class="flex items-center gap-3">
                <p class="text-lg font-semibold text-theme-success">"Niveau atteint : Niveau 2"</p>
                <span class="text-sm px-3 py-1 rounded-full bg-green-100 text-green-800">"[ À évaluer ]"</span>
            </div>

        </main>
    }
}
