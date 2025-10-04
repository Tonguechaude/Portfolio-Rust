use leptos::prelude::*;

#[component]
pub fn ComptoirPage() -> impl IntoView {
    view! {
        <main class="max-w-5xl mx-auto px-6 py-12">
            <h1 class="text-4xl font-bold text-theme-primary mb-4">"Audit automatisé — Comptoir du Libre"</h1>
            <p class="mb-4 italic text-theme-secondary">"Projet d'alternance — 2 semaines"</p>
            <p class="mb-2 text-theme-secondary">
                "Projet en autonomie à la demande du développeur principal. Scan de tous les logiciels via API pour identifier ceux obsolètes ou non maintenus."
            </p>
            <p class="mb-6 text-theme-secondary">
                "Déploiement rapide, tests unitaires, interprétation des résultats, et documentation à destination du Comptoir."
            </p>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-2">"Compétence 2 — Optimiser des applications"</h2>
                <ul class="list-disc list-inside space-y-2 text-theme-primary">
                    <li>
                        <strong>"AC 1 — Choix de structures adaptées :" </strong>
                        "Analyse des besoins du Comptoir → définition d’un modèle permettant de stocker efficacement les métadonnées logicielles (dates, versions, statuts) avec SQLite."
                    </li>
                    <img alt="bdd" src="/img/apprentissages/comptoir/rusqlite.png" class="rounded-lg border shadow-lg"></img>
                    <li>
                        <strong>"AC 2 — Algorithmes adaptés :" </strong>
                        "Écriture d’un analyseur automatique capable de traiter les retours de l’API publique, d’en extraire les données pertinentes, puis de les filtrer selon des critères de fraîcheur/pertinence."
                    </li>
                    <img alt="api" src="/img/apprentissages/comptoir/api_management.png" class="rounded-lg border shadow-lg"></img>
                    <li>
                        <strong>"AC 3 — Sécurité :" </strong>
                        "Bien que le projet soit plus orienté data, le code a été écrit avec rigueur pour éviter les erreurs (validation des entrées, traitement des erreurs API, etc.)."
                    </li>
                    <img alt="error" src="/img/apprentissages/comptoir/gestion_erreur.png" class="rounded-lg border shadow-lg"></img>
                </ul>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-2">"Compétence 4 — Gérer des données de l'information"</h2>
                <ul class="list-disc list-inside space-y-2 text-theme-primary">
                    <li>
                        <strong>"AC 3 — Restitution et visualisation :"</strong>
                        "Résultats présentés sous forme de rapports clairs et lisibles pour le client."
                    </li>
                    <img alt="error" src="/img/apprentissages/comptoir/resultat.png" class="rounded-lg border shadow-lg"></img>
                    <li>
                        <strong>"AC 4 — Données hétérogènes :"</strong>
                        "Traitement de données variées issues de l’API : texte, dates, URLs. Parsing et homogénéisation pour en tirer des conclusions utiles."
                    </li>
                </ul>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold text-theme-success mb-2">"Compétence 5 — Conduire un projet"</h2>
                <ul class="list-disc list-inside space-y-2 text-theme-primary">
                    <li>
                        <strong>"AC 2 — Formalisation du besoin client :" </strong>
                        "Échanges directs avec le développeur principal pour bien comprendre le besoin d’audit, puis reformulation dans une documentation."
                    </li>
                    <li>
                        <strong>"AC 4 — Suivi de projet :"</strong>
                        "Bien que court, ce projet a été structuré autour de jalons précis : spécification, développement, livraison, retour client."
                    </li>
                </ul>
            </section>

            <p class="text-lg font-semibold text-theme-success">"Niveau global : Acquis ✅"</p>
        </main>
    }
}
