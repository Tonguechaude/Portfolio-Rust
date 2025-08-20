use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct GitHubSearchResponse {
    pub items: Vec<SearchPullRequest>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SearchPullRequest {
    pub title: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub state: String,
    #[serde(rename = "merged_at")]
    pub merged_at: Option<String>,
    pub repository_url: String,
}

#[derive(Debug, Clone)]
pub struct SimplifiedPR {
    pub title: String,
    pub url: String,
    pub state: String,
    pub created_at: String,
    pub merged_at: Option<String>,
    pub repo_name: String,
    pub repo_url: String,
}