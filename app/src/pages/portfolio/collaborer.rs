use leptos::prelude::*;

// Compétence 6 — Collaborer
// Niveau 2 : Situer son rôle et ses missions au sein d'une équipe informatique
//
// Apprentissages Critiques (AC) typiques niveau 2 :
//   AC2.01 — Comprendre la diversité et les rôles dans une équipe informatique
//   AC2.02 — Appliquer une démarche pour intégrer une équipe
//   AC2.03 — Mobiliser les compétences interpersonnelles (communication, écoute, partage)
//   AC2.04 — Rendre compte de son activité professionnelle
//
// Ressources mobilisées : R6.01, R6.03, R6.04, alternance

#[component]
pub fn PortfolioCollaborerPage() -> impl IntoView {
    view! {
        <main class="max-w-5xl mx-auto px-6 py-12">

            <div class="mb-8">
                <a href="/portfolio" class="text-sm text-theme-secondary hover:underline mb-4 block">
                    "← Retour au portfolio"
                </a>
                <span class="text-xs font-bold uppercase text-gray-500">"Compétence 6"</span>
                <h1 class="text-4xl font-bold text-theme-primary mt-1 mb-2">"Collaborer"</h1>
                <p class="text-xl text-theme-secondary italic">
                    "Niveau 2 — Situer son rôle et ses missions au sein d'une équipe informatique"
                </p>
            </div>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Mise en contexte"</h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Au sein de l'ADULLACT, j'ai travaillé principalement en binôme avec le sysadmin de \
                    l'association, expert Puppet et référent technique de l'infrastructure. Pour certains \
                    projets de développement (comme l'outil de ticketing interne), j'étais accompagné \
                    d'un chef de projet, mais en responsabilité quasi-autonome sur la partie technique."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "La collaboration au quotidien se faisait essentiellement en présentiel — nos bureaux \
                    étant dans la même pièce — ce qui favorisait des échanges directs, rapides et informels. \
                    Pour les jours de télétravail, nous utilisions BigBlueButton (BBB), une solution de \
                    visioconférence libre, cohérente avec les valeurs de l'association. La communication \
                    asynchrone passait par Mattermost, également libre."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.01 — Comprendre la diversité des rôles dans une équipe informatique"
                </h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "À l'ADULLACT, j'ai côtoyé des profils très différents : le sysadmin, expert \
                    infrastructure et Puppet avec qui je travaillais directement ; des développeurs \
                    contribuant aux logiciels libres hébergés ; des chefs de projet coordonnant les \
                    relations avec les collectivités adhérentes ; et des responsables associatifs dont \
                    les décisions orientaient les priorités techniques."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "Comprendre ces rôles m'a permis de saisir comment une décision technique \
                    (par exemple changer l'ENC Puppet) a des répercussions organisationnelles \
                    (formation, communication aux collectivités, planning de migration) et pas \
                    seulement des répercussions sur le code."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.02 — S'intégrer dans une équipe et y trouver sa place"
                </h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Mon intégration s'est construite progressivement au fil des deux années. En première \
                    année, je prenais en charge des tâches bien délimitées sous supervision du sysadmin. \
                    En deuxième année, j'ai pris plus d'autonomie, notamment sur l'écriture de modules \
                    Puppet complets et la mise en place de la supervision Prometheus/Grafana."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "Un exemple concret de cette montée en autonomie collaborative : le projet de migration \
                    de Puppet 7 vers OpenVox 8 (fork communautaire de Puppet). Ce chantier a été mené \
                    en mode agile avec le sysadmin : découpage en itérations, points réguliers pour \
                    arbitrer les problèmes de compatibilité des modules, et validation progressive \
                    nœud par nœud avant de généraliser."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.03 — Mobiliser les compétences interpersonnelles"
                </h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Travailler en binôme dans la même pièce qu'un expert demande une posture d'écoute \
                    active : savoir poser les bonnes questions, comprendre les contraintes que l'autre \
                    voit et que je ne vois pas encore, et ne pas hésiter à challenger une décision \
                    technique quand j'avais des arguments."
                </p>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Sur GitLab, la collaboration prenait une forme plus formelle : issues pour tracer \
                    les tâches, merge requests pour soumettre les modifications de modules Puppet à \
                    relecture, pipelines CI pour valider automatiquement la syntaxe avant merge. \
                    Chaque MR était l'occasion d'un échange technique entre le sysadmin et moi."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "À distance, Mattermost structurait les échanges asynchrones, permettant de \
                    garder une trace des décisions prises et d'éviter les pertes d'information \
                    entre les jours de présentiel et de télétravail."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-3">
                    "AC2.04 — Rendre compte de son activité professionnelle"
                </h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Le rendu compte passait par plusieurs canaux. Sur GitLab, chaque issue fermée et \
                    chaque MR mergée constituait une trace horodatée de mon activité. Pour les projets \
                    plus structurés (outil de ticketing, migration OpenVox), des points d'avancement \
                    réguliers avec le chef de projet ou le sysadmin permettaient de valider les \
                    itérations et de réorienter si nécessaire."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "J'ai également produit de la documentation technique pour les modules Puppet \
                    que j'ai écrits, afin que l'équipe puisse les maintenir indépendamment de ma \
                    présence — une exigence normale dans un contexte associatif où la continuité \
                    de service prime sur la centralisation des connaissances."
                </p>
            </section>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Traces et preuves"</h2>
                <ul class="list-disc list-inside space-y-2 text-theme-secondary">
                    <li>"Issues et Merge Requests sur la forge GitLab ADULLACT (gitlab.adullact.net)"</li>
                    <li>"Migration Puppet 7 → OpenVox 8 menée en binôme avec le sysadmin"</li>
                    <li>"Documentation des modules Puppet rédigée pour la continuité de service"</li>
                    <li>"Suivi de projet outil de ticketing — points d'avancement avec le chef de projet"</li>
                </ul>
            </section>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Ressources mobilisées"</h2>
                <ul class="list-disc list-inside space-y-1 text-theme-secondary">
                    <li>"R6.01 — Initiation à l'entrepreneuriat"</li>
                    <li>"R6.03 — Communication : organisation et diffusion de l'information"</li>
                    <li>"R6.04 — Projet personnel et professionnel"</li>
                </ul>
            </section>

            <section class="mb-8 p-6 border rounded-lg bg-theme-nav">
                <h2 class="text-2xl font-semibold text-theme-primary mb-3">"Analyse réflexive"</h2>
                <p class="text-theme-secondary leading-relaxed mb-3">
                    "Collaborer avec un expert comme le sysadmin de l'ADULLACT a été une expérience \
                    formatrice à double titre : technique d'abord, car sa maîtrise de Puppet m'a permis \
                    de progresser bien plus vite que je ne l'aurais fait seul ; humaine ensuite, car \
                    apprendre à travailler avec quelqu'un qui en sait beaucoup plus que soi demande \
                    une forme d'humilité et de rigueur dans la communication."
                </p>
                <p class="text-theme-secondary leading-relaxed">
                    "La migration vers OpenVox 8 a été le projet où j'ai le mieux perçu la valeur \
                    d'une démarche agile en binôme : les imprévus étaient fréquents (modules \
                    incompatibles, comportements différents entre Puppet 7 et OpenVox 8), et c'est \
                    la capacité à itérer rapidement et à ajuster ensemble les priorités qui a permis \
                    de mener cette migration à bien sans bloquer la production."
                </p>
            </section>

            <div class="flex items-center gap-3">
                <p class="text-lg font-semibold text-theme-success">"Niveau atteint : Niveau 2"</p>
                <span class="text-sm px-3 py-1 rounded-full bg-green-100 text-green-800">"Acquis"</span>
            </div>

        </main>
    }
}
