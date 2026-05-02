use leptos::prelude::*;

#[component]
pub fn PortfolioIndexPage() -> impl IntoView {
    view! {
        <main class="max-w-5xl mx-auto px-6 py-12">

            // ── En-tête ────────────────────────────────────────────────────────
            <section class="mb-12">
                <h1 class="text-4xl font-bold text-theme-primary mb-2">
                    "Portfolio d'apprentissage — Semestre 6"
                </h1>
                <p class="text-theme-secondary italic mb-4">
                    "BUT Informatique · Parcours Déploiement · 2025–2026"
                </p>
                <p class="text-theme-secondary">
                    // TODO : présentation personnelle, objectifs du semestre,
                    // contexte d'alternance / stage, ce que vous attendiez de ce semestre.
                    "[ À compléter : présentation et contexte ]"
                </p>
            </section>

            // ── Démarche portfolio ─────────────────────────────────────────────
            <section class="mb-12 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">
                    "Démarche portfolio"
                </h2>
                <p class="text-theme-secondary mb-2">
                    // TODO : expliquer votre approche pour construire ce portfolio.
                    // Comment avez-vous sélectionné vos traces ? Quelle posture réflexive ?
                    "[ À compléter : comment j'ai construit ce portfolio, les choix de traces, la démarche réflexive ]"
                </p>
            </section>

            // ── Grille des 6 compétences ───────────────────────────────────────
            <section class="mb-12">
                <h2 class="text-2xl font-semibold text-theme-primary mb-6">
                    "Les 6 compétences du BUT Informatique"
                </h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">

                    <a href="/portfolio/realiser">
                        <div class="p-5 border rounded-lg bg-theme-nav hover:shadow-md transition">
                            <span class="text-xs font-bold uppercase text-orange-500">"Compétence 1"</span>
                            <h3 class="text-lg font-bold text-theme-primary mt-1 mb-1">"Réaliser"</h3>
                            <p class="text-sm text-theme-secondary">"Niveau 3 — Adapter des applications sur un ensemble de supports"</p>
                        </div>
                    </a>

                    <a href="/portfolio/optimiser">
                        <div class="p-5 border rounded-lg bg-theme-nav hover:shadow-md transition">
                            <span class="text-xs font-bold uppercase text-orange-400">"Compétence 2"</span>
                            <h3 class="text-lg font-bold text-theme-primary mt-1 mb-1">"Optimiser"</h3>
                            <p class="text-sm text-theme-secondary">"Niveau 2 — Sélectionner les algorithmes adéquats"</p>
                        </div>
                    </a>

                    <a href="/portfolio/administrer">
                        <div class="p-5 border rounded-lg bg-theme-nav hover:shadow-md transition">
                            <span class="text-xs font-bold uppercase text-yellow-600">"Compétence 3"</span>
                            <h3 class="text-lg font-bold text-theme-primary mt-1 mb-1">"Administrer"</h3>
                            <p class="text-sm text-theme-secondary">"Niveau 3 — Faire évoluer et maintenir un système informatique"</p>
                        </div>
                    </a>

                    <a href="/portfolio/gerer">
                        <div class="p-5 border rounded-lg bg-theme-nav hover:shadow-md transition">
                            <span class="text-xs font-bold uppercase text-green-600">"Compétence 4"</span>
                            <h3 class="text-lg font-bold text-theme-primary mt-1 mb-1">"Gérer"</h3>
                            <p class="text-sm text-theme-secondary">"Niveau 2 — Optimiser une base de données avec sécurité"</p>
                        </div>
                    </a>

                    <a href="/portfolio/conduire">
                        <div class="p-5 border rounded-lg bg-theme-nav hover:shadow-md transition">
                            <span class="text-xs font-bold uppercase text-blue-600">"Compétence 5"</span>
                            <h3 class="text-lg font-bold text-theme-primary mt-1 mb-1">"Conduire"</h3>
                            <p class="text-sm text-theme-secondary">"Niveau 2 — Suivi de projet selon les besoins métiers"</p>
                        </div>
                    </a>

                    <a href="/portfolio/collaborer">
                        <div class="p-5 border rounded-lg bg-theme-nav hover:shadow-md transition">
                            <span class="text-xs font-bold uppercase text-gray-500">"Compétence 6"</span>
                            <h3 class="text-lg font-bold text-theme-primary mt-1 mb-1">"Collaborer"</h3>
                            <p class="text-sm text-theme-secondary">"Niveau 2 — Rôle et missions au sein d'une équipe"</p>
                        </div>
                    </a>

                </div>
            </section>

            // ── Ressources du semestre ─────────────────────────────────────────
            <section class="mb-12">
                <h2 class="text-2xl font-semibold text-theme-primary mb-4">"Ressources mobilisées"</h2>
                <div class="space-y-3">

                    <div class="p-4 border rounded-lg bg-theme-nav">
                        <h3 class="font-semibold text-theme-primary">"R6.01 — Initiation à l'entrepreneuriat"</h3>
                        <p class="text-sm text-theme-secondary mt-1">
                            "[ À compléter : ce que vous en avez retenu, liens avec vos compétences ]"
                        </p>
                    </div>

                    <div class="p-4 border rounded-lg bg-theme-nav">
                        <h3 class="font-semibold text-theme-primary">"R6.02 — Droit du numérique et de la propriété intellectuelle"</h3>
                        <p class="text-sm text-theme-secondary mt-1">
                            "[ À compléter ]"
                        </p>
                    </div>

                    <div class="p-4 border rounded-lg bg-theme-nav">
                        <h3 class="font-semibold text-theme-primary">"R6.03 — Communication : organisation et diffusion de l'information"</h3>
                        <p class="text-sm text-theme-secondary mt-1">
                            "[ À compléter ]"
                        </p>
                    </div>

                    <div class="p-4 border rounded-lg bg-theme-nav">
                        <h3 class="font-semibold text-theme-primary">"R6.04 — Projet personnel et professionnel"</h3>
                        <p class="text-sm text-theme-secondary mt-1">
                            "[ À compléter : vision de carrière, projet professionnel ]"
                        </p>
                    </div>

                    <div class="p-4 border rounded-lg bg-theme-nav">
                        <h3 class="font-semibold text-theme-primary">"R6.Deploi.05 — Optimisation des services complexes"</h3>
                        <p class="text-sm text-theme-secondary mt-1">
                            "[ À compléter ]"
                        </p>
                    </div>

                    <div class="p-4 border rounded-lg bg-theme-nav">
                        <h3 class="font-semibold text-theme-primary">"R6.Deploi.06 — Cloud computing"</h3>
                        <p class="text-sm text-theme-secondary mt-1">
                            "[ À compléter ]"
                        </p>
                    </div>

                </div>
            </section>

            // ── Bilan global ───────────────────────────────────────────────────
            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Bilan et perspectives"</h2>
                <p class="text-theme-secondary">
                    // TODO : synthèse de votre progression sur le semestre,
                    // ce que vous avez consolidé, ce qui reste à travailler,
                    // vos perspectives professionnelles.
                    "[ À compléter : bilan du semestre et projet professionnel ]"
                </p>
            </section>

        </main>
    }
}
