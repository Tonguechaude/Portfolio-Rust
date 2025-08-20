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
        let pull_requests = pull_requests;
        let loading = loading;
        let error = error;
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
                                                .last()
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

    let get_state_badge = |state: &str| -> &'static str {
        match state {
            "MERGED" => "bg-green-100 text-green-800",
            "OPEN" => "bg-blue-100 text-blue-800",
            "CLOSED" => "bg-red-100 text-red-800",
            _ => "bg-gray-100 text-gray-800",
        }
    };

    // <!-- Badge Open style GitHub -->
    // <span class="inline-flex items-center gap-1 px-2 py-1 rounded-full text-xs font-medium text-white" style="background-color: #1a7f37;">
    //     <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
    //         <path d="M1.5 3.25a2.25 2.25 0 1 1 3 2.122v5.256a2.251 2.251 0 1 1-1.5 0V5.372A2.25 2.25 0 0 1 1.5 3.25Zm5.677-.177L9.573.677A.25.25 0 0 1 10 .854V2.5h1A2.5 2.5 0 0 1 13.5 5v5.628a2.251 2.251 0 1 1-1.5 0V5a1 1 0 0 0-1-1h-1v1.646a.25.25 0 0 1-.427.177L7.177 3.427a.25.25 0 0 1 0-.354ZM3.75 2.5a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5Zm0 9.5a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5Zm8.25.75a.75.75 0 1 0 1.5 0 .75.75 0 0 0-1.5 0Z"/>
    //     </svg>
    //     Open
    // </span>

    view! {
        <div class="w-full">
            <div class="flex justify-between items-center mb-6">
                <h3 class="text-2xl font-bold text-zinc-800">
                    "🚀 Mes contributions GitHub"
                </h3>
                <button
                    on:click={
                        let username = username.clone();
                        let pull_requests = pull_requests;
                        let loading = loading;
                        let error = error;
                        let excluded_repos = excluded_repos.clone();
                        let excluded_titles = excluded_titles.clone();

                        move |_| {
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
                                                            let repo_name = repo_url.split('/').last().unwrap_or("unknown").to_string();

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
                    class="px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors"
                    disabled=move || loading.get()
                >
                    {move || if loading.get() { "🔄 Chargement..." } else { "🔄 Actualiser" }}
                </button>
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
                                        let state_badge = get_state_badge(&pr.state);
                                        let created_date = format_date(&pr.created_at);
                                        let merged_date = pr.merged_at.as_ref().map(|d| format_date(d));

                                        view! {
                                            <div class="bg-white border border-zinc-200 rounded-lg p-6 hover:shadow-md transition-shadow">
                                                <div class="flex items-start justify-between mb-3">
                                                    <div class="flex items-center gap-2">
                                                        <span class={format!("px-2 py-1 rounded-full text-xs font-medium {}", state_badge)}>
                                                            {pr.state}
                                                        </span>
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
