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
                <p class="text-theme-secondary leading-relaxed">
                    "À l'issue de trois années de formation en BUT Informatique, je me trouve à un moment \
                    charnière de mon parcours : celui où les compétences acquises en formation et les \
                    expériences vécues sur le terrain commencent à dessiner les contours d'un projet \
                    professionnel cohérent et assumé. Mon cursus m'a permis de construire des bases solides \
                    en développement logiciel, administration systèmes et réseaux, gestion de bases de données \
                    et conduite de projets. C'est surtout mon alternance de deux ans au sein de l'ADULLACT — \
                    l'Association des Développeurs et Utilisateurs de Logiciels Libres pour les Administrations \
                    et les Collectivités Territoriales — qui a le plus profondément orienté ma réflexion \
                    professionnelle, en m'ancrant dans des valeurs de souveraineté numérique et de service public."
                </p>
            </section>

            // ── Démarche portfolio ─────────────────────────────────────────────
            <section class="mb-12 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">
                    "Démarche portfolio"
                </h2>
                <p class="text-theme-secondary leading-relaxed">
                    "Ce portfolio rassemble les traces et analyses réflexives produites au cours du semestre 6. \
                    J'ai sélectionné pour chaque compétence les réalisations les plus significatives de mon \
                    alternance à l'ADULLACT et de mes projets personnels, en cherchant à montrer non seulement \
                    ce que j'ai fait, mais pourquoi et ce que j'en ai appris. La démarche réflexive consiste \
                    à prendre du recul sur mes pratiques : identifier ce qui a fonctionné, comprendre les \
                    difficultés rencontrées et formaliser les apprentissages pour construire une posture \
                    professionnelle cohérente."
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
                        <h3 class="font-semibold text-theme-primary mb-2">"R6.01 — Initiation à l'entrepreneuriat"</h3>
                        <p class="text-sm text-theme-secondary leading-relaxed mb-2">
                            "Stage d'initiation à l'entrepreneuriat animé par M. Chollet. En équipe, \
                            nous avons conçu le modèle économique du projet HoloPast — une solution \
                            d'hologramme reposant sur un format ouvert et libre, permettant à n'importe \
                            qui de créer ses propres hologrammes sans dépendance à un logiciel propriétaire. \
                            Nous avons terminé 3ᵉ de la promotion."
                        </p>
                        <p class="text-sm text-theme-secondary leading-relaxed">
                            "Ce que j'en retiens : la réflexion sur un modèle économique basé sur \
                            l'open source n'est pas si éloignée de ce que fait l'ADULLACT au quotidien — \
                            valoriser le libre, mutualiser les coûts, et trouver des sources de financement \
                            pérennes sans verrouiller les utilisateurs."
                        </p>
                    </div>

                    <div class="p-4 border rounded-lg bg-theme-nav">
                        <h3 class="font-semibold text-theme-primary mb-2">"R6.02 — Droit du numérique et de la propriété intellectuelle"</h3>
                        <p class="text-sm text-theme-secondary leading-relaxed mb-2">
                            "En formation, j'ai conduit des analyses d'impact sur les systèmes d'IA \
                            autonomes : le Règlement IA (AI Act européen), le RIA, et le RGPD appliqué \
                            aux traitements automatisés. Ces analyses m'ont appris à lire un cadre \
                            réglementaire, à identifier les obligations concrètes et à évaluer les \
                            risques juridiques d'un système logiciel."
                        </p>
                        <p class="text-sm text-theme-secondary leading-relaxed">
                            "En alternance, ces enjeux sont devenus très concrets : le service \
                            demarches.adullact.org héberge des données hautement identifiables de \
                            citoyens, soumises à des contraintes strictes de souveraineté et de \
                            sécurité. Travailler dans une association de logiciel libre m'a également \
                            donné une bonne culture des licences (GPL, AGPL, MIT, EUPL) et de leurs \
                            implications juridiques pour les administrations publiques."
                        </p>
                    </div>

                    <div class="p-4 border rounded-lg bg-theme-nav">
                        <h3 class="font-semibold text-theme-primary mb-2">"R6.03 — Communication : organisation et diffusion de l'information"</h3>
                        <p class="text-sm text-theme-secondary leading-relaxed">
                            "Cette ressource a débouché sur l'élaboration d'une formation destinée \
                            à des PME, portant sur les enjeux de l'adoption du logiciel libre face \
                            aux solutions propriétaires. Le livrable final combinait plusieurs formats : \
                            visuels pédagogiques, présentation orale, rapport écrit et poster. \
                            Cet exercice m'a appris à adapter le discours technique à un public non \
                            spécialiste — compétence que je mobilise aussi quand je documente des \
                            modules Puppet ou que je présente un avancement projet à un chef de projet."
                        </p>
                    </div>

                    // ── R6.04 : Projet personnel et professionnel ──────────────
                    <div class="p-4 border rounded-lg bg-theme-nav">
                        <h3 class="font-semibold text-theme-primary mb-3">
                            "R6.04 — Projet personnel et professionnel"
                        </h3>

                        <p class="text-sm text-theme-secondary mb-4 leading-relaxed">
                            "Cette ressource m'a conduit à formaliser deux projets distincts mais complémentaires \
                            envisagés à l'issue de mon BUT. Ils convergent tous deux vers le même objectif : \
                            devenir un professionnel capable d'allier expertise technique en sécurité des \
                            systèmes et engagement éthique au service du secteur public."
                        </p>

                        // Projet 1
                        <div class="mb-4 p-4 border-l-4 border-blue-500 bg-theme-card rounded-r-lg">
                            <h4 class="font-bold text-theme-primary mb-1">
                                "Projet 1 — Master Informatique au CNAM (cybersécurité)"
                            </h4>
                            <p class="text-sm text-theme-secondary leading-relaxed mb-2">
                                "Poursuite d'études vers un Master Sécurité informatique, cybersécurité et \
                                cybermenaces (RNCP39278) délivré par le Conservatoire National des Arts et \
                                Métiers. Programme en deux ans (M1 + M2, 60 ECTS chacun) couvrant : sécurité \
                                des réseaux, droit et conformité, criminologie, posture de l'attaquant, \
                                hacking réseau, audit de sécurité, forensique, rétro-ingénierie et mémoire \
                                de fin d'études."
                            </p>
                            <p class="text-sm text-theme-secondary leading-relaxed mb-2">
                                "Motivation : mon expérience à l'ADULLACT m'a exposé à des systèmes manipulant \
                                des données sensibles de citoyens et m'a rendu conscient des enjeux de sécurité \
                                autour des logiciels libres dans le secteur public. Protéger ces infrastructures \
                                est pour moi une responsabilité citoyenne et éthique. Le CNAM, ouvert aux \
                                alternants et actifs, résonne également avec ma sensibilité au service public."
                            </p>
                            <p class="text-sm text-theme-secondary leading-relaxed">
                                "Atouts : bases techniques solides du BUT, maturité professionnelle issue de \
                                l'alternance, autonomie dans l'apprentissage. Contrainte principale : accès \
                                sélectif sur dossier (niveau licence exigé, lettre de motivation déterminante)."
                            </p>
                        </div>

                        // Projet 2
                        <div class="p-4 border-l-4 border-green-500 bg-theme-card rounded-r-lg">
                            <h4 class="font-bold text-theme-primary mb-1">
                                "Projet 2 — CDI à l'ADULLACT (Admin Sys & DevSecOps junior)"
                            </h4>
                            <p class="text-sm text-theme-secondary leading-relaxed mb-2">
                                "Intégrer l'ADULLACT en CDI en qualité d'Administrateur Systèmes & Réseaux \
                                avec une composante DevSecOps junior. Missions : maintien des infrastructures \
                                hébergeant la forge GitLab et les services des collectivités adhérentes, \
                                automatisation via Puppet (Infrastructure as Code), intégration de la sécurité \
                                dans les pipelines CI/CD, tests SAST/DAST, sécurisation des environnements \
                                conteneurisés."
                            </p>
                            <p class="text-sm text-theme-secondary leading-relaxed mb-2">
                                "Motivation : deux ans d'alternance ont ancré en moi une vraie passion pour \
                                le code d'infrastructure et la conviction que la technologie doit être au \
                                service de l'intérêt général. La philosophie du logiciel libre — transparence, \
                                souveraineté, mutualisation — est pour moi une conviction éthique autant \
                                que technique."
                            </p>
                            <p class="text-sm text-theme-secondary leading-relaxed">
                                "Atouts : connaissance intime de la structure, maîtrise concrète de Puppet \
                                (rare chez un profil junior), confiance déjà témoignée par l'association. \
                                Contraintes : rémunération inférieure au secteur privé, montée en compétences \
                                autonome sans cadre académique, taille limitée de la structure."
                            </p>
                        </div>

                        // Scénario idéal
                        <div class="mt-4 p-3 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-700 rounded-lg">
                            <p class="text-sm text-theme-secondary leading-relaxed italic">
                                "Scénario idéal : combiner les deux en intégrant le Master CNAM en alternance \
                                tout en continuant à l'ADULLACT — configuration que l'association a déjà \
                                signifié être envisageable, et qui correspond précisément au modèle pédagogique \
                                que promeut le CNAM."
                            </p>
                        </div>
                    </div>

                    <div class="p-4 border rounded-lg bg-theme-nav">
                        <h3 class="font-semibold text-theme-primary mb-2">"R6.Deploi.05 — Optimisation des services complexes"</h3>
                        <p class="text-sm text-theme-secondary leading-relaxed">
                            "Déploiement d'une infrastructure complète via Terraform, Ansible et Puppet : \
                            une instance GitLab, un Nextcloud, et une stack LAMP accompagnée d'une \
                            supervision Prometheus et d'un monitoring Grafana. Ce TP m'a permis de \
                            combiner trois outils d'automatisation complémentaires — provisionnement \
                            de l'infrastructure avec Terraform, configuration initiale des machines \
                            avec Ansible, gestion de l'état à long terme avec Puppet — et de mettre \
                            en place dès le départ la supervision des services déployés. Une approche \
                            directement en lien avec ce que je pratique à l'ADULLACT."
                        </p>
                    </div>

                    <div class="p-4 border rounded-lg bg-theme-nav">
                        <h3 class="font-semibold text-theme-primary mb-2">"R6.Deploi.06 — Cloud computing"</h3>
                        <p class="text-sm text-theme-secondary leading-relaxed">
                            "Visite du CINES (Centre Informatique National de l'Enseignement Supérieur), \
                            hébergeant le supercalculateur Ad Astra. Cette ressource a couvert l'étude \
                            des systèmes Unix et de la conteneurisation (Podman et Docker), le \
                            provisionnement d'infrastructure avec Terraform, l'utilisation d'hyperviseurs \
                            et la gestion des ressources machines. Ces notions font directement écho \
                            à l'infrastructure OVH de l'ADULLACT : serveurs hypervisés, gestion de \
                            ressources à l'échelle d'un parc de 37 nœuds, et réflexion sur la \
                            souveraineté numérique des données hébergées."
                        </p>
                    </div>

                </div>
            </section>

            // ── Bilan global ───────────────────────────────────────────────────
            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Bilan et perspectives"</h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Les deux projets envisagés ne s'opposent pas : ils se répondent et se nourrissent \
                    mutuellement. Le Master du CNAM apporterait une expertise approfondie en cybersécurité \
                    et une légitimité académique ; le CDI à l'ADULLACT offrirait une immersion professionnelle \
                    immédiate et la possibilité de mettre en pratique les savoir-faire acquis. Loin d'être \
                    des chemins divergents, ces deux trajectoires convergent vers le même objectif."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "Au-delà des choix de formation ou d'emploi, ce parcours reflète une vision du métier \
                    construite progressivement : être informaticien, ce n'est pas seulement résoudre des \
                    problèmes techniques, c'est aussi choisir pour qui et pourquoi on les résout. Travailler \
                    sur des logiciels libres déployés dans des collectivités, sécuriser des infrastructures \
                    de service public, contribuer à un numérique transparent et souverain — voilà ce qui \
                    donne du sens à mon engagement professionnel."
                </p>
            </section>

        </main>
    }
}
