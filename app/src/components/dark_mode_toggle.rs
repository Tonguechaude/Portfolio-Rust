use leptos::prelude::*;

#[component]
pub fn DarkModeToggle() -> impl IntoView {
    let is_dark = RwSignal::new(false);

    let toggle_theme = move |_| {
        let new_dark_state = !is_dark.get();
        is_dark.set(new_dark_state);

        // Application du thème sur l'élément HTML
        let document = document();
        if let Some(html) = document.document_element() {
            let theme = if new_dark_state { "dark" } else { "light" };
            let _ = html.set_attribute("data-theme", theme);
        }
    };

    view! {
        <button
            on:click=toggle_theme
            class="p-2 rounded-lg bg-theme-nav text-theme-secondary hover:text-theme-accent transition-colors"
            title=move || if is_dark.get() { "Activer le mode clair" } else { "Activer le mode sombre" }
        >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d=move || if is_dark.get() {
                        "M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
                    } else {
                        "M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"
                    }
                />
            </svg>
        </button>
    }
}
