use leptos::prelude::*;

/// 404 Not Found Page
#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="bg-[url('/img/boomy.png')] bg-cover bg-center min-h-screen w-full flex items-center justify-center text-white text-center px-4">
            <h1 class="text-4xl font-bold">
                "Uh oh!" <br /> "We couldn't find that page!"
            </h1>
        </div>
    }
}
