use leptos::prelude::*;

// Compétence 2 — Optimiser
// Niveau 2 : Sélectionner les algorithmes adéquats pour répondre à un problème donné
//
// Apprentissages Critiques (AC) typiques niveau 2 :
//   AC2.01 — Analyser un problème et choisir une structure de données adaptée
//   AC2.02 — Comparer des solutions algorithmiques selon leur complexité
//   AC2.03 — S'assurer de la sécurité des données et du code
//
// Ressources mobilisées : cours d'algorithmique, R6.Deploi.05

#[component]
pub fn PortfolioOptimiserPage() -> impl IntoView {
    view! {
        <main class="max-w-5xl mx-auto px-6 py-12">

            <div class="mb-8">
                <a href="/portfolio" class="text-sm text-theme-secondary hover:underline mb-4 block">
                    "← Retour au portfolio"
                </a>
                <span class="text-xs font-bold uppercase text-orange-400">"Compétence 2"</span>
                <h1 class="text-4xl font-bold text-theme-primary mt-1 mb-2">"Optimiser"</h1>
                <p class="text-xl text-theme-secondary italic">
                    "Niveau 2 — Sélectionner les algorithmes adéquats pour répondre à un problème donné"
                </p>
            </div>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Mise en contexte"</h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Cette compétence s'est exercée sur deux projets où les choix algorithmiques \
                    avaient des conséquences directes sur les performances : l'implémentation du \
                    pathfinding des entités dans "
                    <a href="https://github.com/ferrumc-rs/ferrumc" class="text-theme-accent hover:underline" target="_blank">
                        "temper"
                    </a>
                    " (fork de FerruMC, serveur Minecraft en Rust) et la gestion de la concurrence \
                    dans l'"
                    <a href="/articles/comptoir" class="text-theme-accent hover:underline">
                        "outil d'audit du Comptoir du Libre"
                    </a>
                    ". Dans les deux cas, la question n'était pas seulement \"quel algo choisir\" \
                    mais \"quelles structures de données rendent cet algo viable en pratique\"."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.01 — Analyser un problème et choisir une structure de données adaptée"
                </h2>

                <p class="text-theme-secondary leading-relaxed mb-4">
                    "Dans temper, j'ai implémenté le pathfinding des entités (mobs, PNJ) via \
                    l'algorithme A*. Le problème : trouver le chemin optimal d'un point A vers \
                    un point B dans le monde Minecraft, représenté comme une grille 3D de blocs."
                </p>

                <p class="text-theme-secondary leading-relaxed mb-4">
                    "Le choix des structures de données est central dans A* :"
                </p>

                <ul class="list-disc list-inside space-y-2 text-theme-secondary mb-4 ml-4">
                    <li>
                        "L'"
                        <strong>"open set"</strong>
                        " (nœuds à explorer) est implémenté avec un "
                        <code class="text-sm bg-theme-card px-1 rounded">"BinaryHeap&lt;Reverse&lt;(u32, BlockPos)&gt;&gt;"</code>
                        " — une file de priorité min-heap. Le wrapper "
                        <code class="text-sm bg-theme-card px-1 rounded">"Reverse"</code>
                        " est nécessaire car le "
                        <code class="text-sm bg-theme-card px-1 rounded">"BinaryHeap"</code>
                        " de Rust est un max-heap par défaut, et A* doit toujours traiter \
                        le nœud de coût minimal en premier."
                    </li>
                    <li>
                        "Le "
                        <strong>"closed set"</strong>
                        " et les coûts "
                        <code class="text-sm bg-theme-card px-1 rounded">"g(n)"</code>
                        " sont stockés dans un "
                        <code class="text-sm bg-theme-card px-1 rounded">"HashMap&lt;BlockPos, u32&gt;"</code>
                        " pour une vérification en O(1) si un nœud a déjà été visité."
                    </li>
                    <li>
                        "La reconstruction du chemin utilise une "
                        <code class="text-sm bg-theme-card px-1 rounded">"HashMap&lt;BlockPos, BlockPos&gt;"</code>
                        " de parents, parcourue à rebours depuis la destination."
                    </li>
                </ul>

                <p class="text-theme-secondary leading-relaxed mb-4">
                    "La fonction de coût est "
                    <code class="text-sm bg-theme-card px-1 rounded">"f(n) = g(n) + h(n)"</code>
                    " où "
                    <code class="text-sm bg-theme-card px-1 rounded">"g(n)"</code>
                    " est le coût réel depuis le départ et "
                    <code class="text-sm bg-theme-card px-1 rounded">"h(n)"</code>
                    " l'heuristique — la distance de Manhattan adaptée en 3D : \
                    |Δx| + |Δy| + |Δz|. Cette heuristique est admissible (elle ne surestime \
                    jamais le coût réel sur une grille sans mouvement diagonal), ce qui garantit \
                    l'optimalité du chemin trouvé."
                </p>

                <p class="text-theme-secondary leading-relaxed">
                    "Dans l'"
                    <a href="/articles/comptoir" class="text-theme-accent hover:underline">
                        "outil d'audit"
                    </a>
                    ", le choix de SQLite comme structure de stockage (plutôt qu'un simple Vec en \
                    mémoire) permet de persister les résultats entre deux exécutions et de les \
                    interroger avec des requêtes filtrées, sans recharger toutes les données."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.02 — Comparer des solutions selon leur complexité"
                </h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Avant de choisir A*, j'ai comparé les algorithmes de pathfinding classiques \
                    applicables au contexte d'un serveur de jeu en temps réel :"
                </p>

                <div class="overflow-x-auto mb-4">
                    <table class="w-full text-sm text-theme-secondary border-collapse">
                        <thead>
                            <tr class="bg-theme-card">
                                <th class="border border-theme-secondary px-3 py-2 text-left">"Algorithme"</th>
                                <th class="border border-theme-secondary px-3 py-2 text-left">"Complexité"</th>
                                <th class="border border-theme-secondary px-3 py-2 text-left">"Optimal ?"</th>
                                <th class="border border-theme-secondary px-3 py-2 text-left">"Adapté ici ?"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td class="border border-theme-secondary px-3 py-2">"BFS"</td>
                                <td class="border border-theme-secondary px-3 py-2">"O(V + E)"</td>
                                <td class="border border-theme-secondary px-3 py-2">"Oui (coûts égaux)"</td>
                                <td class="border border-theme-secondary px-3 py-2">"Non — explore trop de nœuds inutiles"</td>
                            </tr>
                            <tr>
                                <td class="border border-theme-secondary px-3 py-2">"Dijkstra"</td>
                                <td class="border border-theme-secondary px-3 py-2">"O(E log V)"</td>
                                <td class="border border-theme-secondary px-3 py-2">"Oui"</td>
                                <td class="border border-theme-secondary px-3 py-2">"Partiel — sans heuristique, explore dans toutes les directions"</td>
                            </tr>
                            <tr>
                                <td class="border border-theme-secondary px-3 py-2">"Greedy best-first"</td>
                                <td class="border border-theme-secondary px-3 py-2">"O(E log V)"</td>
                                <td class="border border-theme-secondary px-3 py-2">"Non"</td>
                                <td class="border border-theme-secondary px-3 py-2">"Non — rapide mais ne garantit pas le chemin optimal"</td>
                            </tr>
                            <tr class="bg-green-50 dark:bg-green-900/20">
                                <td class="border border-theme-secondary px-3 py-2 font-bold">"A*"</td>
                                <td class="border border-theme-secondary px-3 py-2">"O(E log V)"</td>
                                <td class="border border-theme-secondary px-3 py-2">"Oui (heuristique admissible)"</td>
                                <td class="border border-theme-secondary px-3 py-2 font-bold">"Oui — optimal et guidé vers la cible"</td>
                            </tr>
                        </tbody>
                    </table>
                </div>

                <p class="text-theme-secondary leading-relaxed mb-3">
                    "A* explore significativement moins de nœuds que Dijkstra grâce à l'heuristique \
                    qui oriente la recherche vers la destination. Sur une grille Minecraft (chunks de \
                    16×16×16 blocs), cette différence est critique : un serveur peut avoir des dizaines \
                    d'entités calculant leur chemin simultanément à chaque tick."
                </p>

                <p class="text-theme-secondary leading-relaxed">
                    "Sur l'outil d'audit, j'ai comparé une approche sans contrôle de concurrence \
                    (toutes les requêtes HTTP en parallèle) à une approche avec "
                    <code class="text-sm bg-theme-card px-1 rounded">"tokio::Semaphore"</code>
                    ". Sans limite, des centaines de connexions simultanées saturaient les serveurs \
                    cibles et provoquaient des timeouts en cascade, faussant les résultats. Le \
                    sémaphore borne la concurrence à N requêtes actives, garantissant à la fois \
                    la fiabilité des mesures et le respect des serveurs audités."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.03 — S'assurer de la sécurité des données et du code"
                </h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Rust joue ici un rôle structurant. Dans l'implémentation A* de temper, le \
                    compilateur garantit statiquement l'absence de déréférencement nul, de \
                    débordement de tableau, et d'accès concurrent non synchronisé à la HashMap \
                    — des sources classiques de bugs dans les implémentations de pathfinding en C++."
                </p>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Un point de vigilance spécifique : l'arithmétique sur les coordonnées de blocs \
                    peut déborder si on utilise des types non contrôlés. L'utilisation de types \
                    sémantiques ("
                    <code class="text-sm bg-theme-card px-1 rounded">"BlockPos"</code>
                    " comme newtype) plutôt que des tuples nus évite les confusions d'axes et \
                    rend les opérations invalides non compilables."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "Sur l'outil d'audit, les insertions SQLite utilisent des requêtes paramétrées \
                    pour éviter toute injection, et les entrées de l'API sont validées avant \
                    traitement pour éviter les paniques sur des données malformées."
                </p>
            </section>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Traces et preuves"</h2>
                <ul class="list-disc list-inside space-y-2 text-theme-secondary">
                    <li>
                        "Implémentation A* pathfinding dans temper (fork FerruMC) — "
                        <a href="https://github.com/ferrumc-rs/ferrumc" class="text-theme-accent hover:underline" target="_blank">
                            "github.com/ferrumc-rs/ferrumc"
                        </a>
                    </li>
                    <li>
                        <a href="/articles/comptoir" class="text-theme-accent hover:underline">
                            "Outil d'audit Comptoir du Libre"
                        </a>
                        " — gestion de concurrence avec tokio::Semaphore · "
                        <a href="https://gitlab.adullact.net/echallias/comptoir_tests_urls" class="text-theme-accent hover:underline" target="_blank">
                            "Code source"
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
                    "L'implémentation de A* dans temper m'a forcé à comprendre pourquoi un algorithme \
                    est efficace, pas seulement comment il fonctionne. La différence entre A* et \
                    Dijkstra ne tient qu'à l'ajout d'une heuristique — mais cette heuristique change \
                    radicalement le comportement en pratique sur des graphes larges comme un monde \
                    Minecraft."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "Ce qui m'a le plus appris : le choix de la structure de données (BinaryHeap \
                    avec Reverse) était aussi important que le choix de l'algorithme. Un A* avec \
                    une liste non triée pour l'open set aurait une complexité O(V²) au lieu de \
                    O(E log V) — inutilisable pour un serveur en temps réel. L'algo et la \
                    structure sont indissociables."
                </p>
            </section>

            <div class="flex items-center gap-3">
                <p class="text-lg font-semibold text-theme-success">"Niveau atteint : Niveau 2"</p>
                <span class="text-sm px-3 py-1 rounded-full bg-green-100 text-green-800">"Acquis"</span>
            </div>

        </main>
    }
}
