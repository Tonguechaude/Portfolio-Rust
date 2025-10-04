use leptos::prelude::*;

#[component]
pub fn SaePage() -> impl IntoView {
    view! {
        <main class="max-w-5xl mx-auto px-6 py-12">
            <h1 class="text-4xl font-bold text-theme-primary mb-4">"SAE : Logiciel de sondage en réseau"</h1>
            <p class="mb-4 italic text-theme-secondary">"2ᵉ année BUT Informatique — DACS"</p>
            <p class="mb-2 text-theme-secondary">
                "Travail en équipe de 5 personnes en méthode AGILE. Développement d'un logiciel de sondage avec un focus sur la cryptographie."
            </p>
            <p class="mb-6 text-theme-secondary">
                "J'ai pu y approfondir mes compétences en réseau, en sécurité, et en travail collaboratif sous pression."
            </p>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-2">"Compétence 2 — Optimiser, Sélectionner les algorithme adéquats pour répondre à un problème donné"</h2>
                <ul class="list-disc list-inside space-y-2 text-theme-primary">
                    <li>
                        <strong>"AC 1 — Structures de données complexes adaptées au problème :" </strong>
                        "Implémentation d’un modèle MVC pour séparer les responsabilités. Utilisation de design patterns pour rendre l’application modulaire."
                    </li>
                    <img alt="MVC" src="/img/apprentissages/sae/mvc.png" class="rounded-lg border shadow-lg"></img>
                    <li>
                        <strong>"AC 2 — Utiliser des techniques algorithmiques adaptées :" </strong>
                        "Mise en œuvre de l’algorithme ElGamal (cryptographie asymétrique), avec génération de clés, chiffrement et déchiffrement sécurisés."
                    </li>
                    <img alt="Keygen" src="/img/apprentissages/sae/keygen.png" class="rounded-lg border shadow-lg"></img>
                    <li>
                        <strong>"AC 3 — Sécurisation des données et du code :" </strong>
                        "Sécurité centrale dans ce projet : chiffrement des bulletins, anonymisation des résultats, vérification des signatures."
                    </li>
                    <img alt="securité" src="/img/apprentissages/sae/cert.png" class="rounded-lg border shadow-lg"></img>
                </ul>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-2">"Compétence 4 — Gérer, Optimiser une base de données, interagir avec une application et mettre en oeuvre la sécurité"</h2>
                <ul class="list-disc list-inside space-y-2 text-theme-primary">
                    <li>
                        <strong>"AC 1 — Modélisation BD efficace :" </strong>
                        "Modèle de données pensé pour assurer l’intégrité et l’anonymat du vote. Utilisation de contraintes et d’index adaptés."
                    </li>
                    <img alt="bdd" src="/img/apprentissages/sae/bdd.png" class="rounded-lg border shadow-lg"></img>
                    <li>
                        <strong>"AC 2 — Confidentialité des données :" </strong>
                        "Hachage des identifiants, chiffrement des votes. Aucune donnée personnelle n'était conservée en clair."
                    </li>
                    <img alt="intégrité des données" src="/img/apprentissages/sae/integrite-donnees.png" class="rounded-lg border shadow-lg"></img>
                </ul>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-2">"Compétence 5 — Conduire, , Appliquer une démarche de suivi de projet en fonction des besoins métiers des clients et des utilisateurs"</h2>
                <ul class="list-disc list-inside space-y-2 text-theme-primary">
                    <li>
                        <strong>"AC 4 — Démarche de suivi :" </strong>
                        "Travail structuré en méthode agile avec Gitlab. Chaque itération donnait lieu à un test complet de sécurité et à un point d’équipe."
                    </li>
                    <img alt="metodologie" src="/img/apprentissages/sae/metodo.png" class="rounded-lg border shadow-lg"></img>
                </ul>
            </section>

            <p class="text-lg font-semibold text-theme-success">"Niveau global : Acquis ✅"</p>
        </main>
    }
}
