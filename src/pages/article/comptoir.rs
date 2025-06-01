use leptos::prelude::*;

#[component]
pub fn ComptoirArticlePage() -> impl IntoView {
    view! {
        <main class="max-w-5xl mx-auto px-6 py-12 space-y-6 text-zinc-800 dark:text-zinc-100">
            <h1 class="text-4xl font-bold">"Analyse des Logiciels Libres du Comptoir du Libre"</h1>

            <p class="text-lg">
                "Un outil en Rust pour analyser les logiciels libres du Comptoir du Libre en vérifiant la disponibilité de leurs sites web et de leurs dépôts de code source. "
                <a href="https://comptoir-du-libre.org/" class="text-blue-600 underline" target="_blank">"Voir le site"</a>
                " — "
                <a href="https://gitlab.adullact.net/echallias/comptoir_tests_urls" class="text-blue-600 underline" target="_blank">"Code source sur GitLab"</a>
            </p>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Contexte du Projet"</h2>
                <p>
                    "Réalisé dans le cadre de mon alternance à l’"
                    <a href="https://adullact.org" class="text-blue-600 underline" target="_blank">"ADULLACT"</a>
                    ", ce projet vise à évaluer la fiabilité des ressources en ligne des logiciels libres métiers recensés par le Comptoir du Libre."
                </p>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Méthodes de Travail et Résultats Obtenus"</h2>
                <ol class="list-decimal list-inside space-y-2">
                    <li>"Téléchargement du fichier JSON depuis le site du Comptoir."</li>
                    <li>"Utilisation de `reqwest` pour envoyer des requêtes HTTP asynchrones."</li>
                    <li>"Contrôle du nombre de requêtes concurrentes via `tokio::Semaphore`."</li>
                    <li>"Stockage des résultats dans une base de données SQLite (`rusqlite`)."</li>
                    <li>"Exportation finale des données dans un fichier CSV pour analyse."</li>
                </ol>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Compétences Travaillées"</h2>

                <h3 class="text-xl font-semibold mt-2">"Compétences techniques"</h3>
                <ul class="list-disc list-inside space-y-1">
                    <li>"Programmation en Rust (gestion d'erreurs, modules, crates externes)."</li>
                    <li>"Programmation asynchrone avec `tokio`, futures, sémaphores."</li>
                    <li>"Requêtes HTTP avec `reqwest`, gestion fine des statuts et des erreurs."</li>
                    <li>"Manipulation de fichiers JSON avec `serde` / `serde_json`."</li>
                    <li>"Interaction avec SQLite via `rusqlite`, insertion efficace de données."</li>
                    <li>"Export CSV pour exploitation dans d’autres outils (LibreOffice, Excel)."</li>
                </ul>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Exemple de Code"</h2>
                <pre class="bg-zinc-900 text-green-200 p-4 rounded-md overflow-x-auto text-sm">
                    <code>
                        {r###"
use reqwest::Client;
use reqwest::StatusCode;

async fn test_url(client: &Client, url: &str) -> Result<StatusCode, String> {
    println!("Test en cours pour l'URL: {}", url);
    match client
        .head(url)
        .header("User-Agent", "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:132.0) Gecko/20100101 Firefox/132.0")
        .send()
        .await
    {
        Ok(res) => {
            println!("Réponse reçue pour {}: {}", url, res.status());
            Ok(res.status())
        }
        Err(e) => {
            let error_message = if e.is_timeout() {
                format!("Erreur: Timeout pour {}", url)
            } else if e.is_connect() {
                format!("Erreur de connexion pour {}: {}", url, e)
            } else {
                format!("Erreur lors de la requête pour {}: {}", url, e)
            };
            println!("{}", error_message);
            Err(error_message)
        }
    }
}
"###}
                    </code>
                </pre>
            </section>

            <section>
                <h2 class="text-2xl font-semibold mt-6 mb-2">"Conclusion"</h2>
                <p>
                    "Ce projet a permis de mettre en œuvre des techniques avancées en Rust tout en apportant une vraie valeur ajoutée à l’analyse des logiciels libres. Il renforce mes compétences en programmation réseau, gestion de données et outils d’analyse."
                </p>
            </section>
        </main>
    }
}
