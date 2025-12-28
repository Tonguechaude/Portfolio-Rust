use crate::components::github_contributions::GitHubContributions;
use leptos::prelude::*;

#[component]
pub fn ContributionsPage() -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col">
            <main class="flex-grow max-w-6xl mx-auto px-6 py-12">
                <section class="mb-12">
                    <h1 class="text-4xl font-bold text-theme-primary mb-4">
                        "Mes contributions au libre"
                    </h1>
                    <p class="text-lg text-theme-secondary max-w-3xl">
                        "Voici l'ensemble de mes contributions aux projets open source. Ces pull requests reflètent mon engagement envers le développement collaboratif et ma passion pour l'amélioration continue des outils que nous utilisons tous."
                    </p>
                </section>

                <section>
                    <GitHubContributions
                        username="tonguechaude".to_string()
                    />
                </section>
            </main>
        </div>
    }
}
