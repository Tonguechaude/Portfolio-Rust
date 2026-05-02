use leptos::prelude::*;

// Compétence 1 — Réaliser
// Niveau 3 : Adapter des applications sur un ensemble de supports (embarqué, web, mobile, IoT...)
//
// Apprentissages Critiques (AC) typiques niveau 3 :
//   AC3.01 — Concevoir et développer des applications communicantes
//   AC3.02 — Utiliser des frameworks et bibliothèques adaptés au support cible
//   AC3.03 — Adapter l'application aux contraintes spécifiques (IoT, embarqué, mobile, web)
//   AC3.04 — Assurer la qualité et la maintenabilité du code (tests, documentation)
//
// Ressources mobilisées : R6.Deploi.05, R6.Deploi.06
// Traces possibles : ce portfolio (Leptos/WASM), projets alternance, SAE

#[component]
pub fn PortfolioRealiserPage() -> impl IntoView {
    view! {
        <main class="max-w-5xl mx-auto px-6 py-12">

            // ── En-tête compétence ─────────────────────────────────────────────
            <div class="mb-8">
                <a href="/portfolio" class="text-sm text-theme-secondary hover:underline mb-4 block">
                    "← Retour au portfolio"
                </a>
                <span class="text-xs font-bold uppercase text-orange-500">"Compétence 1"</span>
                <h1 class="text-4xl font-bold text-theme-primary mt-1 mb-2">"Réaliser"</h1>
                <p class="text-xl text-theme-secondary italic">
                    "Niveau 3 — Adapter des applications sur un ensemble de supports (embarqué, web, mobile, IoT...)"
                </p>
            </div>

            // ── Mise en contexte ───────────────────────────────────────────────
            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Mise en contexte"</h2>
                <p class="text-theme-secondary">
                    // TODO : comment cette compétence s'est manifestée dans vos projets/alternance.
                    // Quels supports avez-vous abordés ? Quels défis spécifiques à chaque support ?
                    "[ À compléter ]"
                </p>
            </section>

            // ── AC 1 ───────────────────────────────────────────────────────────
            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC3.01 — Concevoir et développer des applications communicantes"
                </h2>
                <p class="text-theme-secondary mb-4">
                    // TODO : décrire une situation concrète, les choix techniques, le résultat.
                    "[ À compléter : situation, action, résultat ]"
                </p>
                // <img alt="capture" src="/img/portfolio/realiser/ac1.png" class="rounded-lg border shadow-lg" />
            </section>

            // ── AC 2 ───────────────────────────────────────────────────────────
            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC3.02 — Utiliser des frameworks adaptés au support cible"
                </h2>
                <p class="text-theme-secondary mb-4">
                    "[ À compléter ]"
                </p>
            </section>

            // ── AC 3 ───────────────────────────────────────────────────────────
            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC3.03 — Adapter l'application aux contraintes spécifiques"
                </h2>
                <p class="text-theme-secondary mb-4">
                    "[ À compléter : contraintes IoT, embarqué, web, mobile rencontrées ]"
                </p>
            </section>

            // ── AC 4 ───────────────────────────────────────────────────────────
            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC3.04 — Assurer la qualité et la maintenabilité du code"
                </h2>
                <p class="text-theme-secondary mb-4">
                    "[ À compléter : tests, documentation, revue de code ]"
                </p>
            </section>

            // ── Traces et preuves ─────────────────────────────────────────────
            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Traces et preuves"</h2>
                <ul class="list-disc list-inside space-y-2 text-theme-secondary">
                    <li>"[ Trace 1 : lien, dépôt, capture d'écran... ]"</li>
                    <li>"[ Trace 2 ]"</li>
                    <li>"[ Trace 3 ]"</li>
                </ul>
            </section>

            // ── Ressources mobilisées ─────────────────────────────────────────
            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Ressources mobilisées"</h2>
                <ul class="list-disc list-inside space-y-1 text-theme-secondary">
                    <li>"R6.Deploi.05 — Optimisation des services complexes"</li>
                    <li>"R6.Deploi.06 — Cloud computing"</li>
                    // TODO : ajouter d'autres ressources pertinentes
                </ul>
            </section>

            // ── Analyse réflexive ─────────────────────────────────────────────
            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Analyse réflexive"</h2>
                <p class="text-theme-secondary">
                    // TODO : qu'est-ce que vous avez appris ? Qu'est-ce qui a été difficile ?
                    // Comment vous êtes-vous amélioré ? Que feriez-vous différemment ?
                    "[ À compléter : regard critique sur vos apprentissages ]"
                </p>
            </section>

            // ── Auto-évaluation ───────────────────────────────────────────────
            <div class="flex items-center gap-3">
                <p class="text-lg font-semibold text-theme-success">"Niveau atteint : Niveau 3"</p>
                // TODO : changer en "En cours d'acquisition" / "Acquis" / "Maîtrisé" selon votre auto-évaluation
                <span class="text-sm px-3 py-1 rounded-full bg-green-100 text-green-800">"[ À évaluer ]"</span>
            </div>

        </main>
    }
}
