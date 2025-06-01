use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    let is_menu_open = RwSignal::new(false);

    view! {
        <nav class="bg-white shadow-md py-4 px-6">
            <div class="container mx-auto flex justify-between items-center">
                <div class="text-2xl font-bold text-gray-800">
                    <a href="/" class="hover:text-blue-600 transition">Atelier de Tonguechaude</a>
                </div>

                <ul class="hidden md:flex space-x-6 text-gray-700">
                    <li><a href="/" class="hover:text-blue-600 transition">Accueil</a></li>
                    <li><a href="/projects" class="hover:text-blue-600 transition">Projets</a></li>
                    <li><a href="/articles" class="hover:text-blue-600 transition">Articles</a></li>
                    <li><a href="/apprentissages" class="hover:text-blue-600 transition">Apprentissages</a></li>
                    <li><a href="/contact" class="hover:text-blue-600 transition">Contact</a></li>
                </ul>

                <button
                    class="md:hidden"
                    on:click=move |_| is_menu_open.update(|open| *open = !*open)
                >
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M4 6h16M4 12h16M4 18h16" />
                    </svg>
                </button>
            </div>

            <div class=move || {
                if is_menu_open.get() {
                    "block md:hidden mt-2"
                } else {
                    "hidden md:hidden mt-2"
                }
            }>
                <ul class="flex flex-col space-y-4 text-gray-700">
                    <li>
                        <a
                            href="/"
                            class="hover:text-blue-600 transition"
                            on:click=move |_| is_menu_open.set(false)
                        >Accueil</a>
                    </li>
                    <li>
                        <a
                            href="/projects"
                            class="hover:text-blue-600 transition"
                            on:click=move |_| is_menu_open.set(false)
                        >Projets</a>
                    </li>
                    <li>
                        <a
                            href="/articles"
                            class="hover:text-blue-600 transition"
                            on:click=move |_| is_menu_open.set(false)
                        >Articles</a>
                    </li>
                    <li>
                        <a
                            href="/apprentissages"
                            class="hover:text-blue-600 transition"
                            on:click=move |_| is_menu_open.set(false)
                        >Apprentissages</a>
                    </li>
                    <li>
                        <a
                            href="/contact"
                            class="hover:text-blue-600 transition"
                            on:click=move |_| is_menu_open.set(false)
                        >Contact</a>
                    </li>

                </ul>
            </div>
        </nav>
    }
}
