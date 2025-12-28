use crate::data::github::{GitHubSearchResponse, SimplifiedPR};

pub async fn fetch_pull_requests(
    username: &str,
    excluded_repos: &[&str],
    excluded_titles: &[&str],
) -> Result<Vec<SimplifiedPR>, String> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.github.com/search/issues?q=type:pr+author:{}&sort=created&order=desc&per_page=100",
        username
    );

    let request_builder = client
        .get(&url)
        .header("User-Agent", "Portfolio-Rust/1.0")
        .header("Accept", "application/vnd.github.v3+json");

    let response = request_builder
        .send()
        .await
        .map_err(|e| format!("Erreur de requête: {}", e))?;

    let status = response.status();
    let text = response.text().await.unwrap_or_default();

    if !status.is_success() {
        return Err(format!("GitHub API error ({}): {}", status, text));
    }

    let search_response: GitHubSearchResponse = serde_json::from_str(&text)
        .map_err(|e| format!("Erreur JSON: {}", e))?;

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

    Ok(pr_events)
}

pub async fn fetch_recent_merged_prs(
    username: &str,
    excluded_repos: &[&str],
    limit: usize,
) -> Result<Vec<SimplifiedPR>, String> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.github.com/search/issues?q=type:pr+author:{}&sort=created&order=desc&per_page=100",
        username
    );

    let request_builder = client
        .get(&url)
        .header("User-Agent", "Portfolio-Rust/1.0")
        .header("Accept", "application/vnd.github.v3+json");

    let response = request_builder
        .send()
        .await
        .map_err(|e| format!("Erreur de requête: {}", e))?;

    let status = response.status();
    let text = response.text().await.unwrap_or_default();

    if !status.is_success() {
        return Err(format!("GitHub API error ({}): {}", status, text));
    }

    let search_response: GitHubSearchResponse = serde_json::from_str(&text)
        .map_err(|e| format!("Erreur JSON: {}", e))?;

    let pr_events: Vec<SimplifiedPR> = search_response
        .items
        .into_iter()
        .filter_map(|pr| {
            // Only closed PR (= generally merged)
            if pr.state.to_lowercase() != "closed" {
                return None;
            }

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
        .take(limit)
        .collect();

    Ok(pr_events)
}
