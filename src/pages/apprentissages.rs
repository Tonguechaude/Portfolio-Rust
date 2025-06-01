use leptos::prelude::*;

#[component]
pub fn ApprentissagePage() -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col bg-white dark:bg-gray-900">
            <main class="flex-grow max-w-5xl mx-auto px-6 py-12">
                <section class="text-center mb-12">
                    <h1 class="text-5xl font-extrabold text-zinc-800 dark:text-zinc-100 mb-4">
                        "👋 Mes apprentissages"
                    </h1>
                    <p class="text-xl text-zinc-600 dark:text-zinc-400 max-w-2xl mx-auto">
                        "Voici un espace où je partage mes acquis, notes, et réflexions issues de ma formation."
                    </p>
                </section>

                <section>
                    <a href="/apprentissages/ticketing">
                        <article class="hover:shadow-md transition mb-8 p-6 border rounded-lg shadow-sm bg-gray-50 dark:bg-gray-800">
                            <h3 class="text-2xl font-bold text-zinc-900 dark:text-zinc-100 mb-2">
                                "Outil de ticketing — Alternance à l'ADULLACT"
                            </h3>
                            <p class="italic text-zinc-600 dark:text-zinc-400 mb-4">
                                "Projet professionnel sur 6 mois"
                            </p>
                            <p class="mb-2 text-zinc-700 dark:text-zinc-300">
                                "Projet réalisé en binôme : moi-même (développeur) et un chef de projet."
                            </p>
                            <p class="mb-2 text-zinc-700 dark:text-zinc-300">
                                "Conception et développement complet d’un outil de ticketing adapté aux besoins internes de l'association."
                            </p>
                            <p class="mb-2 text-zinc-700 dark:text-zinc-300">
                                "Suivi en méthode AGILE avec itérations régulières, retours utilisateurs, et documentation."
                            </p>
                        </article>
                    </a>

                    <a href="/apprentissages/sae">
                        <article class="hover:shadow-md transition mb-8 p-6 border rounded-lg shadow-sm bg-gray-50 dark:bg-gray-800">
                            <h3 class="text-2xl font-bold text-zinc-900 dark:text-zinc-100 mb-2">
                                "SAE : Logiciel de sondage en réseau"
                            </h3>
                            <p class="italic text-zinc-600 dark:text-zinc-400 mb-4">
                                "2ᵉ année BUT Informatique — DACS"
                            </p>
                            <p class="mb-2 text-zinc-700 dark:text-zinc-300">
                                "Travail en équipe de 5 personnes en méthode AGILE."
                            </p>
                            <p class="mb-2 text-zinc-700 dark:text-zinc-300">
                                "Développement d'un logiciel de sondage réseau avec un focus sur la cryptographie."
                            </p>
                        </article>
                    </a>

                    <a href="/apprentissages/comptoir">
                        <article class="hover:shadow-md transition mb-8 p-6 border rounded-lg shadow-sm bg-gray-50 dark:bg-gray-800">
                            <h3 class="text-2xl font-bold text-zinc-900 dark:text-zinc-100 mb-2">
                                "Audit automatisé du Comptoir du Libre"
                            </h3>
                            <p class="italic text-zinc-600 dark:text-zinc-400 mb-4">
                                "Projet d'alternance — 2 semaines"
                            </p>
                            <p class="mb-2 text-zinc-700 dark:text-zinc-300">
                                "Mission confiée par le développeur principal du Comptoir du Libre."
                            </p>
                            <p class="mb-2 text-zinc-700 dark:text-zinc-300">
                                "Développement d’un outil de scan basé sur l’API publique du Comptoir pour identifier les logiciels potentiellement obsolètes ou inactifs."
                            </p>
                            <p class="mb-2 text-zinc-700 dark:text-zinc-300">
                                "Projet mené de manière autonome, de la spécification à la livraison, avec documentation des résultats."
                            </p>
                        </article>
                    </a>
                </section>
            </main>
        </div>
    }
}
