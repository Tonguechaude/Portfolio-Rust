use crate::components::dark_mode_toggle::DarkModeToggle;
use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    let is_menu_open = RwSignal::new(false);

    view! {
        <nav class="bg-theme-nav shadow-theme py-4 px-6">
            <div class="container mx-auto flex justify-between items-center">
                <div class="text-2xl font-bold text-theme-primary">
                    <a href="/" class="hover:text-theme-accent transition">"Mon Atelier"</a>
                </div>

                <div class="hidden md:flex items-center space-x-6">
                    <ul class="flex space-x-6 text-theme-secondary">
                        <li><a href="/" class="hover:text-theme-accent transition">Accueil</a></li>
                        <li><a href="/projects" class="hover:text-theme-accent transition">Projets</a></li>
                        <li><a href="/articles" class="hover:text-theme-accent transition">Articles</a></li>
                        <li><a href="/apprentissages" class="hover:text-theme-accent transition">Apprentissages</a></li>
                        <li><a href="/contributions" class="hover:text-theme-accent transition">Contributions au libre</a></li>
                        <li><a href="/contact" class="hover:text-theme-accent transition">Contact</a></li>
                    </ul>
                    <DarkModeToggle />
                </div>

                <div class="md:hidden flex items-center space-x-2">
                    <DarkModeToggle />
                    <button
                        class="text-theme-secondary"
                        on:click=move |_| is_menu_open.update(|open| *open = !*open)
                    >
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M4 6h16M4 12h16M4 18h16" />
                        </svg>
                    </button>
                </div>
            </div>

            <div class=move || {
                if is_menu_open.get() {
                    "block md:hidden mt-2"
                } else {
                    "hidden md:hidden mt-2"
                }
            }>
                <ul class="flex flex-col space-y-4 text-theme-secondary">
                    <li>
                        <a
                            href="/"
                            class="hover:text-theme-accent transition"
                            on:click=move |_| is_menu_open.set(false)
                        >Accueil</a>
                    </li>
                    <li>
                        <a
                            href="/projects"
                            class="hover:text-theme-accent transition"
                            on:click=move |_| is_menu_open.set(false)
                        >Projets</a>
                    </li>
                     <li>
                         <a
                             href="/articles"
                             class="hover:text-theme-accent transition"
                             on:click=move |_| is_menu_open.set(false)
                         >Articles</a>
                     </li>
                     <li>
                         <a
                             href="/apprentissages"
                             class="hover:text-theme-accent transition"
                             on:click=move |_| is_menu_open.set(false)
                         >Apprentissages</a>
                     </li>
                    <li>
                        <a
                            href="/contributions"
                            class="hover:text-theme-accent transition"
                            on:click=move |_| is_menu_open.set(false)
                        >Contributions au libre</a>
                    </li>
                    <li>
                        <a
                            href="/contact"
                            class="hover:text-theme-accent transition"
                            on:click=move |_| is_menu_open.set(false)
                        >Contact</a>
                    </li>

                </ul>
            </div>
        </nav>
    }
}
