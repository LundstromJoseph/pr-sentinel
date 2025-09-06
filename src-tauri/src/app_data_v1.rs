use std::collections::HashMap;

use fractional_index::FractionalIndex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn default_version() -> u8 {
    1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfigV1 {
    #[serde(default = "default_version")]
    pub version: u8,
    pub github_token: Option<String>,
    pub filters: Vec<GithubFilterV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppDataV1 {
    #[serde(default = "default_version")]
    pub version: u8,
    pub pull_requests: HashMap<Uuid, PullRequestsDataV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestsDataV1 {
    pub last_updated: u64,
    pub pull_requests: Vec<PullRequestItemV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubFilterV1 {
    pub id: Uuid,
    pub query: String,
    pub notify: bool,
    pub name: String,
    pub fractional_index: FractionalIndex,
}

fn default_repo_url() -> String {
    "".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestItemV1 {
    pub id: u64,
    pub title: String,
    #[serde(default = "default_repo_url")]
    pub repository_url: String,
    pub user: PullRequestItemUserV1,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
    pub pull_request: PullRequestItemPullRequestV1,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestItemPullRequestV1 {
    pub html_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestItemUserV1 {
    pub login: String,
    pub avatar_url: String,
}
