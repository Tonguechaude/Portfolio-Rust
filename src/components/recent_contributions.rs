use crate::data::github::{GitHubSearchResponse, SimplifiedPR};
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn RecentContributions(username: String, token: Option<String>) -> impl IntoView {
    let excluded_repos: Vec<&str> = vec!["pendu.rs", "puppet-epel"];
    let excluded_titles: Vec<&str> = vec![];

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

                                            // only closed PR (= generally merged)
                                            if pr.state.to_lowercase() != "closed" {
                                                return None;
                                            }

                                            Some(SimplifiedPR {
                                                title: pr.title,
                                                url: pr.html_url,
                                                state: "MERGED".to_string(), // Assume closed = merged
                                                created_at: pr.created_at,
                                                merged_at: pr.merged_at,
                                                repo_name,
                                                repo_url,
                                            })
                                        })
                                        .take(3) // limit 3 PR
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
            {move || {
                if loading.get() {
                    view! {
                        <div class="flex justify-center items-center py-6">
                            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-indigo-600"></div>
                        </div>
                    }.into_any()
                } else if let Some(error_msg) = error.get() {
                    view! {
                        <div class="bg-red-50 border border-red-200 rounded-lg p-3 text-sm">
                            <p class="text-red-600">{error_msg}</p>
                        </div>
                    }.into_any()
                } else {
                    let prs = pull_requests.get();
                    if prs.is_empty() {
                        view! {
                            <div class="text-center py-6 text-zinc-500">
                                <p class="text-sm">"Aucune contribution récente"</p>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="space-y-3">
                                <div class="flex justify-between items-center mb-3">
                                    <h3 class="text-lg font-semibold text-zinc-800">
                                        "Contributions récentes au libre"
                                    </h3>
                                    <a href="/contributions" class="text-sm text-indigo-600 hover:text-indigo-800 hover:underline">
                                        "Voir toutes →"
                                    </a>
                                </div>

                                {prs.into_iter().map(|pr| {
                                    let merged_date = pr.merged_at.as_ref().map(|d| format_date(d));

                                    view! {
                                        <div class="bg-white border border-zinc-200 rounded-lg p-4 hover:shadow-sm transition-shadow">
                                            <div class="flex items-start justify-between mb-2">
                                                <div class="flex items-center gap-2">
                                                    <span class="inline-flex items-center gap-1 px-2 py-1 rounded-full text-xs font-medium text-white" style="background-color: #8250df;">
                                                        <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
                                                            <path d="M5.45 5.154A4.25 4.25 0 0 0 9.25 7.5h1.378a2.251 2.251 0 1 1 0 1.5H9.25A5.734 5.734 0 0 1 5 7.123v3.505a2.25 2.25 0 1 1-1.5 0V5.372a2.25 2.25 0 1 1 1.95-.218ZM4.25 13.5a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5Zm8.5-4.5a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5ZM4.25 4a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5Z"/>
                                                        </svg>
                                                        Merged
                                                    </span>
                                                </div>
                                                <div class="text-xs text-zinc-500">
                                                    {merged_date.unwrap_or_default()}
                                                </div>
                                            </div>

                                            <h4 class="font-medium text-zinc-800 mb-1 text-sm leading-tight">
                                                <a href={pr.url.clone()} target="_blank" class="hover:text-indigo-600 transition-colors">
                                                    {pr.title}
                                                </a>
                                            </h4>

                                            <div class="flex items-center text-xs text-zinc-600">
                                                <span class="font-medium">{"📁"}</span>
                                                <a href={pr.repo_url.clone()} target="_blank" class="ml-1 hover:text-indigo-600 transition-colors">
                                                    {pr.repo_name.clone()}
                                                </a>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_any()
                    }
                }
            }}
        </div>
    }
}
