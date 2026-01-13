use leptos::prelude::*;

#[component]
pub fn PortfolioRustArticlePage() -> impl IntoView {
    view! {
        <main class="max-w-5xl mx-auto px-6 py-12 space-y-6 text-theme-primary">
            <h1 class="text-4xl font-bold">"Portfolio en Rust avec Leptos — De la Conception au Déploiement"</h1>

            <p class="text-lg">
                "Création d'un portfolio personnel moderne et performant en utilisant Rust et WebAssembly. Un projet qui m'a permis d'explorer les technologies web de nouvelle génération tout en maîtrisant l'ensemble de la chaîne DevOps."
            </p>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Le pari du Rust pour le Web"</h2>
                <p>
                    "Quand j'ai décidé de refaire mon portfolio, j'aurais pu opter pour les classiques du web moderne : React, Vue, ou Svelte. Mais j'ai choisi une approche plus audacieuse en utilisant Leptos, un framework web Rust qui compile en WebAssembly. Ce choix n'était pas qu'une question de curiosité technique, c'était l'occasion de repousser les limites de ce qu'on peut faire avec Rust en dehors du backend et du système."
                </p>

                <p>
                    "Leptos offre quelque chose de fondamentalement différent des frameworks JavaScript. Grâce à WebAssembly, le code tourne à des vitesses proches du natif dans le navigateur. Le système de types de Rust détecte les erreurs à la compilation plutôt qu'au runtime, et le bundle final est significativement plus léger qu'une application JavaScript traditionnelle. Mais le vrai atout, c'est son système de signaux : la réactivité est fine-grained, ce qui signifie que seules les parties du DOM qui doivent changer sont mises à jour, sans calculs superflus."
                </p>

                <img
                    src="/img/articles/portfolio-rust/performance-metrics.png"
                    alt="Métriques de performance du portfolio"
                    class="rounded-lg shadow-md my-6"
                />

                <p>
                    "J'ai opté pour un site statique généré (SSG) plutôt qu'une application server-side rendered. Ce choix architectural simplifie l'hébergement tout en gardant d'excellentes performances et un bon SEO grâce au HTML pré-rendu. Pas besoin de serveur Node en production, juste un serveur web classique comme Nginx pour servir les fichiers statiques."
                </p>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Une Pipeline CI/CD Maison"</h2>
                <p>
                    "L'un des aspects les plus enrichissants de ce projet a été la mise en place d'une infrastructure DevOps complète. J'ai configuré un GitLab Runner sur mon VPS personnel qui orchestre toute la chaîne de déploiement. À chaque push sur la branche principale, la pipeline se déclenche automatiquement : compilation du projet avec cargo-leptos, exécution des tests, optimisation du bundle WASM, et enfin déploiement sur le serveur Nginx via SSH."
                </p>
                <img
                    src="/img/articles/portfolio-rust/cicd-pipeline.png"
                    alt="Architecture de la pipeline CI/CD"
                    class="rounded-lg shadow-md my-6"
                />

                <p>
                    "Le Runner utilise Docker Rootless pour isoler chaque build, ce qui garantit la reproductibilité et évite les conflits de dépendances. J'ai aussi implémenté un système de cache pour les artifacts Rust, ce qui a divisé par trois le temps de compilation. Les builds qui prenaient initialement 8 minutes tournent maintenant en moins de 3 minutes grâce à la compilation incrémentale et au cache des crates."
                </p>

                <p>
                    "Configurer ce système m'a forcé à plonger dans l'administration système Linux. J'ai dû gérer les permissions Docker, configurer les clés SSH pour le déploiement automatisé, et mettre en place des webhooks GitLab. Chaque problème rencontré était une opportunité d'apprentissage : quand Nginx refusait de servir les fichiers WASM, j'ai découvert l'importance des MIME types. Quand les builds échouaient mystérieusement, j'ai appris à débugger les problèmes de permissions dans les conteneurs."
                </p>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Hébergement et Infrastructure"</h2>
                <p>
                    "Le site tourne sur un VPS que je loue et administre moi-même. Ce choix me donne un contrôle total sur l'infrastructure, contrairement aux solutions d'hébergement managées. Nginx sert les fichiers statiques avec une configuration optimisée : compression gzip activée pour réduire la bande passante, headers de cache configurés pour accélérer les visites suivantes, et HTTPS via Let's Encrypt pour la sécurité. J'ai aussi activé HTTP/2, ce qui permet de multiplexer les requêtes et d'améliorer significativement les temps de chargement."
                </p>

                <p>
                    "Un des défis techniques a été de configurer correctement Nginx pour servir les fichiers WebAssembly. Par défaut, Nginx ne reconnaît pas le MIME type application/wasm, ce qui causait des erreurs de chargement dans le navigateur. J'ai dû ajouter manuellement la directive dans la configuration Nginx. De même, la taille initiale du bundle WASM était problématique – environ 2 Mo non compressé. Après optimisation avec wasm-opt et activation de la compression gzip, j'ai réduit le poids à moins de 400 Ko, ce qui rend le site utilisable même sur des connexions modestes."
                </p>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Fonctionnalités et Expérience Utilisateur"</h2>
                <p>
                    "Le portfolio intègre plusieurs fonctionnalités modernes. Le routing est géré côté client avec Leptos Router, ce qui offre une navigation fluide sans rechargement de page. J'ai implémenté un système de dark mode avec persistance dans le localStorage, une fonctionnalité devenue indispensable pour le confort visuel. La section projets présente des cards interactives avec des gradients dynamiques générés programmatiquement, et j'ai créé une visualisation en graphe des technologies que je maîtrise, inspirée des graphes de connaissances."
                </p>

                <img
                    src="/img/articles/portfolio-rust/tech-graph.png"
                    alt="Graphe interactif des technologies"
                    class="rounded-lg shadow-md my-6"
                />

                <p>
                    "Le design est entièrement responsive, construit avec TailwindCSS. Cette approche utility-first m'a permis de prototyper rapidement tout en gardant un CSS maintenable. Chaque composant s'adapte fluidement du mobile au desktop, avec des breakpoints soigneusement choisis pour offrir la meilleure expérience sur chaque appareil."
                </p>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Ce que j'en retire"</h2>
                <p>
                    "Ce projet va bien au-delà d'un simple site vitrine. C'est une plateforme d'expérimentation qui m'a permis de maîtriser l'ensemble du stack, du code Rust compilé en WebAssembly jusqu'à l'infrastructure de déploiement sur un VPS Linux. J'ai acquis une compréhension profonde de la compilation WASM, des subtilités du système de lifetimes Rust, et des patterns de réactivité fine-grained."
                </p>

                <p>
                    "Côté DevOps, j'ai construit une pipeline CI/CD complète qui automatise chaque étape du déploiement. Cette expérience m'a donné confiance dans ma capacité à gérer une infrastructure en production, à diagnostiquer des problèmes en environnement réel, et à optimiser les performances d'une application web moderne. Chaque optimisation, chaque bug résolu, chaque configuration Nginx peaufinée a renforcé ma compréhension de l'écosystème web."
                </p>

                <p>
                    "Plus largement, ce projet valide pour moi l'idée que Rust et WebAssembly représentent une direction prometteuse pour le développement web. Les performances natives, la sécurité du typage, et l'expérience développeur améliorée en font des technologies d'avenir. Ce portfolio n'est pas qu'une vitrine de mes compétences : c'est la preuve concrète que cette approche est viable et performante."
                </p>
            </section>
        </main>
    }
}
