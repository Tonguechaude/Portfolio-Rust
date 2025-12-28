use components::copyright::Copyright;
use components::navbar::Navbar;
use leptos_router::{components::*, path};
use leptos::prelude::*;
use leptos_meta::*;

// Modules
mod components;
mod data;
mod pages;
mod utils;

// Top-Level pages
use crate::pages::apprentissage::comptoir::ComptoirPage;
use crate::pages::apprentissage::sae_reseaux::SaePage;
use crate::pages::apprentissage::service_desk::TicketingPage;
use crate::pages::apprentissages::ApprentissagePage;
use crate::pages::article::comptoir::ComptoirArticlePage;
use crate::pages::article::convertisseur_rust::ConvertisseurRustArticlePage;
use crate::pages::article::gol_java::GolJavaArticlePage;
use crate::pages::article::voteomatic::VoteOmaticArticlePage;
use crate::pages::articles::ArticlesPage;
use crate::pages::contributions::ContributionsPage;
use crate::pages::not_found::NotFoundPage;
use crate::pages::projects::ProjectPage;
use crate::pages::contact::ContactPage;
use crate::pages::home::HomePage;

// An app router which renders the homepage and handles 404's

#[component]
fn DefaultLayout() -> impl IntoView {
    view! {
        <>
            <Navbar />
            <main class="flex-grow">
                <Outlet />
            </main>
            <Copyright year=2025 />
        </>
    }
}

#[component]
fn BareLayout() -> impl IntoView {
    view! {
        <Outlet />
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" />
        <Title text="Chez Tonguechaude" />
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <div class="min-h-screen flex flex-col bg-theme-primary">
            <Router>
            <Routes fallback=|| view! { <NotFoundPage /> }>
                    <ParentRoute path=path!("") view=DefaultLayout>
                        <Route path=path!("/") view=HomePage />
                        <Route path=path!("/contact") view=ContactPage />
                        <Route path=path!("/projects") view=ProjectPage />
                        <Route path=path!("/articles") view=ArticlesPage />
                        <Route path=path!("/apprentissages/sae") view=SaePage />
                        <Route path=path!("/contributions") view=ContributionsPage />
                        <Route path=path!("/apprentissages") view=ApprentissagePage />
                        <Route path=path!("/apprentissages/comptoir") view=ComptoirPage />
                        <Route path=path!("/articles/gol_java") view=GolJavaArticlePage />
                        <Route path=path!("/articles/comptoir") view=ComptoirArticlePage />
                        <Route path=path!("/apprentissages/ticketing") view=TicketingPage />
                        <Route path=path!("/articles/voteomatic") view=VoteOmaticArticlePage />
                        <Route path=path!("/articles/convertisseur_rust") view=ConvertisseurRustArticlePage />
                    </ParentRoute>

                    <ParentRoute path=path!("/*any") view=BareLayout>
                        <Route path=path!("/*any") view=NotFoundPage />
                    </ParentRoute>
                </Routes>
            </Router>
        </div>
    }
}
