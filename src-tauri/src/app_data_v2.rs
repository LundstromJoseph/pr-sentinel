use serde::{Deserialize, Serialize};

use crate::{
    app_data_v1::{AppConfigV1, AppDataV1},
    verify_token,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfigV2 {
    pub version: u8,
    pub github_token: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppDataV2 {
    pub version: u8,
    pub pull_requests: PullRequestsDataV2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestsDataV2 {
    pub last_updated: u64,
    pub pull_requests: Vec<PullRequestItemV2>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PullRequestCategoryV2 {
    MineApproved,
    MineChangesRequested,
    MinePending,
    ReviewRequested,
    Rereview,
    ReviewMissing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestItemV2 {
    pub id: u64,
    pub title: String,
    pub repository_url: String,
    pub login: String,
    pub avatar_url: String,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
    pub html_url: String,
    pub category: PullRequestCategoryV2,
}

pub async fn convert_config_to_v2(config: AppConfigV1) -> AppConfigV2 {
    let config = AppConfigV2 {
        version: 2,
        github_token: config.github_token.clone(),
        username: match config.github_token {
            Some(token) => Some(verify_token(token).await.unwrap()),
            None => None,
        },
    };
    config
}

pub fn convert_data_to_v2(_data: AppDataV1) -> AppDataV2 {
    AppDataV2 {
        version: 2,
        pull_requests: PullRequestsDataV2 {
            last_updated: 0,
            pull_requests: Vec::new(),
        },
    }
}
