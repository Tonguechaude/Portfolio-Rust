use leptos::prelude::*;

/// 404 Not Found Page
#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! { <h1>"Uh oh!" <br /> "We couldn't find that page!"</h1> }
}
