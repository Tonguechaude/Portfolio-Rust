use leptos::prelude::*;

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col items-center justify-center px-6 py-12">
            <div class="max-w-4xl w-full">
                // Header section
                <div class="text-center mb-12">
                    <h1 class="text-5xl font-extrabold text-theme-primary mb-4 tracking-tight">
                        "Restons en contact"
                    </h1>
                    <p class="text-xl text-theme-secondary max-w-2xl mx-auto">
                        "Que ce soit pour contribuer a un projet, ou juste discuter tech, je suis toujours partant ! 🚀"
                    </p>
                </div>

                // Contact cards grid
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
                    // Email card
                    <a
                        href="mailto:evan.challias@tonguechaude.fr"
                        class="group relative bg-theme-nav border border-zinc-200 dark:border-zinc-700 rounded-2xl p-8 hover:shadow-2xl hover:scale-105 transition-all duration-300 overflow-hidden"
                    >
                        <div class="absolute inset-0 bg-gradient-to-br from-blue-500/5 to-purple-500/5 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                        <div class="relative z-10">
                            <div class="flex items-center gap-4 mb-4">
                                <div class="p-3 bg-blue-100 dark:bg-blue-900/30 rounded-xl">
                                    <svg class="w-8 h-8 text-blue-600 dark:text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/>
                                    </svg>
                                </div>
                                <div>
                                    <h3 class="text-2xl font-bold text-theme-primary">"Email"</h3>
                                    <p class="text-sm text-theme-secondary">"La méthode classique"</p>
                                </div>
                            </div>
                            <p class="text-theme-primary font-mono text-sm break-all">
                                "evan.challias@tonguechaude.fr"
                            </p>
                        </div>
                    </a>

                    // GitHub card
                    <a
                        href="https://github.com/Tonguechaude"
                        target="_blank"
                        class="group relative bg-theme-nav border border-zinc-200 dark:border-zinc-700 rounded-2xl p-8 hover:shadow-2xl hover:scale-105 transition-all duration-300 overflow-hidden"
                    >
                        <div class="absolute inset-0 bg-gradient-to-br from-gray-500/5 to-black/5 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                        <div class="relative z-10">
                            <div class="flex items-center gap-4 mb-4">
                                <div class="p-3 bg-gray-100 dark:bg-gray-900/30 rounded-xl">
                                    <svg class="w-8 h-8 text-gray-800 dark:text-gray-200" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/>
                                    </svg>
                                </div>
                                <div>
                                    <h3 class="text-2xl font-bold text-theme-primary">"GitHub"</h3>
                                    <p class="text-sm text-theme-secondary">"Mon code et mes projets"</p>
                                </div>
                            </div>
                            <p class="text-theme-primary font-mono text-sm">
                                "@Tonguechaude"
                            </p>
                        </div>
                    </a>

                    // LinkedIn card
                    <a
                        href="https://www.linkedin.com/in/evan-c-29805a276/"
                        target="_blank"
                        class="group relative bg-theme-nav border border-zinc-200 dark:border-zinc-700 rounded-2xl p-8 hover:shadow-2xl hover:scale-105 transition-all duration-300 overflow-hidden"
                    >
                        <div class="absolute inset-0 bg-gradient-to-br from-blue-600/5 to-blue-800/5 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                        <div class="relative z-10">
                            <div class="flex items-center gap-4 mb-4">
                                <div class="p-3 bg-blue-100 dark:bg-blue-900/30 rounded-xl">
                                    <svg class="w-8 h-8 text-blue-600 dark:text-blue-400" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z"/>
                                    </svg>
                                </div>
                                <div>
                                    <h3 class="text-2xl font-bold text-theme-primary">"LinkedIn"</h3>
                                    <p class="text-sm text-theme-secondary">"Mon parcours professionnel"</p>
                                </div>
                            </div>
                            <p class="text-theme-primary font-mono text-sm">
                                "Evan Tonguechaude"
                            </p>
                        </div>
                    </a>

                    // Mastodon card
                    <a
                        href="https://mastodon.social/@tonguechaude"
                        target="_blank"
                        class="group relative bg-theme-nav border border-zinc-200 dark:border-zinc-700 rounded-2xl p-8 hover:shadow-2xl hover:scale-105 transition-all duration-300 overflow-hidden"
                    >
                        <div class="absolute inset-0 bg-gradient-to-br from-purple-600/5 to-purple-800/5 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                        <div class="relative z-10">
                            <div class="flex items-center gap-4 mb-4">
                                <div class="p-3 bg-purple-100 dark:bg-purple-900/30 rounded-xl">
                                    <svg class="w-8 h-8 text-purple-600 dark:text-purple-400" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M23.268 5.313c-.35-2.578-2.617-4.61-5.304-5.004C17.51.242 15.792 0 11.813 0h-.03c-3.98 0-4.835.242-5.288.309C3.882.692 1.496 2.518.917 5.127.64 6.412.61 7.837.661 9.143c.074 1.874.088 3.745.26 5.611.118 1.24.325 2.47.62 3.68.55 2.237 2.777 4.098 4.96 4.857 2.336.792 4.849.923 7.256.38.265-.061.527-.132.786-.213.585-.184 1.27-.39 1.774-.753a.057.057 0 0 0 .023-.043v-1.809a.052.052 0 0 0-.02-.041.053.053 0 0 0-.046-.01 20.282 20.282 0 0 1-4.709.545c-2.73 0-3.463-1.284-3.674-1.818a5.593 5.593 0 0 1-.319-1.433.053.053 0 0 1 .066-.054c1.517.363 3.072.546 4.632.546.376 0 .75 0 1.125-.01 1.57-.044 3.224-.124 4.768-.422.038-.008.077-.015.11-.024 2.435-.464 4.753-1.92 4.989-5.604.008-.145.03-1.52.03-1.67.002-.512.167-3.63-.024-5.545zm-3.748 9.195h-2.561V8.29c0-1.309-.55-1.976-1.67-1.976-1.23 0-1.846.79-1.846 2.35v3.403h-2.546V8.663c0-1.56-.617-2.35-1.848-2.35-1.112 0-1.668.668-1.67 1.977v6.218H4.822V8.102c0-1.31.337-2.35 1.011-3.12.696-.77 1.608-1.164 2.74-1.164 1.311 0 2.302.5 2.962 1.498l.638 1.06.638-1.06c.66-.999 1.65-1.498 2.96-1.498 1.13 0 2.043.395 2.74 1.164.675.77 1.012 1.81 1.012 3.12z"/>
                                    </svg>
                                </div>
                                <div>
                                    <h3 class="text-2xl font-bold text-theme-primary">"Mastodon"</h3>
                                    <p class="text-sm text-theme-secondary">"Le fediverse m'appelle"</p>
                                </div>
                            </div>
                            <p class="text-theme-primary font-mono text-sm">
                                "@tonguechaude@mastodon.social"
                            </p>
                        </div>
                    </a>
                </div>

                // Footer note
                <div class="text-center mt-12">
                    <p class="text-theme-secondary text-sm">
                        "💡 Astuce : Les meilleurs projets commencent par une simple conversation"
                    </p>
                </div>
            </div>
        </div>
    }
}
