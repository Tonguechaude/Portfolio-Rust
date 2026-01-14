use leptos::prelude::*;

#[component]
pub fn ArticlesPage() -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col">
            <main class="flex-grow max-w-5xl mx-auto px-6 py-12">
                <section class="text-center mb-12">
                    <h1 class="text-5xl font-extrabold text-theme-primary mb-4">
                        "Mes articles"
                    </h1>
                    <p class="text-xl text-theme-secondary max-w-2xl mx-auto">
                        "Retours d'expérience sur mes projets personnels, académiques et contributions open-source"
                    </p>
                </section>

                // Featured articles - Projects récents avec articles détaillés
                <section class="mb-12">
                    <h2 class="text-2xl font-bold text-theme-primary mb-6">
                        "Articles détaillés"
                    </h2>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        // Portfolio Rust
                        <a
                            href="/articles/portfolio_rust"
                            class="group relative bg-theme-nav border border-zinc-200 dark:border-zinc-700 rounded-2xl p-8 hover:shadow-2xl hover:scale-105 transition-all duration-300 overflow-hidden"
                        >
                            <div class="absolute inset-0 bg-gradient-to-br from-orange-500/10 to-red-500/10 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                            <div class="relative z-10">
                                <div class="mb-4">
                                    <span class="text-xs font-semibold text-orange-600 dark:text-orange-400 uppercase tracking-wide">"Personnel • DevOps"</span>
                                </div>
                                <h3 class="text-xl font-bold text-theme-primary mb-3">
                                    "Portfolio en Rust avec Leptos"
                                </h3>
                                <p class="text-sm text-theme-secondary leading-relaxed">
                                    "Site statique performant avec Leptos et WebAssembly, pipeline CI/CD complète sur VPS personnel avec GitLab Runner"
                                </p>
                                <div class="mt-4 flex flex-wrap gap-2">
                                    <span class="px-2 py-1 text-xs bg-orange-100 dark:bg-orange-900/30 text-orange-600 dark:text-orange-400 rounded-full">"Rust"</span>
                                    <span class="px-2 py-1 text-xs bg-orange-100 dark:bg-orange-900/30 text-orange-600 dark:text-orange-400 rounded-full">"WASM"</span>
                                    <span class="px-2 py-1 text-xs bg-orange-100 dark:bg-orange-900/30 text-orange-600 dark:text-orange-400 rounded-full">"CI/CD"</span>
                                </div>
                            </div>
                        </a>

                        // FerruMC
                        <a
                            href="/articles/ferrumc"
                            class="group relative bg-theme-nav border border-zinc-200 dark:border-zinc-700 rounded-2xl p-8 hover:shadow-2xl hover:scale-105 transition-all duration-300 overflow-hidden"
                        >
                            <div class="absolute inset-0 bg-gradient-to-br from-green-500/10 to-emerald-500/10 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                            <div class="relative z-10">
                                <div class="mb-4">
                                    <span class="text-xs font-semibold text-green-600 dark:text-green-400 uppercase tracking-wide">"Open Source"</span>
                                </div>
                                <h3 class="text-xl font-bold text-theme-primary mb-3">
                                    "Contributions à FerruMC"
                                </h3>
                                <p class="text-sm text-theme-secondary leading-relaxed">
                                    "Serveur Minecraft en Rust, apprentissage de Bevy ECS et du développement collaboratif international"
                                </p>
                                <div class="mt-4 flex flex-wrap gap-2">
                                    <span class="px-2 py-1 text-xs bg-green-100 dark:bg-green-900/30 text-green-600 dark:text-green-400 rounded-full">"Rust"</span>
                                    <span class="px-2 py-1 text-xs bg-green-100 dark:bg-green-900/30 text-green-600 dark:text-green-400 rounded-full">"Bevy"</span>
                                    <span class="px-2 py-1 text-xs bg-green-100 dark:bg-green-900/30 text-green-600 dark:text-green-400 rounded-full">"ECS"</span>
                                </div>
                            </div>
                        </a>

                        // Rustic
                        <a
                            href="/articles/rustic"
                            class="group relative bg-theme-nav border border-zinc-200 dark:border-zinc-700 rounded-2xl p-8 hover:shadow-2xl hover:scale-105 transition-all duration-300 overflow-hidden"
                        >
                            <div class="absolute inset-0 bg-gradient-to-br from-blue-500/10 to-cyan-500/10 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                            <div class="relative z-10">
                                <div class="mb-4">
                                    <span class="text-xs font-semibold text-blue-600 dark:text-blue-400 uppercase tracking-wide">"Académique • Équipe"</span>
                                </div>
                                <h3 class="text-xl font-bold text-theme-primary mb-3">
                                    "Rustic - Logiciel de Sauvegarde"
                                </h3>
                                <p class="text-sm text-theme-secondary leading-relaxed">
                                    "Outil de backup CLI en équipe de 4, déduplication content-addressed, chiffrement AES-256, workflow GitLab professionnel"
                                </p>
                                <div class="mt-4 flex flex-wrap gap-2">
                                    <span class="px-2 py-1 text-xs bg-blue-100 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400 rounded-full">"Rust"</span>
                                    <span class="px-2 py-1 text-xs bg-blue-100 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400 rounded-full">"Crypto"</span>
                                    <span class="px-2 py-1 text-xs bg-blue-100 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400 rounded-full">"CLI"</span>
                                </div>
                            </div>
                        </a>

                        // Comptoir
                        <a
                            href="/articles/comptoir"
                            class="group relative bg-theme-nav border border-zinc-200 dark:border-zinc-700 rounded-2xl p-8 hover:shadow-2xl hover:scale-105 transition-all duration-300 overflow-hidden"
                        >
                            <div class="absolute inset-0 bg-gradient-to-br from-purple-500/10 to-pink-500/10 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                            <div class="relative z-10">
                                <div class="mb-4">
                                    <span class="text-xs font-semibold text-purple-600 dark:text-purple-400 uppercase tracking-wide">"Alternance • ADULLACT"</span>
                                </div>
                                <h3 class="text-xl font-bold text-theme-primary mb-3">
                                    "Analyse du Comptoir du Libre"
                                </h3>
                                <p class="text-sm text-theme-secondary leading-relaxed">
                                    "Vérification automatisée de la disponibilité des sites web et dépôts des logiciels libres métiers"
                                </p>
                                <div class="mt-4 flex flex-wrap gap-2">
                                    <span class="px-2 py-1 text-xs bg-purple-100 dark:bg-purple-900/30 text-purple-600 dark:text-purple-400 rounded-full">"Rust"</span>
                                    <span class="px-2 py-1 text-xs bg-purple-100 dark:bg-purple-900/30 text-purple-600 dark:text-purple-400 rounded-full">"Async"</span>
                                    <span class="px-2 py-1 text-xs bg-purple-100 dark:bg-purple-900/30 text-purple-600 dark:text-purple-400 rounded-full">"SQLite"</span>
                                </div>
                            </div>
                        </a>
                    </div>
                </section>

                // Autres projets
                <section>
                    <h2 class="text-2xl font-bold text-theme-primary mb-6">
                        "Autres projets"
                    </h2>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                        // VoteOmatic
                        <a
                            href="/articles/voteomatic"
                            class="group relative bg-theme-nav border border-zinc-200 dark:border-zinc-700 rounded-2xl p-6 hover:shadow-xl hover:scale-105 transition-all duration-300 overflow-hidden"
                        >
                            <div class="absolute inset-0 bg-gradient-to-br from-amber-500/10 to-yellow-500/10 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                            <div class="relative z-10">
                                <div class="mb-3">
                                    <span class="block text-xs font-semibold text-amber-600 dark:text-amber-400 uppercase tracking-wide">"Java • Réseau"</span>
                                </div>
                                <h3 class="text-lg font-bold text-theme-primary mb-2">
                                    "VoteOmatic"
                                </h3>
                                <p class="text-xs text-theme-secondary leading-relaxed">
                                    "Service de sondage en réseau"
                                </p>
                            </div>
                        </a>

                        // Convertisseur
                        <a
                            href="/articles/convertisseur_rust"
                            class="group relative bg-theme-nav border border-zinc-200 dark:border-zinc-700 rounded-2xl p-6 hover:shadow-xl hover:scale-105 transition-all duration-300 overflow-hidden"
                        >
                            <div class="absolute inset-0 bg-gradient-to-br from-red-500/10 to-rose-500/10 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                            <div class="relative z-10">
                                <div class="mb-3">
                                    <span class="block text-xs font-semibold text-red-600 dark:text-red-400 uppercase tracking-wide">"Rust • CLI"</span>
                                </div>
                                <h3 class="text-lg font-bold text-theme-primary mb-2">
                                    "Convertisseur"
                                </h3>
                                <p class="text-xs text-theme-secondary leading-relaxed">
                                    "Conversion de bases numériques"
                                </p>
                            </div>
                        </a>

                        // Game of Life
                        <a
                            href="/articles/gol_java"
                            class="group relative bg-theme-nav border border-zinc-200 dark:border-zinc-700 rounded-2xl p-6 hover:shadow-xl hover:scale-105 transition-all duration-300 overflow-hidden"
                        >
                            <div class="absolute inset-0 bg-gradient-to-br from-teal-500/10 to-cyan-500/10 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                            <div class="relative z-10">
                                <div class="mb-3">
                                    <span class="block text-xs font-semibold text-teal-600 dark:text-teal-400 uppercase tracking-wide">"Java • Simulation"</span>
                                </div>
                                <h3 class="text-lg font-bold text-theme-primary mb-2">
                                    "Game of Life"
                                </h3>
                                <p class="text-xs text-theme-secondary leading-relaxed">
                                    "Jeu de la vie de Conway"
                                </p>
                            </div>
                        </a>
                    </div>
                </section>
            </main>
        </div>
    }
}
