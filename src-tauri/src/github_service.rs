use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubResponse {
    pub total_count: u64,
    pub items: Vec<PullRequestItem>,
}

fn default_repo_url() -> String {
    "".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestItem {
    pub id: u64,
    pub title: String,
    #[serde(default = "default_repo_url")]
    pub repository_url: String,
    pub user: PullRequestItemUser,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
    pub pull_request: PullRequestItemPullRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestItemPullRequest {
    pub html_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestItemUser {
    pub login: String,
    pub avatar_url: String,
}

pub struct GithubClient {
    client: Client,
    token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub login: String,
}

impl GithubClient {
    pub fn new(github_token: String) -> GithubClient {
        let client = Client::new();
        GithubClient {
            client,
            token: github_token,
        }
    }

    async fn make_request(&self, url: String) -> Result<reqwest::Response, reqwest::Error> {
        let resp = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header("User-Agent", "rust-be-preview")
            .send() // This actually sends the request
            .await?; // Handle potential errors

        Ok(resp)
    }

    pub async fn get_user(&self) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("https://api.github.com/user");
        let resp = self.make_request(url).await?;
        let user: User = resp.json::<User>().await?;
        Ok(user)
    }

    pub async fn search_pull_requests(
        &self,
        query: String,
    ) -> Result<GithubResponse, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!(
            "https://api.github.com/search/issues?sort=updated&order=desc&q={}",
            query
        );

        let resp = self.make_request(url).await?;

        let github_response: GithubResponse = resp.json::<GithubResponse>().await?;

        Ok(github_response)
    }
}
