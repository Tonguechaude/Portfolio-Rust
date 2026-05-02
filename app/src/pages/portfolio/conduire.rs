use leptos::prelude::*;

// Compétence 5 — Conduire
// Niveau 2 : Appliquer une démarche de suivi de projet en fonction des besoins
//            métiers des clients et des utilisateurs
//
// Apprentissages Critiques (AC) typiques niveau 2 :
//   AC2.01 — Identifier et décrire les processus métiers d'un client
//   AC2.02 — Formaliser les besoins du client et de l'utilisateur
//   AC2.03 — Identifier les critères de faisabilité d'un projet
//   AC2.04 — Définir et mettre en œuvre une démarche de suivi de projet
//
// Ressources mobilisées : R6.03, R6.04, projet alternance

#[component]
pub fn PortfolioConduirePage() -> impl IntoView {
    view! {
        <main class="max-w-5xl mx-auto px-6 py-12">

            <div class="mb-8">
                <a href="/portfolio" class="text-sm text-theme-secondary hover:underline mb-4 block">
                    "← Retour au portfolio"
                </a>
                <span class="text-xs font-bold uppercase text-blue-600">"Compétence 5"</span>
                <h1 class="text-4xl font-bold text-theme-primary mt-1 mb-2">"Conduire"</h1>
                <p class="text-xl text-theme-secondary italic">
                    "Niveau 2 — Appliquer une démarche de suivi de projet en fonction des besoins métiers"
                </p>
            </div>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Mise en contexte"</h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Cette compétence s'est exercée principalement à travers deux projets menés durant \
                    mon alternance à l'ADULLACT : le développement d'un "
                    <a href="/apprentissages/ticketing" class="text-theme-accent hover:underline">
                        "outil de ticketing interne"
                    </a>
                    " (environ 6 mois, en binôme avec un chef de projet) et la migration de \
                    l'infrastructure Puppet 7 vers OpenVox 8 (menée en mode agile avec le sysadmin)."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "Dans les deux cas, le suivi de projet reposait sur des outils libres cohérents avec \
                    les valeurs de l'association : GitLab pour les issues, les merge requests et les \
                    revues de code ; Mattermost pour la communication asynchrone ; BigBlueButton pour \
                    les visios en télétravail — et surtout les échanges en présentiel, nos bureaux \
                    étant dans la même pièce."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.01 — Identifier et décrire les processus métiers d'un client"
                </h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Pour l'"
                    <a href="/apprentissages/ticketing" class="text-theme-accent hover:underline">
                        "outil de ticketing"
                    </a>
                    ", la première étape a été de cartographier les flux internes de l'ADULLACT : \
                    comment une demande d'un agent d'une collectivité adhérente était traitée, quels \
                    interlocuteurs intervenaient à chaque étape, quelles informations devaient être \
                    tracées. Ces échanges avec le chef de projet ont permis de formaliser les processus \
                    avant de toucher au code."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "Pour la migration OpenVox 8, le processus métier était différent : identifier \
                    quels nœuds du parc de 37 serveurs hébergent quels services critiques, dans quel \
                    ordre les migrer sans perturber les collectivités clientes, et quelles fenêtres \
                    de maintenance étaient acceptables."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.02 — Formaliser les besoins du client et de l'utilisateur"
                </h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Le cahier des charges de l'outil de ticketing a été co-rédigé avec le chef de \
                    projet à partir des échanges avec les parties prenantes internes. J'ai reformulé \
                    leurs attentes en spécifications techniques : modèle de données, règles de gestion \
                    des droits par rôle, cinématique des tickets. Les issues GitLab servaient ensuite \
                    de support pour découper ces spécifications en tâches actionnables."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "Pour OpenVox, la formalisation était plus technique : lister les modules Puppet \
                    existants, identifier ceux nécessitant une réécriture ou une adaptation, et \
                    documenter les incompatibilités connues entre Puppet 7 et OpenVox 8 pour guider \
                    les itérations."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.03 — Identifier les critères de faisabilité d'un projet"
                </h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Sur l'outil de ticketing, les contraintes de faisabilité portaient sur \
                    l'hébergement (serveur disponible dans l'infra ADULLACT), la stack technique \
                    compatible avec les compétences internes (Ruby on Rails), et le délai — le \
                    chef de projet avait un besoin opérationnel concret à couvrir dans un temps donné."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "Sur la migration OpenVox, la faisabilité s'évaluait nœud par nœud : certains \
                    services étaient trop critiques pour être migrés sans filet, d'autres pouvaient \
                    servir de terrain d'expérimentation. Cette analyse préalable, menée en binôme avec \
                    le sysadmin lors de points en présentiel, a permis d'éviter des blocages en \
                    production."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.04 — Définir et mettre en œuvre une démarche de suivi de projet"
                </h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Les deux projets ont été conduits en méthode agile. Pour le ticketing : sprints \
                    d'une à deux semaines, réunions hebdomadaires avec le chef de projet, démonstrations \
                    régulières pour valider les itérations et intégrer les retours. Les issues GitLab \
                    structuraient le backlog, et les MR permettaient de soumettre chaque livrable à \
                    relecture avant intégration."
                </p>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Pour OpenVox, le suivi était plus informel mais tout aussi rigoureux : points \
                    quotidiens en présentiel avec le sysadmin pour débloquer les incompatibilités au \
                    fil de l'eau, issues GitLab pour tracer chaque module migré, et Mattermost pour \
                    les jours de télétravail. Chaque nœud migré avec succès était validé et fermé \
                    comme issue avant de passer au suivant."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "Cette double expérience m'a montré qu'une démarche de suivi efficace n'est pas \
                    une question d'outils : c'est d'abord une question de communication régulière, \
                    directe, et honnête sur l'avancement réel — que ce soit en présentiel, sur \
                    Mattermost ou en visio BBB."
                </p>
            </section>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Traces et preuves"</h2>
                <ul class="list-disc list-inside space-y-2 text-theme-secondary">
                    <li>
                        "Issues et MR GitLab — "
                        <a href="https://gitlab.adullact.net" class="text-theme-accent hover:underline" target="_blank">
                            "gitlab.adullact.net"
                        </a>
                    </li>
                    <li>
                        "Page apprentissage — "
                        <a href="/apprentissages/ticketing" class="text-theme-accent hover:underline">
                            "Outil de ticketing ADULLACT"
                        </a>
                    </li>
                    <li>"Migration Puppet 7 → OpenVox 8 — suivi agile en binôme avec le sysadmin"</li>
                    <li>"Cahier des charges et spécifications techniques co-rédigés avec le chef de projet"</li>
                </ul>
            </section>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Ressources mobilisées"</h2>
                <ul class="list-disc list-inside space-y-1 text-theme-secondary">
                    <li>"R6.03 — Communication : organisation et diffusion de l'information"</li>
                    <li>"R6.04 — Projet personnel et professionnel"</li>
                </ul>
            </section>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Analyse réflexive"</h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Conduire un projet en alternance, c'est différent d'un projet académique : \
                    les délais sont réels, les utilisateurs finaux sont des agents publics dont \
                    le travail dépend du résultat, et les imprévus ne se gèrent pas avec une \
                    semaine de rallonge. Cette pression m'a appris à prioriser, à découper les \
                    livrables en incréments testables, et à ne jamais laisser un blocage s'installer \
                    sans en parler."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "La grande leçon de ces deux projets : les outils (GitLab Issues, Mattermost, BBB) \
                    ne remplacent pas la communication directe. Ils la complètent et la tracent. \
                    Les décisions les plus importantes se sont toujours prises en présentiel, \
                    puis documentées dans GitLab pour garder une mémoire partagée."
                </p>
            </section>

            <div class="flex items-center gap-3">
                <p class="text-lg font-semibold text-theme-success">"Niveau atteint : Niveau 2"</p>
                <span class="text-sm px-3 py-1 rounded-full bg-green-100 text-green-800">"Acquis"</span>
            </div>

        </main>
    }
}
