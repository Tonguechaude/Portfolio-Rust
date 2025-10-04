use leptos::prelude::*;

#[component]
pub fn ArticlesPage() -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col bg-theme-primary">
            <main class="flex-grow max-w-5xl mx-auto px-6 py-12">
                <section class="text-center mb-12">
                    <h1 class="text-5xl font-extrabold text-theme-primary">
                        "👋 Mes articles"
                    </h1>
                    <p class="text-xl text-theme-secondary max-w-2xl mx-auto">
                        "Voici une page où je vous décris certains projets et ce qu'ils m'ont apportés"
                    </p>
                </section>

                <section>
                    <a href="/articles/voteomatic">
                        <article class="hover:shadow-md transition mb-8 p-6 border rounded-lg shadow-sm bg-theme-nav">
                            <h3 class="text-2xl font-bold text-theme-primary mb-2">
                                "VoteOmatic"
                            </h3>
                            <p class="italic text-theme-secondary mb-4">
                                "Un service de sondage en réseau en Java"
                            </p>
                        </article>
                    </a>

                    <a href="/articles/comptoir">
                        <article class="hover:shadow-md transition mb-8 p-6 border rounded-lg shadow-sm bg-theme-nav">
                            <h3 class="text-2xl font-bold text-theme-primary mb-2">
                                "Analyse des sites exposés sur le comptoir du libre"
                            </h3>
                            <p class="italic text-theme-secondary mb-4">
                                "Vérification de la disponibilitée des sites webs et du code source des applications exposées sur le comptoir du libre"
                            </p>
                        </article>
                    </a>

                    <a href="/articles/convertisseur_rust">
                        <article class="hover:shadow-md transition mb-8 p-6 border rounded-lg shadow-sm bg-theme-nav">
                            <h3 class="text-2xl font-bold text-theme-primary mb-2">
                                "Convertisseur"
                            </h3>
                            <p class="italic text-theme-secondary mb-4">
                                "Convertisseur (Binaire/Octal/Décimal/Héxadécimal) en Rust"
                            </p>
                        </article>
                    </a>

                    <a href="/articles/gol_java">
                        <article class="hover:shadow-md transition mb-8 p-6 border rounded-lg shadow-sm bg-theme-nav">
                            <h3 class="text-2xl font-bold text-theme-primary">
                                "Conway's Game Of Life"
                            </h3>
                            <p class="italic text-theme-secondary mb-4">
                                "Jeu de la vie de Conway en Java"
                            </p>
                       </article>
                    </a>
                </section>
            </main>
        </div>
    }
}
