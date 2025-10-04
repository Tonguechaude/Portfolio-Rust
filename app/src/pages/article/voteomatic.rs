use leptos::prelude::*;

#[component]
pub fn VoteOmaticArticlePage() -> impl IntoView {
    view! {
        <main class="max-w-5xl mx-auto px-6 py-12 space-y-6 text-theme-primary">
            <h1 class="text-4xl font-bold">"VoteOmatic — Application de Vote en Ligne"</h1>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Contexte du Projet"</h2>
                <p>
                    "Projet académique réalisé dans le cadre du BUT Informatique. L’objectif était de développer une application de vote électronique sécurisée, conviviale et fonctionnelle."
                </p>
                <ul class="list-disc list-inside mt-2 space-y-1">
                    <li><strong>{"Commande académique :"}</strong> " Application Java pour gérer des sessions de référendums."</li>
                    <li><strong>{"Durée :"}</strong> " 2 mois."</li>
                    <li><strong>{"Équipe :"}</strong> " 4 membres."</li>
                    <li><strong>{"Technologies imposées :"}</strong> " Java."</li>
                    <li><strong>{"Objectifs :"}</strong> " sécurité des votes (ElGamal), interface claire."</li>
                </ul>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Méthodes de Travail et Résultats Obtenus"</h2>

                <h3 class="text-xl font-semibold mt-4">"Étapes du projet"</h3>
                <ol class="list-decimal list-inside space-y-2">
                    <li>
                        <strong>{"Analyse des besoins :"}</strong>
                        " Cahier des charges, fonctionnalités clés, répartition des tâches."
                    </li>
                    <li>
                        <strong>{"Développement technique :"}</strong>
                        " Front-end HTML/CSS, back-end Java/Spring, base MariaDB, auth sécurisée."
                    </li>
                    <li>
                        <strong>{"Tests et validation :"}</strong>
                        " Tests unitaires (chiffrement, résultats), scénarios utilisateur."
                    </li>
                </ol>

                <h3 class="text-xl font-semibold mt-4">"Résultats obtenus"</h3>
                <ul class="list-disc list-inside space-y-2">
                    <li>"Page de connexion sécurisée"</li>
                    <li>"Tableau de bord pour sessions de vote"</li>
                    <li>"Résultats affichés en graphiques dynamiques"</li>
                    <li>
                        <img src="/img/interface-vote.png" alt="Interface de vote" class="rounded-lg shadow-md" />
                    </li>
                    <li>
                        <img src="/img/interface-resultat.png" alt="Résultat des votes" class="rounded-lg shadow-md" />
                    </li>
                </ul>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Compétences Travaillées"</h2>

                <h3 class="text-xl font-semibold mt-2">"Techniques"</h3>
                <ul class="list-disc list-inside space-y-1">
                    <li>"Java, SpringBoot, Thymeleaf, MariaDB, Docker"</li>
                    <li>"Sécurité : ElGamal, validation d’entrée, hashage des mots de passe"</li>
                    <li>"CI/CD, gestion de conteneurs, Adminer, JDBC"</li>
                </ul>

                <h3 class="text-xl font-semibold mt-4">"Humaines"</h3>
                <ul class="list-disc list-inside space-y-1">
                    <li>"Communication et coordination dans une équipe de 4"</li>
                    <li>"Planification efficace et respect des délais"</li>
                    <li>"Apprentissage autonome autour de la cryptographie et du dev full-stack"</li>
                </ul>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Ma Contribution Personnelle"</h2>
                <ul class="list-disc list-inside space-y-1">
                    <li>"Conception de l’interface front-end (SpringBoot + Thymeleaf)"</li>
                    <li>"Mise en œuvre et modélisation de la base de données SQL"</li>
                    <li>"Implémentation CI/CD du projet"</li>
                    <li>"Gestion des conteneurs Docker (Docker Compose)"</li>
                </ul>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Conclusion"</h2>
                <p>
                    "Ce projet m’a permis de consolider mes compétences en développement web sécurisé, en gestion de projet collaboratif et en autonomie technique. Une expérience formatrice et complète."
                </p>
            </section>
        </main>
    }
}
