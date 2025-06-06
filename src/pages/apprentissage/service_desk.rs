use leptos::prelude::*;

#[component]
pub fn TicketingPage() -> impl IntoView {
    view! {
        <main class="max-w-5xl mx-auto px-6 py-12">
            <h1 class="text-4xl font-bold mb-4">"Outil de ticketing — Alternance ADULLACT"</h1>
            <p class="mb-4 italic">"Projet professionnel — 6 mois"</p>
            <p class="mb-2">
                "Développement en solo avec un chef de projet. J’ai géré la conception, l’architecture, le back et front-end, ainsi que les tests."
            </p>
            <p class="mb-6">
                "Méthode agile, suivis réguliers, démonstrations, prise en compte de retours, et documentation complète."
            </p>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-green-700 mb-2">"Compétence 2 — Optimiser, Sélectionner les algorithme adéquats pour répondre à un problème donné"</h2>
                <ul class="list-disc list-inside space-y-2 text-zinc-800">
                    <li>
                        <strong>"AC 1 — Choisir des structures de données complexes adaptées au problème :"</strong>
                        "L’architecture de l’outil a été pensée pour séparer les entités principales : tickets, utilisateurs, services… J’ai conçu des classes Ruby modélisant précisément les objets métier, en respectant les principes SOLID, facilitant leur évolution."
                    </li>
                    <img alt="class ruby" src="/img/apprentissages/service-desk/ruby.png" class="rounded-lg border shadow-lg"></img>
                    <li>
                        <strong>"AC 3 — Comprendre les enjeux et moyens de sécurisation des données et du code :"</strong>
                        "Une attention particulière a été portée à la gestion des droits : les rôles des utilisateurs conditionnaient les accès, avec des vérifications côté serveur. Les échanges sensibles étaient sécurisés."
                    </li>
                    <img alt="acl" src="/img/apprentissages/service-desk/acl.png" class="rounded-lg border shadow-lg"></img>
                    <li>
                        <strong>"AC 4 — Évaluer l’impact environnemental et sociétal des solutions proposées :" </strong>
                        "Le projet visait à centraliser la gestion des demandes internes, remplaçant des traitements par mail. Cette dématérialisation améliore la traçabilité tout en réduisant les échanges dispersés et redondants."
                    </li>
                    <img alt="service-desk" src="/img/apprentissages/service-desk/sd.png" class="rounded-lg border shadow-lg"></img>
                </ul>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-green-700 mb-2">"Compétence 4 — Gérer, Optimiser une base de données, interagir avec une application et mettre en oeuvre la sécurité"</h2>
                <ul class="list-disc list-inside space-y-2 text-zinc-800">
                    <li>
                        <strong>"AC 2 — Assurer la confidentialité des données :" </strong>
                        "Connexions sécurisées, contrôle des accès, données sensibles protégées. Les identifiants des agents étaient stockés de manière sécurisée, et les permissions finement gérées."
                    </li>
                    <img alt="service-desk login" src="/img/apprentissages/service-desk/login.png" class="rounded-lg border shadow-lg"></img>
                </ul>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-green-700 mb-2">"Compétence 5 — Conduire, Appliquer une démarche de suivi de projet en fonction des besoins métiers des clients et des utilisateurs"</h2>
                <ul class="list-disc list-inside space-y-2 text-zinc-800">
                    <li>
                        <strong>"AC 1 — Identifier les processus présents dans une organisation :" </strong>
                        "Une analyse des flux internes a été effectuée pour bien comprendre les besoins des utilisateurs et leurs contraintes métier."
                    </li>
                    <li>
                        <strong>"AC 2 — Formaliser les besoins du client et de l’utilisateur :" </strong>
                        "Le cahier des charges a été co-rédigé à partir d’échanges avec les parties prenantes. J’ai reformulé leurs attentes sous forme de spécifications techniques."
                    </li>
                    <li>
                        <strong>"AC 3 — Identifier les critères de faisabilité d’un projet informatique :"</strong>
                        "Des contraintes techniques (serveur, sécurité, hébergement) ont été prises en compte dès l’amorce du projet."
                    </li>
                    <li>
                        <strong>"AC 4 — Définir et mettre en œuvre une démarche de suivi de projet :" </strong>
                        "Projet mené en méthode AGILE : sprints, réunions hebdo, démos régulières. J’ai documenté le projet et mis en place des outils de suivi."
                    </li>
                </ul>
            </section>

            <p class="text-lg font-semibold text-green-800">"Niveau global sur les compétences mobilisées : Acquis ✅"</p>
        </main>
    }
}
