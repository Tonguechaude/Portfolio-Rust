use crate::data::github::{GitHubSearchResponse, SimplifiedPR};
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn GitHubContributions(username: String, token: Option<String>) -> impl IntoView {
    let excluded_repos: Vec<&str> = vec!["pendu.rs", "puppet-epel"];
    let excluded_titles: Vec<&str> = vec![
        // Exemple -> "Draft: Prototype",
    ];
    let pull_requests = RwSignal::new(Vec::<SimplifiedPR>::new());
    let loading = RwSignal::new(true);
    let error = RwSignal::new(None::<String>);

    Effect::new({
        let username = username.clone();
        let token = token.clone();
        let excluded_repos = excluded_repos.clone();
        let excluded_titles = excluded_titles.clone();

        move |_| {
            spawn_local({
                let username = username.clone();
                let _token = token.clone();
                let pull_requests = pull_requests;
                let loading = loading;
                let error = error;
                let excluded_repos = excluded_repos.clone();
                let excluded_titles = excluded_titles.clone();

                async move {
                    loading.set(true);
                    error.set(None);

                    let client = reqwest::Client::new();
                    let url = format!(
                        "https://api.github.com/search/issues?q=type:pr+author:{}&sort=created&order=desc&per_page=100",
                        username
                    );

                    let request_builder = client
                        .get(&url)
                        .header("User-Agent", "Portfolio-Rust/1.0")
                        .header("Accept", "application/vnd.github.v3+json");

                    match request_builder.send().await {
                        Ok(response) => {
                            let status = response.status();
                            let text = response.text().await.unwrap_or_default();

                            if !status.is_success() {
                                error.set(Some(format!("GitHub API error ({}): {}", status, text)));
                                return;
                            }

                            match serde_json::from_str::<GitHubSearchResponse>(&text) {
                                Ok(search_response) => {
                                    let pr_events: Vec<SimplifiedPR> = search_response
                                        .items
                                        .into_iter()
                                        .filter_map(|pr| {
                                            let repo_url = pr
                                                .repository_url
                                                .replace("api.", "")
                                                .replace("repos/", "");
                                            let repo_name = repo_url
                                                .split('/')
                                                .next_back()
                                                .unwrap_or("unknown")
                                                .to_string();

                                            if excluded_repos.contains(&repo_name.as_str()) {
                                                return None;
                                            }

                                            if excluded_titles
                                                .iter()
                                                .any(|excluded| pr.title.contains(excluded))
                                            {
                                                return None;
                                            }

                                            Some(SimplifiedPR {
                                                title: pr.title,
                                                url: pr.html_url,
                                                state: if pr.state.to_lowercase() == "closed" {
                                                    "MERGED".to_string()
                                                } else {
                                                    pr.state.to_uppercase()
                                                },
                                                created_at: pr.created_at,
                                                merged_at: pr.merged_at,
                                                repo_name,
                                                repo_url,
                                            })
                                        })
                                        .collect();

                                    pull_requests.set(pr_events);
                                }
                                Err(e) => {
                                    error.set(Some(format!(
                                        "Erreur de parsing JSON: {} - Réponse: {}",
                                        e,
                                        text.chars().take(200).collect::<String>()
                                    )));
                                }
                            }
                        }
                        Err(e) => {
                            error.set(Some(format!("Erreur de requête: {}", e)));
                        }
                    }

                    loading.set(false);
                }
            });
        }
    });

    let format_date = |date_str: &str| -> String {
        match date_str.split('T').next() {
            Some(date) => date.to_string(),
            None => date_str.to_string(),
        }
    };

    view! {
        <div class="w-full">
            <div class="flex justify-between items-center mb-6">
                <h3 class="text-2xl font-bold text-zinc-800 flex items-center gap-2">
                    <svg width="24" height="24" viewBox="0 0 16 16" fill="currentColor" class="text-zinc-700">
                        <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0 0 16 8c0-4.42-3.58-8-8-8z"/>
                    </svg>
                    "Mes contributions GitHub"
                </h3>
                <a
                    href="#"
                    on:click={
                        let username = username.clone();
                        let excluded_repos = excluded_repos.clone();
                        let excluded_titles = excluded_titles.clone();

                        move |ev| {
                            ev.prevent_default();
                            spawn_local({
                                let username = username.clone();
                                let pull_requests = pull_requests;
                                let loading = loading;
                                let error = error;
                                let excluded_repos = excluded_repos.clone();
                                let excluded_titles = excluded_titles.clone();

                                async move {
                                    loading.set(true);
                                    error.set(None);

                                    let client = reqwest::Client::new();
                                    let url = format!("https://api.github.com/search/issues?q=type:pr+author:{}&sort=created&order=desc&per_page=100", username);

                                    let request_builder = client
                                        .get(&url)
                                        .header("User-Agent", "Portfolio-Rust/1.0")
                                        .header("Accept", "application/vnd.github.v3+json");

                                    match request_builder.send().await {
                                        Ok(response) => {
                                            let status = response.status();
                                            let text = response.text().await.unwrap_or_default();

                                            if !status.is_success() {
                                                error.set(Some(format!("GitHub API error ({}): {}", status, text)));
                                                return;
                                            }

                                            match serde_json::from_str::<GitHubSearchResponse>(&text) {
                                                Ok(search_response) => {
                                                    let pr_events: Vec<SimplifiedPR> = search_response.items
                                                        .into_iter()
                                                        .filter_map(|pr| {
                                                            let repo_url = pr.repository_url.replace("api.", "").replace("repos/", "");
                                                            let repo_name = repo_url.split('/').next_back().unwrap_or("unknown").to_string();

                                                            if excluded_repos.contains(&repo_name.as_str()) {
                                                                return None;
                                                            }

                                                            if excluded_titles.iter().any(|excluded| pr.title.contains(excluded)) {
                                                                return None;
                                                            }

                                                            Some(SimplifiedPR {
                                                                title: pr.title,
                                                                url: pr.html_url,
                                                                state: if pr.state.to_lowercase() == "closed" { "MERGED".to_string() } else { pr.state.to_uppercase() },
                                                                created_at: pr.created_at,
                                                                merged_at: pr.merged_at,
                                                                repo_name,
                                                                repo_url,
                                                            })
                                                        })
                                                        .collect();

                                                    pull_requests.set(pr_events);
                                                }
                                                Err(e) => {
                                                    error.set(Some(format!("Erreur de parsing JSON: {} - Réponse: {}", e, text.chars().take(200).collect::<String>())));
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            error.set(Some(format!("Erreur de requête: {}", e)));
                                        }
                                    }

                                    loading.set(false);
                                }
                            });
                        }
                    }
                    class="text-indigo-600 hover:text-indigo-800 hover:underline text-sm transition-colors"
                >
                    {move || if loading.get() { "Actualisation..." } else { "Actualiser" }}
                </a>
            </div>

            {move || {
                if loading.get() {
                    view! {
                        <div class="flex justify-center items-center py-12">
                            <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-indigo-600"></div>
                        </div>
                    }.into_any()
                } else if let Some(error_msg) = error.get() {
                    view! {
                        <div class="bg-red-50 border border-red-200 rounded-lg p-4 mb-6">
                            <div class="flex items-center">
                                <div class="text-red-600 mr-3">"❌"</div>
                                <div>
                                    <h4 class="text-red-800 font-medium">"Erreur de chargement"</h4>
                                    <p class="text-red-600 text-sm mt-1">{error_msg}</p>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    let prs = pull_requests.get();
                    if prs.is_empty() {
                        view! {
                            <div class="text-center py-12 text-zinc-500">
                                <p>"Aucune pull request trouvée pour ce compte."</p>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="space-y-4">
                                <div class="text-sm text-zinc-600 mb-4">
                                    {format!("{} pull requests trouvées (100 max)", prs.len())}
                                </div>
                                <div class="grid gap-4">
                                    {prs.into_iter().map(|pr| {
                                        let created_date = format_date(&pr.created_at);
                                        let merged_date = pr.merged_at.as_ref().map(|d| format_date(d));

                                        view! {
                                            <div class="bg-white border border-zinc-200 rounded-lg p-6 hover:shadow-md transition-shadow">
                                                <div class="flex items-start justify-between mb-3">
                                                    <div class="flex items-center gap-2">
                                                        {match pr.state.as_str() {
                                                            "MERGED" => view! {
                                                                <span class="inline-flex items-center gap-1 px-2 py-1 rounded-full text-xs font-medium text-white" style="background-color: #8250df;">
                                                                    <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
                                                                        <path d="M5.45 5.154A4.25 4.25 0 0 0 9.25 7.5h1.378a2.251 2.251 0 1 1 0 1.5H9.25A5.734 5.734 0 0 1 5 7.123v3.505a2.25 2.25 0 1 1-1.5 0V5.372a2.25 2.25 0 1 1 1.95-.218ZM4.25 13.5a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5Zm8.5-4.5a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5ZM4.25 4a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5Z"/>
                                                                    </svg>
                                                                    "Merged"
                                                                </span>
                                                            }.into_any(),
                                                            "OPEN" => view! {
                                                                <span class="inline-flex items-center gap-1 px-2 py-1 rounded-full text-xs font-medium text-white" style="background-color: #1a7f37;">
                                                                    <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
                                                                        <path d="M1.5 3.25a2.25 2.25 0 1 1 3 2.122v5.256a2.251 2.251 0 1 1-1.5 0V5.372A2.25 2.25 0 0 1 1.5 3.25Zm5.677-.177L9.573.677A.25.25 0 0 1 10 .854V2.5h1A2.5 2.5 0 0 1 13.5 5v5.628a2.251 2.251 0 1 1-1.5 0V5a1 1 0 0 0-1-1h-1v1.646a.25.25 0 0 1-.427.177L7.177 3.427a.25.25 0 0 1 0-.354ZM3.75 2.5a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5Zm0 9.5a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5Zm8.25.75a.75.75 0 1 0 1.5 0 .75.75 0 0 0-1.5 0Z"/>
                                                                    </svg>
                                                                    "Open"
                                                                </span>
                                                            }.into_any(),
                                                            _ => view! {
                                                                <span class="px-2 py-1 rounded-full text-xs font-medium bg-gray-100 text-gray-800">
                                                                    {pr.state.clone()}
                                                                </span>
                                                            }.into_any(),
                                                        }}
                                                    </div>
                                                    <div class="text-sm text-zinc-500">
                                                        {created_date}
                                                    </div>
                                                </div>

                                                <h4 class="font-semibold text-zinc-800 mb-2 leading-tight">
                                                    <a href={pr.url.clone()} target="_blank" class="hover:text-indigo-600 transition-colors">
                                                        {pr.title}
                                                    </a>
                                                </h4>

                                                <div class="flex items-center justify-between text-sm text-zinc-600">
                                                    <div class="flex items-center gap-2">
                                                        <span class="font-medium">{"📁"}</span>
                                                        <a href={pr.repo_url.clone()} target="_blank" class="hover:text-indigo-600 transition-colors">
                                                            {pr.repo_name.clone()}
                                                        </a>
                                                    </div>

                                                    <div class="flex items-center gap-4">
                                                        {merged_date.map(|date| view! {
                                                            <span class="text-zinc-500">
                                                                "Merged: " {date}
                                                            </span>
                                                        })}
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </div>
                        }.into_any()
                    }
                }
            }}
        </div>
    }
}
