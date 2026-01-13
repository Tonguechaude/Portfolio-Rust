use leptos::prelude::*;

#[component]
pub fn RusticArticlePage() -> impl IntoView {
    view! {
        <main class="max-w-5xl mx-auto px-6 py-12 space-y-6 text-theme-primary">
            <h1 class="text-4xl font-bold">"Rustic — Logiciel de Sauvegarde en Rust"</h1>

            <p class="text-lg">
                "Projet académique ambitieux : créer en équipe un logiciel de sauvegarde complet inspiré de Restic, entièrement en Rust. Trois mois pour concevoir, implémenter, tester et présenter un système de backup chiffré avec déduplication. Une plongée technique dans la cryptographie, les systèmes de fichiers, et le développement collaboratif."
            </p>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Décortiquer un Système de Backup Moderne"</h2>
                <p>
                    "Pour construire Rustic, nous avons commencé par étudier Restic, un logiciel de backup open-source réputé pour sa fiabilité et sa sécurité. Nous avons décortiqué son architecture pour comprendre les principes fondamentaux : déduplication content-addressed pour éviter de stocker plusieurs fois les mêmes données, snapshots pour capturer l'état du système à un instant T, chiffrement AES-256 pour sécuriser les archives, compression pour économiser l'espace disque, et vérification d'intégrité via checksums pour détecter la corruption."
                </p>

                <p>
                    "La décision de réimplémenter ces concepts en Rust n'était pas arbitraire. Rust nous garantissait la sécurité mémoire sans garbage collector, des performances natives comparables au C, et un système de types strict qui détecte les erreurs dès la compilation. Pour un système de backup qui manipule des données sensibles, ces garanties sont cruciales : un bug de segmentation ou une race condition pourrait corrompre des sauvegardes et causer des pertes de données irréversibles."
                </p>

                <img
                    src="/img/articles/rustic/cli.png"
                    alt="Architecture du système Rustic"
                    class="rounded-lg shadow-md my-6"
                />

                <p>
                    "Chaque sous-commande a été pensée pour être intuitive, avec des messages d'erreur clairs et des barres de progression pour les opérations longues."
                </p>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Déduplication et Cryptographie, les Fondations Techniques"</h2>
                <p>
                    "Le cœur technique de Rustic repose sur deux piliers : la déduplication content-addressed et le chiffrement end-to-end. Pour la déduplication, nous avons implémenté un algorithme de Content-Defined Chunking. Plutôt que de découper les fichiers en blocs de taille fixe, on utilise un rolling hash pour identifier des frontières de chunks qui dépendent du contenu. Cela permet de détecter efficacement les données identiques même si elles sont décalées dans le fichier."
                </p>

                <p>
                    "Chaque chunk est identifié par son hash SHA-256, qui sert d'adresse pour le stocker. Si un chunk avec le même hash existe déjà dans le repository, on ne le stocke pas à nouveau, on réutilise simplement sa référence. Cette approche réduit drastiquement l'espace de stockage nécessaire, surtout pour des backups incrémentaux où la majorité des données n'a pas changé. L'implémentation de cet algorithme a été l'un des défis techniques les plus intéressants du projet : il fallait trouver le bon équilibre entre taille des chunks (trop petits = trop de métadonnées, trop gros = déduplication inefficace) et performance."
                </p>

                <p>
                    "Côté sécurité, toutes les données sont chiffrées avec AES-256-GCM avant d'être écrites sur disque. La clé de chiffrement est dérivée d'un mot de passe via Argon2, un algorithme de dérivation de clé résistant aux attaques par force brute. Nous avons passé beaucoup de temps à étudier les bonnes pratiques cryptographiques : comment gérer les nonces pour éviter leur réutilisation, comment s'assurer que les métadonnées ne leakent pas d'informations, comment implémenter un mode append-only pour se protéger contre les ransomwares qui tenteraient de modifier ou supprimer les backups existants."
                </p>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Développement en Équipe et Standards Professionnels"</h2>
                <p>
                    "Travailler à quatre sur un projet de cette envergure nécessitait de l'organisation. Nous avons réparti les responsabilités selon les compétences et intérêts de chacun : architecture générale et design du système, interface CLI et UX, implémentation du stockage et de la déduplication, cryptographie et sécurité. Mais plutôt que de travailler en silos, nous avons mis en place un workflow GitLab rigoureux avec des merge requests systématiques et des code reviews croisées."
                </p>

                <img
                    src="/img/articles/rustic/gitlab-pipeline.png"
                    alt="Pipeline CI/CD GitLab de Rustic"
                    class="rounded-lg shadow-md my-6"
                />

                <p>
                    "J'ai pris en charge la configuration de la pipeline CI/CD. Chaque commit déclenche une série de checks automatiques : build sur plusieurs plateformes, tests unitaires et d'intégration, linting avec Clippy en mode strict, vérification du formatage avec rustfmt, benchmarks de performance pour éviter les régressions, et génération automatique de la documentation. Aucune PR ne pouvait être mergée tant que tous ces checks ne passaient pas au vert. Cette discipline nous a évité de nombreux bugs et a maintenu la qualité du code à un niveau élevé."
                </p>

                <p>
                    "Les reviews étaient l'occasion d'apprendre les uns des autres. Quand quelqu'un proposait une optimisation ou suggérait un pattern Rust plus idiomatique, on en discutait, on justifiait nos choix. Ce processus nous a tous fait progresser en Rust et en architecture logicielle. On a aussi découvert qu'écrire du code lisible et maintenable est au moins aussi important qu'écrire du code qui fonctionne : un algorithme optimisé mais incompréhensible est une dette technique."
                </p>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Tests, Documentation, et Préparation de la Soutenance"</h2>
                <p>
                    "Nous avons mis un point d'honneur à tester rigoureusement chaque composant. Tests unitaires pour les modules critiques comme le chunking et le chiffrement, tests d'intégration pour les scénarios end-to-end (backup complet puis restore, vérification d'intégrité, gestion des corruptions), property-based testing avec proptest pour détecter les edge cases qu'on n'aurait pas imaginés. Nous avons maintenu une couverture de code supérieure à 80%, avec un focus particulier sur les parties sensibles où un bug pourrait avoir des conséquences graves."
                </p>

                <p>
                    "La documentation a été un livrable aussi important que le code lui-même. Nous avons rédigé un README complet avec installation et quick start, un guide utilisateur avec tutoriels et troubleshooting, une documentation d'architecture avec diagrammes UML et explications des choix techniques, et un rustdoc exhaustif pour l'API. Nous avons aussi produit un rapport de projet académique détaillant notre démarche, nos recherches, nos difficultés rencontrées et nos solutions. Rédiger tout cela en français et en anglais nous a forcés à clarifier notre compréhension du système."
                </p>

                <p>
                    "La soutenance finale était le point culminant du projet. Nous avions 30 minutes pour présenter Rustic à un jury simulant un client potentiel, suivies de 15 minutes de questions techniques. Nous avons préparé une démo live où nous montrions l'initialisation d'un repository, la création d'un premier snapshot, la modification de fichiers et le backup incrémental démontrant la déduplication, la restauration d'un snapshot spécifique, et la vérification d'intégrité. Tout fonctionnait parfaitement, et le jury a été impressionné par la qualité technique et la maturité du projet."
                </p>

                <img
                    src="/img/articles/rustic/cli-demo.png"
                    alt="Démonstration de Rustic en ligne de commande"
                    class="rounded-lg shadow-md my-6"
                />

                <p>
                    "Les questions du jury ont testé notre compréhension profonde du système. Pourquoi Rust plutôt que Go comme Restic ? Comment gérer la corruption de données au niveau hardware ? Comment scale avec des téraoctets de données ? Quelles attaques cryptographiques sont possibles ? Comment se compare-t-on aux solutions commerciales comme Veeam ou Acronis ? Nous avons pu répondre avec assurance parce que nous avions vraiment construit ce système de A à Z, nous comprenions chaque décision technique."
                </p>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Un Projet Formateur aux Multiples Facettes"</h2>
                <p>
                    "Rustic a été bien plus qu'un exercice académique. C'était une simulation réaliste d'un projet professionnel avec toutes ses contraintes : deadlines serrées, coordination d'équipe, standards de qualité élevés, documentation exhaustive, présentation client. Les compétences techniques acquises sont directement transférables : manipulation des systèmes de fichiers, cryptographie appliquée, algorithmes de chunking et déduplication, design de CLI ergonomique, testing rigoureux."
                </p>

                <p>
                    "Mais les compétences méthodologiques sont tout aussi précieuses. J'ai appris à travailler en équipe agile avec sprints et stand-ups, à gérer un workflow GitLab professionnel avec branches, MR et reviews, à mettre en place une CI/CD robuste, à documenter techniquement en bilingue, à planifier et répartir les tâches efficacement. Les compétences en communication développées pendant ce projet – présentation orale, gestion d'une démo technique live, réponses aux questions d'un jury exigeant, vulgarisation de concepts complexes – sont indispensables dans le monde professionnel."
                </p>

                <p>
                    "Les défis rencontrés ont été formateurs. Implémenter le content-defined chunking efficacement a demandé des recherches approfondies et beaucoup d'expérimentations. Le système de gestion d'erreurs de Rust, bien que verbeux, nous a forcés à anticiper tous les cas d'échec et a produit un code robuste. Optimiser les performances tout en maintenant la déduplication active était un exercice d'équilibre délicat. Synchroniser quatre emplois du temps différents nécessitait de la discipline et de bons outils de communication. Utiliser correctement les primitives cryptographiques sans introduire de vulnérabilités subtiles demandait une attention constante et des reviews minutieuses."
                </p>

                <p>
                    "Le projet a été couronné de succès : toutes les fonctionnalités prévues implémentées, tests passant à 100% en CI, documentation complète et professionnelle, présentation saluée par le jury, et une note finale de 18/20. Mais au-delà de la note, c'est l'expérience acquise qui compte. Rustic m'a préparé à intégrer des équipes techniques exigeantes, à travailler sur des systèmes complexes critiques, et à maintenir des standards de qualité professionnels. C'est exactement le genre de projet qui fait la différence dans un parcours de développeur."
                </p>
            </section>
        </main>
    }
}
