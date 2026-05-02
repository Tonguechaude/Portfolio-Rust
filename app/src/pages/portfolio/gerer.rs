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
                    "Niveau 2 — Optimiser une base de données, interagir avec une application et mettre en œuvre la sécurité"
                </p>
            </div>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Mise en contexte"</h2>
                <p class="text-theme-secondary leading-relaxed">
                    "Cette compétence s'illustre principalement à travers l'"
                    <a href="/articles/comptoir" class="text-theme-accent hover:underline">
                        "outil d'audit du Comptoir du Libre"
                    </a>
                    ", développé durant mon alternance à l'ADULLACT. L'outil consomme l'API \
                    publique du Comptoir du Libre (catalogue de logiciels libres métiers), vérifie \
                    la disponibilité de leurs ressources en ligne, et stocke les résultats dans \
                    une base SQLite pour analyse. Ce projet concentre les quatre apprentissages \
                    critiques de la compétence : modélisation des données, confidentialité, \
                    restitution et manipulation de formats hétérogènes."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.01 — Optimiser les modèles de données"
                </h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Le modèle de données a été pensé pour répondre au besoin concret : stocker \
                    efficacement les résultats d'audit (URL testée, statut HTTP retourné, \
                    horodatage, message d'erreur éventuel) pour chaque logiciel du catalogue. \
                    J'ai utilisé "
                    <code class="text-sm bg-theme-card px-1 rounded">"rusqlite"</code>
                    " pour interagir avec SQLite depuis Rust."
                </p>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Le choix de SQLite plutôt qu'un simple fichier CSV en mémoire était motivé \
                    par deux raisons : la persistance entre deux exécutions (l'audit peut être \
                    relancé sans tout re-télécharger) et la possibilité de filtrer les résultats \
                    par statut HTTP ou par logiciel via des requêtes SQL, sans recharger \
                    l'intégralité des données."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "Les insertions sont effectuées en batch dans une transaction unique plutôt \
                    qu'une insertion par ligne, ce qui réduit drastiquement le nombre d'accès \
                    disque et améliore les performances sur des milliers d'entrées."
                </p>
                <img
                    alt="Structure base de données SQLite"
                    src="/img/apprentissages/comptoir/rusqlite.png"
                    class="rounded-lg border shadow-lg mt-4"
                />
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.02 — Assurer la confidentialité des données"
                </h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "L'outil traite uniquement des données publiques (URLs et statuts de \
                    disponibilité de logiciels libres), ce qui limite les enjeux RGPD. \
                    Néanmoins, les bonnes pratiques ont été appliquées : les requêtes SQLite \
                    sont systématiquement paramétrées pour prévenir toute injection, et les \
                    entrées issues de l'API externe sont validées avant insertion pour éviter \
                    la corruption de la base."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "La sensibilisation aux enjeux de confidentialité s'est aussi faite dans \
                    le contexte plus large de l'ADULLACT, où les infrastructures hébergées \
                    manipulent des données de collectivités territoriales soumises au RGPD. \
                    Travailler dans cet environnement m'a ancré dans une culture de \
                    protection des données par défaut."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.03 — Organiser la restitution des données"
                </h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Les résultats de l'audit sont restitués sous deux formes complémentaires : \
                    un affichage console en temps réel pendant l'exécution (statut de chaque URL \
                    testée, erreurs rencontrées), et un export CSV en fin d'exécution pour \
                    permettre une analyse dans LibreOffice Calc ou tout autre outil tableur."
                </p>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Ce double format de restitution répondait au besoin du client (le développeur \
                    principal du Comptoir) : une vue synthétique exploitable sans compétences \
                    techniques, exportable et partageable avec l'équipe."
                </p>
                <img
                    alt="Résultats de l'audit"
                    src="/img/apprentissages/comptoir/resultat.png"
                    class="rounded-lg border shadow-lg mt-4"
                />
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.04 — Manipuler des données hétérogènes"
                </h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "L'outil ingère des données dans trois formats distincts : un fichier JSON \
                    téléchargé depuis l'API du Comptoir (désérialisé avec "
                    <code class="text-sm bg-theme-card px-1 rounded">"serde_json"</code>
                    "), des réponses HTTP asynchrones avec leurs codes de statut et headers \
                    (via "
                    <code class="text-sm bg-theme-card px-1 rounded">"reqwest"</code>
                    "), et des enregistrements SQLite en sortie exportés en CSV."
                </p>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "L'hétérogénéité des données de l'API était aussi une contrainte en soi : \
                    certains logiciels n'avaient pas d'URL de dépôt, d'autres avaient des URLs \
                    malformées, des dates dans des formats différents, des champs optionnels \
                    absents. Le parsing avec "
                    <code class="text-sm bg-theme-card px-1 rounded">"serde"</code>
                    " et ses attributs "
                    <code class="text-sm bg-theme-card px-1 rounded">"#[serde(default)]"</code>
                    " / "
                    <code class="text-sm bg-theme-card px-1 rounded">"Option&lt;T&gt;"</code>
                    " a permis de gérer proprement ces cas sans paniquer sur des données \
                    manquantes."
                </p>
                <img
                    alt="Gestion des données API"
                    src="/img/apprentissages/comptoir/api_management.png"
                    class="rounded-lg border shadow-lg mt-4"
                />
            </section>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Traces et preuves"</h2>
                <ul class="list-disc list-inside space-y-2 text-theme-secondary">
                    <li>
                        <a href="/articles/comptoir" class="text-theme-accent hover:underline">
                            "Article — Audit du Comptoir du Libre"
                        </a>
                    </li>
                    <li>
                        "Code source — "
                        <a href="https://gitlab.adullact.net/echallias/comptoir_tests_urls" class="text-theme-accent hover:underline" target="_blank">
                            "gitlab.adullact.net/echallias/comptoir_tests_urls"
                        </a>
                    </li>
                    <li>
                        <a href="/apprentissages/comptoir" class="text-theme-accent hover:underline">
                            "Page apprentissage — Audit automatisé Comptoir du Libre"
                        </a>
                    </li>
                </ul>
            </section>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Ressources mobilisées"</h2>
                <ul class="list-disc list-inside space-y-1 text-theme-secondary">
                    <li>"R6.Deploi.05 — Optimisation des services complexes"</li>
                </ul>
            </section>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Analyse réflexive"</h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Ce projet m'a appris que la gestion des données commence avant la base de \
                    données : la qualité du modèle dépend de la qualité de l'analyse du besoin. \
                    Choisir SQLite plutôt qu'un simple fichier texte, structurer les insertions \
                    en transactions, prévoir l'export CSV dès la conception — ces décisions \
                    prises tôt ont évité des refactorisations coûteuses."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "La manipulation de données hétérogènes issues d'une API externe m'a aussi \
                    sensibilisé à la robustesse : une application qui panique sur un champ \
                    manquant dans un JSON n'est pas une application fiable. Rust et serde \
                    m'ont poussé à traiter explicitement chaque cas d'absence de donnée, \
                    ce qui a rendu l'outil beaucoup plus solide face aux évolutions \
                    de l'API du Comptoir."
                </p>
            </section>

            <div class="flex items-center gap-3">
                <p class="text-lg font-semibold text-theme-success">"Niveau atteint : Niveau 2"</p>
                <span class="text-sm px-3 py-1 rounded-full bg-green-100 text-green-800">"Acquis"</span>
            </div>

        </main>
    }
}
