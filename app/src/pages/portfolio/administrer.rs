use leptos::prelude::*;

// Compétence 3 — Administrer
// Niveau 3 : Faire évoluer et maintenir un système informatique communicant
//            en conditions opérationnelles
//
// Apprentissages Critiques (AC) typiques niveau 3 :
//   AC3.01 — Concevoir et déployer des services dans une infrastructure complexe
//   AC3.02 — Sécuriser les services et superviser leur fonctionnement
//   AC3.03 — Faire évoluer un système en production (zero-downtime, migrations)
//   AC3.04 — Automatiser l'administration (IaC, CI/CD, scripts)
//
// Ressources mobilisées : R6.Deploi.05, R6.Deploi.06
// C'est la compétence cœur du parcours Déploiement

#[component]
pub fn PortfolioAdministrerPage() -> impl IntoView {
    view! {
        <main class="max-w-5xl mx-auto px-6 py-12">

            <div class="mb-8">
                <a href="/portfolio" class="text-sm text-theme-secondary hover:underline mb-4 block">
                    "← Retour au portfolio"
                </a>
                <span class="text-xs font-bold uppercase text-yellow-600">"Compétence 3"</span>
                <h1 class="text-4xl font-bold text-theme-primary mt-1 mb-2">"Administrer"</h1>
                <p class="text-xl text-theme-secondary italic">
                    "Niveau 3 — Faire évoluer et maintenir un système informatique communicant en conditions opérationnelles"
                </p>
            </div>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Mise en contexte"</h2>
                <p class="text-theme-secondary">
                    // TODO : décrire votre environnement (alternance ADULLACT, VPS, infrastructure...).
                    // Quels systèmes avez-vous administrés ? Quelle responsabilité ?
                    "[ À compléter : contexte d'administration (alternance, VPS perso, etc.) ]"
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC3.01 — Concevoir et déployer des services dans une infrastructure complexe"
                </h2>
                <p class="text-theme-secondary mb-4">
                    "[ À compléter : ex. déploiement sur VPS, configuration Nginx, services systemd... ]"
                </p>
                // <img alt="infra" src="/img/portfolio/administrer/infra.png" class="rounded-lg border shadow-lg" />
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC3.02 — Sécuriser les services et superviser leur fonctionnement"
                </h2>
                <p class="text-theme-secondary mb-4">
                    "[ À compléter : pare-feu, TLS, monitoring, alertes... ]"
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC3.03 — Faire évoluer un système en production"
                </h2>
                <p class="text-theme-secondary mb-4">
                    "[ À compléter : ex. migrations, mises à jour sans interruption, rollbacks... ]"
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC3.04 — Automatiser l'administration (IaC, CI/CD, scripts)"
                </h2>
                <p class="text-theme-secondary mb-4">
                    "[ À compléter : ex. Puppet, GitLab CI, scripts Bash/Python/Rust d'automatisation ]"
                </p>
            </section>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Traces et preuves"</h2>
                <ul class="list-disc list-inside space-y-2 text-theme-secondary">
                    <li>"[ Trace 1 : ex. pipeline CI/CD GitLab, lien dépôt ]"</li>
                    <li>"[ Trace 2 : ex. configuration Nginx, Dockerfile ]"</li>
                    <li>"[ Trace 3 ]"</li>
                </ul>
            </section>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Ressources mobilisées"</h2>
                <ul class="list-disc list-inside space-y-1 text-theme-secondary">
                    <li>"R6.Deploi.05 — Optimisation des services complexes"</li>
                    <li>"R6.Deploi.06 — Cloud computing"</li>
                </ul>
            </section>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Analyse réflexive"</h2>
                <p class="text-theme-secondary">
                    "[ À compléter : montée en compétence sur l'administration système, difficultés rencontrées, ce que vous avez acquis ]"
                </p>
            </section>

            <div class="flex items-center gap-3">
                <p class="text-lg font-semibold text-theme-success">"Niveau atteint : Niveau 3"</p>
                <span class="text-sm px-3 py-1 rounded-full bg-green-100 text-green-800">"[ À évaluer ]"</span>
            </div>

        </main>
    }
}
