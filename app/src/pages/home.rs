use crate::components::project::Projects;
use crate::components::recent_contributions::RecentContributions;
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let selected_tags = RwSignal::new(vec!["⭐".to_string()]);
    view! {
        <div class="min-h-screen flex flex-col">
            <main class="flex-grow max-w-5xl mx-auto px-6 py-12">
                <section class="text-center mb-12">
                    <h1 class="text-5xl font-extrabold text-theme-primary mb-4">
                        "👋 Salut, moi c'est Evan"
                    </h1>
                    <p class="text-xl text-theme-secondary max-w-2xl mx-auto">
                        "Développeur passionné et amateur de Rust. Voici un aperçu de ce que je façonne de mes doigts musclés et sans IA !!"
                    </p>
                </section>

                <section class="mb-16">
                    <h2 class="text-3xl font-bold text-theme-primary mb-6">
                        "À propos de moi"
                    </h2>
                    <div class="space-y-4 text-theme-secondary leading-relaxed">
                        <p>
                            {r#"Je m’appelle Evan, étudiant en 3ᵉ année de BUT Informatique à l’IUT de Montpellier-Sète. Passionné par l’administration système et la cybersécurité, je développe mes compétences à travers des projets concrets et variés."#}
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
                         <a href="img/cv.png" target="_blank" class="flex items-center gap-1 text-indigo-600 hover:underline dark:text-indigo-400" download >
                             <p>
                                 {r#"Vous pouvez télécharger mon CV juste ici !"#}
                             </p>
                         </a>
                    </div>
                </section>

                <section class="mb-16">
                    <h2 class="text-3xl font-bold text-theme-primary mb-6">
                        "🛠️ Mes projets récents"
                    </h2>
                    <Projects selected_tags=selected_tags />
                </section>

                <section class="mb-16">
                    <RecentContributions
                        username="tonguechaude".to_string()
                    />
                </section>
            </main>
        </div>
    }
}
