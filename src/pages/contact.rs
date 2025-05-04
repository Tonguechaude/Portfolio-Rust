use crate::components::social::SocialLinks;
use leptos::prelude::*;

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <section class="max-w-3xl mx-auto p-8 mt-10 bg-white dark:bg-zinc-900 shadow-xl rounded-2xl transition-all duration-300">
            <h1 class="text-4xl font-extrabold text-zinc-800 dark:text-zinc-100 mb-4 tracking-tight">"📬 Contact"</h1>
            <p class="text-zinc-600 dark:text-zinc-300 text-lg mb-6">
                "Tu peux me retrouver ou me contacter sur ces plateformes. Je réponds vite, promis. 👀"
            </p>

            <div class="border-t border-zinc-200 dark:border-zinc-700 pt-6">
                <SocialLinks />
            </div>
        </section>
    }
}
