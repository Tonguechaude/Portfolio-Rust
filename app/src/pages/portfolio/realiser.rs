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
// Traces : ce portfolio (Leptos/WASM), outil d'audit Comptoir du Libre, FerruMC

#[component]
pub fn PortfolioRealiserPage() -> impl IntoView {
    view! {
        <main class="max-w-5xl mx-auto px-6 py-12">

            <div class="mb-8">
                <a href="/portfolio" class="text-sm text-theme-secondary hover:underline mb-4 block">
                    "← Retour au portfolio"
                </a>
                <span class="text-xs font-bold uppercase text-orange-500">"Compétence 1"</span>
                <h1 class="text-4xl font-bold text-theme-primary mt-1 mb-2">"Réaliser"</h1>
                <p class="text-xl text-theme-secondary italic">
                    "Niveau 3 — Adapter des applications sur un ensemble de supports"
                </p>
            </div>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Mise en contexte"</h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Cette compétence s'illustre à travers trois projets conduits en Rust, sur des \
                    supports et des contraintes radicalement différents : un outil d'analyse réseau \
                    en ligne de commande ("
                    <a href="/articles/comptoir" class="text-theme-accent hover:underline">
                        "audit du Comptoir du Libre"
                    </a>
                    "), une application web full-stack compilée en WebAssembly ("
                    <a href="/articles/portfolio_rust" class="text-theme-accent hover:underline">
                        "ce portfolio"
                    </a>
                    "), et des contributions à un serveur de jeu open-source implémentant \
                    un protocole réseau complexe ("
                    <a href="/articles/ferrumc" class="text-theme-accent hover:underline">
                        "FerruMC"
                    </a>
                    "). Ces trois projets partagent le même langage mais imposent des architectures, \
                    des frameworks et des contraintes de déploiement complètement différents."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC3.01 — Concevoir et développer des applications communicantes"
                </h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "L'"
                    <a href="/articles/comptoir" class="text-theme-accent hover:underline">
                        "outil d'audit du Comptoir du Libre"
                    </a>
                    " est une application Rust entièrement orientée réseau : elle consomme \
                    l'API publique du Comptoir, envoie des centaines de requêtes HTTP concurrentes \
                    via "
                    <code class="text-sm bg-theme-card px-1 rounded">"reqwest"</code>
                    " et "
                    <code class="text-sm bg-theme-card px-1 rounded">"tokio"</code>
                    ", contrôle la concurrence avec un "
                    <code class="text-sm bg-theme-card px-1 rounded">"tokio::Semaphore"</code>
                    " pour ne pas saturer les serveurs cibles, et stocke les résultats dans SQLite \
                    pour une analyse différée. La conception a nécessité de modéliser les données \
                    JSON de l'API, de gérer finement les cas d'erreur (timeout, connexion refusée, \
                    statut HTTP inattendu) et d'exporter les résultats en CSV."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "Sur "
                    <a href="/articles/ferrumc" class="text-theme-accent hover:underline">
                        "FerruMC"
                    </a>
                    ", la communication est au cœur du projet : implémenter le protocole réseau \
                    binaire de Minecraft, sérialiser et désérialiser des paquets, gérer des \
                    milliers de connexions TCP concurrentes avec tokio. J'ai contribué à \
                    l'implémentation de features du protocole et à la correction de bugs de \
                    synchronisation réseau, en veillant à la compatibilité stricte avec le \
                    client officiel."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC3.02 — Utiliser des frameworks adaptés au support cible"
                </h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Chaque projet a imposé un choix de framework dicté par le support cible. \
                    Pour le web, j'ai utilisé "
                    <a href="/articles/portfolio_rust" class="text-theme-accent hover:underline">
                        "Leptos"
                    </a>
                    " — un framework Rust compilant en WebAssembly — qui apporte la réactivité \
                    fine-grained, le SSR/SSG, et le routage côté client. Ce choix non conventionnel \
                    (Rust plutôt que JavaScript pour le frontend) m'a confronté aux spécificités \
                    du target WASM : pas de threads standard, pas d'accès direct au système de \
                    fichiers, bundle size à optimiser."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "Pour FerruMC, le framework est Bevy, un moteur basé sur l'architecture ECS \
                    (Entity Component System). Ce paradigme est radicalement différent de la POO : \
                    on raisonne en termes de composants de données et de systèmes de transformation, \
                    pas d'objets avec méthodes. L'ECS permet une exécution parallèle automatique \
                    des systèmes et une organisation cache-friendly des données — deux impératifs \
                    pour un serveur de jeu performant."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC3.03 — Adapter l'application aux contraintes spécifiques"
                </h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Le "
                    <a href="/articles/portfolio_rust" class="text-theme-accent hover:underline">
                        "portfolio"
                    </a>
                    " a imposé des contraintes propres au web et à WebAssembly : le bundle WASM \
                    initial de ~2 Mo était trop lourd — après optimisation avec "
                    <code class="text-sm bg-theme-card px-1 rounded">"wasm-opt"</code>
                    " et compression gzip, il a été réduit à moins de 400 Ko. Nginx devait être \
                    configuré avec le MIME type "
                    <code class="text-sm bg-theme-card px-1 rounded">"application/wasm"</code>
                    " sans quoi le navigateur refusait de charger le module. Ces détails, \
                    invisibles en développement, bloquent en production."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "Pour FerruMC, la contrainte principale est la compatibilité stricte avec le \
                    client Minecraft officiel : le protocole réseau n'est pas documenté \
                    officiellement, ce qui a nécessité du reverse-engineering par analyse de \
                    trafic réseau et lecture de code décompilé. Chaque paquet a une structure \
                    précise, des règles de sérialisation et des edge cases à couvrir — une \
                    erreur de quelques bits et le client déconnecte."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC3.04 — Assurer la qualité et la maintenabilité du code"
                </h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    <a href="/articles/ferrumc" class="text-theme-accent hover:underline">
                        "FerruMC"
                    </a>
                    " est le projet où j'ai eu les exigences de qualité les plus élevées. \
                    La CI du projet imposait : tests unitaires pour chaque feature, tests \
                    d'intégration simulant des sessions de jeu complètes, Clippy en mode strict \
                    pour les patterns non idiomatiques, rustfmt pour le formatage, cargo-audit \
                    pour les vulnérabilités, et des benchmarks pour détecter les régressions \
                    de performance. Chaque pull request passait par plusieurs cycles de review \
                    avec les mainteneurs avant d'être mergée — parfois trois ou quatre \
                    itérations sur le même changement."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "Sur le "
                    <a href="/articles/portfolio_rust" class="text-theme-accent hover:underline">
                        "portfolio"
                    </a>
                    ", j'ai mis en place une pipeline CI/CD GitLab avec un Runner Docker Rootless \
                    sur mon VPS, qui compile, teste et déploie automatiquement à chaque push. \
                    Un système de cache des artifacts Rust a divisé le temps de build par trois \
                    (de 8 à moins de 3 minutes). J'ai aussi rédigé de la documentation \
                    pour les modules Puppet de l'ADULLACT, afin que l'équipe puisse les \
                    maintenir indépendamment."
                </p>
            </section>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Traces et preuves"</h2>
                <ul class="list-disc list-inside space-y-2 text-theme-secondary">
                    <li>
                        <a href="/articles/portfolio_rust" class="text-theme-accent hover:underline">
                            "Portfolio Rust — Leptos + WASM + CI/CD GitLab"
                        </a>
                    </li>
                    <li>
                        <a href="/articles/comptoir" class="text-theme-accent hover:underline">
                            "Outil d'audit Comptoir du Libre — Rust async, reqwest, SQLite"
                        </a>
                        " · "
                        <a href="https://gitlab.adullact.net/echallias/comptoir_tests_urls" class="text-theme-accent hover:underline" target="_blank">
                            "Code source"
                        </a>
                    </li>
                    <li>
                        <a href="/articles/ferrumc" class="text-theme-accent hover:underline">
                            "Contributions FerruMC — Rust, Bevy ECS, protocole Minecraft"
                        </a>
                        " · "
                        <a href="https://github.com/ferrumc-rs/ferrumc" class="text-theme-accent hover:underline" target="_blank">
                            "GitHub"
                        </a>
                    </li>
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
                    "Ces trois projets m'ont appris que réaliser une application, c'est d'abord \
                    comprendre les contraintes de son support avant d'écrire une ligne de code. \
                    Leptos sans comprendre les limites de WASM, Bevy sans comprendre l'ECS, \
                    tokio sans comprendre le modèle async de Rust : dans chaque cas, aller trop \
                    vite sur le code sans maîtriser le paradigme coûte cher en débogage."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "FerruMC a été l'expérience la plus formatrice sur la qualité : voir mon code \
                    renvoyé en review avec des commentaires précis sur la lisibilité, les \
                    performances ou l'idiomatic Rust m'a plus appris qu'une année de projets \
                    académiques. Accepter que son code puisse être amélioré — et itérer sans \
                    ego — est une compétence en soi."
                </p>
            </section>

            <div class="flex items-center gap-3">
                <p class="text-lg font-semibold text-theme-success">"Niveau atteint : Niveau 3"</p>
                <span class="text-sm px-3 py-1 rounded-full bg-green-100 text-green-800">"Acquis"</span>
            </div>

        </main>
    }
}
