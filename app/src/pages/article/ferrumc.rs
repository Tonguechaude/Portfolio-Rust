use leptos::prelude::*;

#[component]
pub fn FerrumcArticlePage() -> impl IntoView {
    view! {
        <main class="max-w-5xl mx-auto px-6 py-12 space-y-6 text-theme-primary">
            <h1 class="text-4xl font-bold">"Contributions à FerruMC — Serveur Minecraft en Rust"</h1>

            <p class="text-lg">
                "FerruMC est un serveur Minecraft entièrement réécrit en Rust, un projet open-source ambitieux qui m'a plongé dans l'univers du développement collaboratif international. Une aventure technique qui m'a fait découvrir l'architecture ECS, le moteur Bevy, et les exigences du développement open-source de qualité."
            </p>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Un Serveur Minecraft en Rust, le Défi"</h2>
                <p>
                    "Quand j'ai découvert FerruMC, l'idée m'a immédiatement séduit : réécrire un serveur Minecraft complet en Rust pour offrir une alternative performante et sûre aux serveurs Java traditionnels. Ce n'est pas un simple port, c'est une refonte complète qui repense l'architecture pour tirer parti des forces de Rust : sécurité mémoire garantie, performances natives, et parallélisme sans compromis."
                </p>

                <p>
                    "Le projet utilise Bevy, un moteur de jeu moderne basé sur l'architecture ECS (Entity Component System). C'était ma première vraie confrontation avec ce paradigme, et j'ai dû désapprendre beaucoup de réflexes de programmation orientée objet. Dans l'ECS, on ne pense plus en termes d'objets avec des méthodes, mais en termes de données (les Components) et de systèmes de transformation (les Systems) qui opèrent sur ces données. Les entités ne sont que des identifiants, des handles vers des collections de composants."
                </p>

                <img
                    src="/img/articles/ferrumc/ecs-architecture.png"
                    alt="Architecture ECS de Bevy"
                    class="rounded-lg shadow-md my-6"
                />

                <p>
                    "Cette approche offre des avantages spectaculaires. Les données sont organisées de manière cache-friendly, ce qui booste les performances. Le moteur peut exécuter les systèmes en parallèle automatiquement, sans synchronisation manuelle. Mais surtout, l'architecture encourage la composition plutôt que l'héritage : on ajoute des comportements en attachant des composants, pas en construisant des hiérarchies de classes complexes. C'est élégant, mais ça demande un vrai changement de mentalité."
                </p>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Collaboration Internationale et Standards de Qualité"</h2>
                <p>
                    "Contribuer à FerruMC m'a obligé à sortir de ma zone de confort linguistique. Toute la communication se fait en anglais : issues, pull requests, commentaires de code, discussions architecturales avec les mainteneurs. Au début, expliquer un choix technique complexe en anglais était un défi en soi. Mais cette contrainte s'est rapidement transformée en opportunité d'apprentissage. J'ai appris le vocabulaire technique anglais du développement logiciel, à structurer mes arguments clairement, à anticiper les questions."
                </p>

                <p>
                    "Le processus de review est exigeant, et c'est exactement ce qui rend le projet formateur. Chaque pull request doit être approuvée par au moins un mainteneur, et les reviews ne sont pas de simples validations. Les mainteneurs examinent la lisibilité du code, les choix d'architecture, les performances, la sécurité. Il n'est pas rare qu'une PR passe par trois ou quatre cycles de révision avant d'être mergée. Les premiers retours pouvaient être difficiles à encaisser, mais j'ai vite compris que c'était là que je progressais le plus."
                </p>

                <img
                    src="/img/articles/ferrumc/pr-review-example.png"
                    alt="Exemple de code review sur une pull request"
                    class="rounded-lg shadow-md my-6"
                />

                <p>
                    "Je pense qu'il faut aussi savoir montrer certaines phases plus compliqué pour moi donc ..."
                </p>

                <a href="https://github.com/ferrumc-rs/ferrumc/pull/276" class="text-blue-600 underline" target="_blank"> "Une PR concernant le system de gravité et de collision plus compliqué pour moi ..." </a>

                <p>
                    "Le projet impose des standards stricts via sa CI. Tests unitaires obligatoires pour chaque feature, tests d'intégration pour le protocole réseau, Clippy en mode strict pour détecter les patterns non idiomatiques, rustfmt pour le formatage, cargo-audit pour les vulnérabilités de sécurité, et même des benchmarks pour éviter les régressions de performance. Cette infrastructure force à penser qualité dès le départ, pas après coup."
                </p>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Plonger dans le Protocole Minecraft"</h2>
                <p>
                    "Une des parties les plus fascinantes – et frustrantes – du projet est l'implémentation du protocole réseau de Minecraft. Le protocole est vaste, complexe, et étonnamment peu documenté officiellement. On se retrouve souvent à reverse-engineer le comportement du client en analysant le trafic réseau ou en lisant le code décompilé du jeu. Chaque paquet réseau a sa structure spécifique, ses règles de sérialisation, ses edge cases."
                </p>

                <p>
                    "Travailler sur ce protocole m'a forcé à maîtriser l'async Rust avec tokio. La gestion du réseau est asynchrone par nature : on attend des paquets, on en envoie, on gère des milliers de connexions concurrentes. Tokio offre les primitives pour ça, mais la courbe d'apprentissage est raide. Les concepts de futures, de polling, de pinning, de Send/Sync bounds deviennent soudain très concrets quand on debug un deadlock ou une panique dans un context async."
                </p>

                <p>
                    "J'ai contribué à l'implémentation de plusieurs features du protocole, corrigé des bugs de synchronisation réseau, optimisé des hot paths identifiés par profiling. Chaque contribution nécessitait des tests robustes : on ne peut pas se permettre de casser la compatibilité avec le client officiel. Les tests d'intégration simulent des scénarios de jeu complets, vérifient que les paquets sont bien formés, que les états sont cohérents."
                </p>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Documentation et Transmission de Savoir"</h2>
                <p>
                    "Au-delà du code, j'ai contribué à la documentation du projet. Écrire de la doc technique en anglais est un exercice d'équilibre : être précis sans être verbeux, être complet sans noyer le lecteur sous les détails. J'ai rédigé des guides d'architecture, documenté des modules avec rustdoc, expliqué des choix de design non évidents. Cette pratique m'a rendu meilleur communicateur : si je n'arrive pas à expliquer clairement pourquoi j'ai fait tel choix, c'est peut-être que le choix n'était pas si bon."
                </p>

                <img
                    src="/img/articles/ferrumc/server-running.png"
                    alt="Serveur FerruMC en fonctionnement"
                    class="rounded-lg shadow-md my-6"
                />

                <p>
                    "J'ai aussi participé aux code reviews des pull requests d'autres contributeurs. Reviewer le code des autres est une responsabilité : on cherche les bugs potentiels, on suggère des améliorations, on vérifie la conformité aux guidelines du projet. Mais c'est aussi un formidable moyen d'apprendre en voyant comment d'autres développeurs abordent les mêmes problèmes, quelles bibliothèques ils utilisent, quels patterns ils appliquent."
                </p>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Les Leçons de l'Open Source"</h2>
                <p>
                    "FerruMC m'a appris bien plus que du Rust et du Bevy. C'est une école de développement professionnel. J'ai développé l'humilité face aux reviews : accepter que mon code peut être amélioré, que d'autres peuvent avoir de meilleures idées. J'ai appris la persévérance : itérer sur une PR jusqu'à ce qu'elle atteigne les standards, même si ça prend du temps. J'ai appris à communiquer efficacement dans un contexte international, à justifier mes choix techniques, à comprendre les contraintes d'un projet de cette envergure."
                </p>

                <p>
                    "Les défis techniques étaient nombreux : comprendre l'ECS alors que j'étais habitué à la POO, maîtriser l'async Rust avec ses subtilités, décoder un protocole réseau obscur, optimiser des chemins critiques sans casser la compatibilité. Mais les défis humains étaient tout aussi importants : communiquer en anglais, accepter les critiques, collaborer avec des gens que je n'ai jamais rencontrés, dans des fuseaux horaires différents."
                </p>

                <p>
                    "Cette expérience m'a donné un autre aperçu concret de ce qu'est le développement logiciel dans un contexte professionnel. Les standards de qualité, l'importance des tests, la nécessité d'une documentation claire, le poids des reviews, l'itération continue. C'est exactement le genre d'expérience qu'on ne peut pas acquérir en travaillant seul sur des projets personnels. FerruMC m'a préparé à intégrer une équipe technique exigeante et à contribuer efficacement à des projets de qualité."
                </p>
            </section>
        </main>
    }
}
