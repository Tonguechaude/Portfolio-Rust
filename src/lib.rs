use components::copyright::Copyright;
use components::navbar::Navbar;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

// Modules
mod components;
mod data;
mod pages;

// Top-Level pages
use crate::pages::contact::ContactPage;
use crate::pages::home::HomePage;
use crate::pages::projects::ProjectPage;

// An app router which renders the homepage and handles 404's

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />
        <Title text="Chez Tonguechaude" />
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <div class="min-h-screen flex flex-col">
            <Router>
                <Navbar />
                <main class="flex-grow">
                    <Routes fallback=|| view! { NotFound }>
                        <Route path=path!("/") view=HomePage />
                        <Route path=path!("/contact") view=ContactPage />
                        <Route path=path!("/projects") view=ProjectPage />
                    </Routes>
                </main>
            </Router>
            <Copyright year=2025 />
        </div>
    }
}
