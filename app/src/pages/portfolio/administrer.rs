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
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Dans le cadre de mon alternance de deux ans à l'ADULLACT, j'ai été intégré à l'équipe \
                    technique en charge de l'infrastructure hébergeant l'ensemble des services et initiatives \
                    de l'association. Ce parc est composé de 37 serveurs sous Ubuntu 22.04 minimum, \
                    tous hypervisés et provisionnés par OVH. Ces machines hébergent des services critiques \
                    utilisés par des collectivités territoriales et des administrations publiques : la forge \
                    coopérative GitLab, le Comptoir du Libre, Démarches Simplifiées, et d'autres outils \
                    métiers du secteur public."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "Ma responsabilité couvrait à la fois le maintien en conditions opérationnelles de \
                    cette infrastructure, la mise en place de la supervision, et l'automatisation de \
                    l'administration via Puppet — l'outil d'Infrastructure as Code central de l'association."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC3.01 — Concevoir et déployer des services dans une infrastructure complexe"
                </h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "L'ensemble des services de l'ADULLACT est décrit et déployé via Puppet. J'ai contribué \
                    à la conception et au déploiement de plusieurs services en écrivant des modules Puppet \
                    from scratch, permettant d'instancier de manière reproductible et idempotente des \
                    environnements complets sur les 37 nœuds du parc."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "J'ai notamment conçu le module Puppet du Comptoir du Libre, en modélisant l'ensemble \
                    de sa configuration (services systemd, dépendances, paramètres applicatifs) sous forme \
                    de code versionné. Ce module permet de déployer et de reconfigurer le service de manière \
                    automatique sur n'importe quel nœud cible, sans intervention manuelle."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC3.02 — Sécuriser les services et superviser leur fonctionnement"
                </h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "J'ai mis en place la supervision de l'infrastructure en déployant une stack Prometheus \
                    avec des exporters adaptés aux besoins de l'association. J'ai contribué au module Puppet \
                    gérant Prometheus et écrit des configurations d'exporters sur mesure pour couvrir les \
                    métriques spécifiques aux services hébergés."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "Un tableau de bord Grafana a été déployé pour centraliser la visualisation des métriques \
                    collectées : disponibilité des services, charge des nœuds, latences. Cette supervision \
                    active permet de détecter les anomalies avant qu'elles n'impactent les utilisateurs finaux \
                    — des agents de collectivités territoriales dont les workflows dépendent de ces outils."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC3.03 — Faire évoluer un système en production"
                </h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "J'ai réalisé la mise à jour de demarches.adullact.org en production sans interruption \
                    de service. Cette mise à jour a tiré parti du hot-reload de systemd combiné à Puma \
                    (le serveur d'application Ruby), permettant de recharger l'application à chaud sans \
                    couper les connexions en cours."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "Ce type de déploiement zero-downtime est particulièrement important dans ce contexte : \
                    les services sont utilisés en journée par des agents publics dont les démarches en cours \
                    ne doivent pas être interrompues. La maîtrise du cycle de vie des processus systemd \
                    (socket activation, reload signal vers Puma) a été déterminante pour mener cette \
                    évolution sans impact utilisateur."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC3.04 — Automatiser l'administration (IaC, CI/CD, scripts)"
                </h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Puppet est l'épine dorsale de l'automatisation à l'ADULLACT. J'ai écrit plusieurs \
                    modules from scratch, dont le module du Comptoir du Libre et le module Prometheus avec \
                    ses exporters. Ces modules décrivent de manière déclarative l'état cible des nœuds : \
                    packages installés, services actifs, fichiers de configuration générés, droits \
                    appliqués — le tout de manière idempotente et versionnée dans GitLab."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "Cette approche Infrastructure as Code permet de garantir la cohérence de configuration \
                    sur l'ensemble du parc de 37 serveurs, d'éliminer la dérive de configuration, et de \
                    rendre chaque changement traçable et réversible via l'historique git. Les pipelines \
                    GitLab CI valident les manifests Puppet avant tout merge, réduisant le risque \
                    d'introduire des régressions en production."
                </p>
            </section>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Traces et preuves"</h2>
                <ul class="list-disc list-inside space-y-2 text-theme-secondary">
                    <li>
                        "Module Puppet Comptoir du Libre — "
                        <a href="https://gitlab.adullact.net" class="text-theme-accent hover:underline" target="_blank">
                            "gitlab.adullact.net"
                        </a>
                    </li>
                    <li>"Module Puppet Prometheus + exporters (contribution et écriture from scratch)"</li>
                    <li>"Tableau de bord Grafana — supervision de l'infrastructure ADULLACT"</li>
                    <li>"Déploiement zero-downtime de demarches.adullact.org (hot-reload systemd + Puma)"</li>
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
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "L'alternance à l'ADULLACT m'a plongé dès la première année dans une infrastructure \
                    de production réelle, avec des contraintes de disponibilité fortes. Administrer 37 \
                    serveurs en production n'est pas un exercice académique : une erreur de configuration \
                    propagée via Puppet peut affecter l'ensemble du parc en quelques minutes."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "Cette réalité m'a appris à travailler avec rigueur : tester en environnement de \
                    staging avant toute application en production, documenter chaque module, versionner \
                    chaque changement. J'ai également développé une appétence réelle pour l'IaC — la \
                    satisfaction de décrire un système entier sous forme de code, de le rejouer à \
                    l'identique sur n'importe quel nœud, est intellectuellement gratifiante et \
                    professionnellement précieuse."
                </p>
            </section>

            <div class="flex items-center gap-3">
                <p class="text-lg font-semibold text-theme-success">"Niveau atteint : Niveau 3"</p>
                <span class="text-sm px-3 py-1 rounded-full bg-green-100 text-green-800">"Acquis"</span>
            </div>

        </main>
    }
}
