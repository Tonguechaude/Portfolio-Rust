use crate::components::social::SocialLinks;
use leptos::prelude::*;

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <section class="max-w-3xl mx-auto p-8 mt-10 bg-white shadow-xl rounded-2xl transition-all duration-300">
            <h1 class="text-4xl font-extrabold text-zinc-800 mb-4 tracking-tight">"📬 Contact"</h1>
            <p class="text-zinc-600 text-lg mb-6">
                "Tu peux me retrouver ou me contacter sur ces plateformes. Je réponds vite, promis. 👀"
            </p>

            <div class="border-t border-zinc-200 pt-6">
                <SocialLinks />
            </div>
        </section>
    }
}
