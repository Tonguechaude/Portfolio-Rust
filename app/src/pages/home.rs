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
                         <a href="cv/cv.pdf" target="_blank" class="flex items-center gap-1 text-indigo-600 hover:underline dark:text-indigo-400" download >
                             <p>
                                 {r#"Vous pouvez télécharger mon CV juste ici !"#}
                             </p>
                         </a>
                    </div>
                </section>

                <section class="mb-16">
                    <h2 class="text-3xl font-bold text-theme-primary mb-6">
                        "Actuellement"
                    </h2>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                        // Contribution active
                        <a
                            href="https://github.com/ferrumc-rs/ferrumc"
                            target="_blank"
                            class="group relative bg-theme-nav border border-zinc-200 dark:border-zinc-700 rounded-2xl p-6 hover:shadow-2xl hover:scale-105 transition-all duration-300 overflow-hidden"
                        >
                            <div class="absolute inset-0 bg-gradient-to-br from-orange-500/10 to-red-500/10 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                            <div class="relative z-10">
                                <div class="flex items-center gap-4 mb-4">
                                    <div class="p-3 bg-orange-100 dark:bg-orange-900/30 rounded-xl">
                                        <span class="text-2xl">"🔥"</span>
                                    </div>
                                    <div>
                                        <h3 class="text-xl font-bold text-theme-primary">"Je contribue à"</h3>
                                        <p class="text-sm text-theme-secondary">"Open Source"</p>
                                    </div>
                                </div>
                                <p class="text-orange-500 dark:text-orange-400 font-semibold text-lg">"Ferrumc"</p>
                                <p class="text-sm text-theme-secondary mt-2">
                                    "Serveur Minecraft haute performance écrit en Rust"
                                </p>
                            </div>
                        </a>

                        // Apprentissage
                        <a
                            href="https://bevyengine.org"
                            target="_blank"
                            class="group relative bg-theme-nav border border-zinc-200 dark:border-zinc-700 rounded-2xl p-6 hover:shadow-2xl hover:scale-105 transition-all duration-300 overflow-hidden"
                        >
                            <div class="absolute inset-0 bg-gradient-to-br from-purple-500/10 to-pink-500/10 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                            <div class="relative z-10">
                                <div class="flex items-center gap-4 mb-4">
                                    <div class="p-3 bg-purple-100 dark:bg-purple-900/30 rounded-xl">
                                        <span class="text-2xl">"📚"</span>
                                    </div>
                                    <div>
                                        <h3 class="text-xl font-bold text-theme-primary">"J'apprends"</h3>
                                        <p class="text-sm text-theme-secondary">"Game Dev"</p>
                                    </div>
                                </div>
                                <p class="text-purple-500 dark:text-purple-400 font-semibold text-lg">"Bevy Engine"</p>
                                <p class="text-sm text-theme-secondary mt-2">
                                    "Moteur de jeu data-driven en Rust"
                                </p>
                            </div>
                        </a>

                        // Objectif
                        <div class="group relative bg-theme-nav border border-zinc-200 dark:border-zinc-700 rounded-2xl p-6 hover:shadow-2xl hover:scale-105 transition-all duration-300 overflow-hidden cursor-default">
                            <div class="absolute inset-0 bg-gradient-to-br from-green-500/10 to-emerald-500/10 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                            <div class="relative z-10">
                                <div class="flex items-center gap-4 mb-4">
                                    <div class="p-3 bg-green-100 dark:bg-green-900/30 rounded-xl">
                                        <span class="text-2xl">"🎯"</span>
                                    </div>
                                    <div>
                                        <h3 class="text-xl font-bold text-theme-primary">"Mon objectif"</h3>
                                        <p class="text-sm text-theme-secondary">"Philosophie"</p>
                                    </div>
                                </div>
                                <p class="text-green-500 dark:text-green-400 font-semibold text-lg">"Arrêter de réinventer la roue"</p>
                                <p class="text-sm text-theme-secondary mt-2">
                                    "Utiliser plus de libs existantes, contribuer plutôt que recréer"
                                </p>
                            </div>
                        </div>
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
