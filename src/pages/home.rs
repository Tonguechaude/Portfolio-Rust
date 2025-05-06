use crate::components::project::Projects;
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn HomePage() -> impl IntoView {
    let selected_tags = RwSignal::new(vec!["⭐".to_string()]);
    view! {
        <div class="min-h-screen flex flex-col">
            <main class="flex-grow max-w-5xl mx-auto px-6 py-12">
                <section class="text-center mb-12">
                    <h1 class="text-5xl font-extrabold text-zinc-800 dark:text-zinc-100 mb-4">
                        "👋 Salut, moi c'est Evan"
                    </h1>
                    <p class="text-xl text-zinc-600 dark:text-zinc-400 max-w-2xl mx-auto">
                        "Développeur passionné, amateur de Rust, explorateur de pixels et de performances. Voici un aperçu de ce que je bricole."
                    </p>
                </section>

                <section class="mb-16">
                    <h2 class="text-3xl font-bold text-zinc-800 dark:text-zinc-100 mb-6">
                        "👨‍💻 À propos de moi"
                    </h2>
                    <div class="space-y-4 text-zinc-700 dark:text-zinc-300 leading-relaxed">
                        <p>
                            {r#"Je m’appelle Evan, étudiant en 2ᵉ année de BUT Informatique à l’IUT de Montpellier-Sète. Passionné par l’administration système et la cybersécurité, je développe mes compétences à travers des projets concrets et variés."#}
                        </p>
                        <p>
                            {r#"Actuellement en alternance à l’ADULLACT, je contribue à des projets open source, enrichissant mes connaissances techniques dans un cadre professionnel stimulant."#}
                        </p>
                        <p>
                            {r#"Je gère également un serveur personnel sur Raspberry Pi, hébergeant entre autres ce portfolio et un serveur Nextcloud. L’automatisation avec Puppet me permet d’optimiser et de sécuriser cet environnement."#}
                        </p>
                        <p>
                            {r#"Mon intérêt pour la cybersécurité s’étend à la gestion des risques et à la sensibilisation des utilisateurs. Je vise à terme un rôle de RSSI, combinant expertise technique et stratégie de sécurité."#}
                        </p>
                        <p>
                            {r#"Curieux et engagé, je reste ouvert aux opportunités mêlant innovation technologique, sécurité et impact social via le logiciel libre."#}
                        </p>
                    </div>
                </section>

                <section class="mb-16">
                    <h2 class="text-3xl font-bold text-zinc-800 dark:text-zinc-100 mb-6">
                        "🛠️ Mes projets récents"
                    </h2>
                    <Projects selected_tags=selected_tags />
                </section>
            </main>
        </div>
    }
}
