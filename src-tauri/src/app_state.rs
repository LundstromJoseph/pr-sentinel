use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::Emitter;
use tauri::Manager;

use tokio::sync::Mutex;

use crate::app_data::AppConfig;
use crate::app_data::AppData;
use crate::app_data::PullRequestCategory;
use crate::app_data::PullRequestItem;
use crate::app_data::PullRequestsData;
use crate::app_data_v2::RepoConfigV2;
use crate::event_names::AppConfigUpdatedPayload;
use crate::event_names::AppDataUpdatedPayload;
use crate::event_names::{EventNames, FilterDataUpdatedPayload};
use crate::github_service::get_owner_and_repo;
use crate::github_service::GithubPRWithReviews;
use crate::pr_predicates::PR_CATEGORIES;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubFilterUpdate {
    pub query: String,
    pub notify: bool,
    pub name: String,
}

pub struct AppState {
    pub data: Arc<Mutex<AppData>>,
    pub config: Arc<Mutex<AppConfig>>,
}

impl AppState {
    pub fn new() -> Result<Self> {
        Ok(Self {
            config: Arc::new(Mutex::new(AppConfig {
                version: 2,
                github_token: None,
                username: None,
                repo_config: Vec::new(),
            })),
            data: Arc::new(Mutex::new(AppData {
                version: 2,
                pull_requests: PullRequestsData {
                    last_updated: 0,
                    pull_requests: Vec::new(),
                },
            })),
        })
    }
}

fn get_category_from_reviews(
    pr_with_reviews: &GithubPRWithReviews,
    config: &AppConfig,
) -> PullRequestCategory {
    for category in PR_CATEGORIES.iter() {
        if (category.predicate)(pr_with_reviews, config) {
            return category.category.clone();
        }
    }
    crate::log::error(&format!(
        "No category found for PR: {}",
        pr_with_reviews.pr.id
    ));
    return PullRequestCategory::MinePending;
}

fn map_to_app_data(
    github_pr_with_reviews: &GithubPRWithReviews,
    config: &AppConfig,
) -> PullRequestItem {
    PullRequestItem {
        id: github_pr_with_reviews.pr.id.to_be(),
        title: github_pr_with_reviews.pr.title.clone(),
        url: github_pr_with_reviews.pr.url.to_string(),
        repository_url: github_pr_with_reviews.pr.repository_url.to_string(),
        login: github_pr_with_reviews.pr.user.login.clone(),
        avatar_url: github_pr_with_reviews.pr.user.avatar_url.to_string(),
        html_url: github_pr_with_reviews.pr.html_url.to_string(),
        created_at: github_pr_with_reviews.pr.created_at.to_rfc3339(),
        updated_at: github_pr_with_reviews.pr.updated_at.to_rfc3339(),
        is_assigned: github_pr_with_reviews
            .pr
            .assignees
            .iter()
            .any(|a| a.login == config.username.clone().unwrap_or("".to_string())),
        category: get_category_from_reviews(github_pr_with_reviews, &config),
    }
}

pub async fn new_pull_request_response(
    app_handle: &tauri::AppHandle,
    response: &Vec<GithubPRWithReviews>,
) {
    let state = app_handle.state::<AppState>();
    let config = app_handle.state::<AppState>().config.lock().await.clone();
    let old_data = app_handle.state::<AppState>().data.lock().await.clone();

    let old_pr_data = old_data.pull_requests;

    let new_pr_data = PullRequestsData {
        last_updated: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        pull_requests: response
            .iter()
            .map(|r| map_to_app_data(r, &config))
            .collect(),
    };

    {
        let mut data = state.data.lock().await;
        data.pull_requests = new_pr_data.clone();
    }

    let all_repos = new_pr_data
        .pull_requests
        .iter()
        .map(|r| get_owner_and_repo(&r.repository_url))
        .map(|(owner, repo)| format!("{}/{}", owner, repo))
        .collect::<Vec<String>>();

    for repo in all_repos {
        let repo_config = config
            .repo_config
            .iter()
            .find(|r| r.repo_name == repo)
            .cloned();
        if repo_config.is_none() {
            save_repo_config(repo, 1, app_handle.clone())
                .await
                .unwrap_or_else(|e| {
                    crate::log::error(&format!("Failed to save repo config: {}", e));
                });
        }
    }

    let payload = FilterDataUpdatedPayload {
        new_data: new_pr_data.clone(),
        old_data: old_pr_data.clone(),
    };
    app_handle
        .emit(EventNames::FILTER_DATA_UPDATED, payload)
        .unwrap_or_else(|e| {
            crate::log::error(&format!("Failed to emit app data updated event: {}", e));
        });

    let new_app_data = app_handle.state::<AppState>().data.lock().await.clone();
    let payload = AppDataUpdatedPayload { data: new_app_data };
    app_handle
        .emit(EventNames::APP_DATA_UPDATED, payload)
        .unwrap_or_else(|e| {
            crate::log::error(&format!("Failed to emit app data updated event: {}", e));
        });
}

// Tauri commands
#[tauri::command]
pub async fn get_config(state: tauri::State<'_, AppState>) -> Result<AppConfig, String> {
    let config = state.config.lock().await;

    Ok(AppConfig {
        version: config.version,
        username: config.username.clone(),
        github_token: config.github_token.clone(),
        repo_config: config.repo_config.clone(),
    })
}

#[tauri::command]
pub async fn get_data(state: tauri::State<'_, AppState>) -> Result<AppData, String> {
    let data = state.data.lock().await;
    Ok(data.clone())
}

#[tauri::command]
pub async fn save_repo_config(
    repo_name: String,
    needed_approvals: usize,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let state = app_handle.state::<AppState>();
    {
        let mut config = state.config.lock().await;

        config.repo_config = config
            .repo_config
            .iter()
            .filter(|r| r.repo_name != repo_name)
            .cloned()
            .collect();

        config.repo_config.push(RepoConfigV2 {
            repo_name,
            needed_approvals,
        });
    }

    app_handle
        .emit(
            EventNames::APP_CONFIG_UPDATED,
            AppConfigUpdatedPayload {
                config: state.config.lock().await.clone(),
            },
        )
        .unwrap_or_else(|e| {
            crate::log::error(&format!("Failed to emit app data updated event: {}", e));
        });

    Ok(())
}

#[tauri::command]
pub async fn save_token(
    token: String,
    username: String,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let state = app_handle.state::<AppState>();
    {
        let mut config = state.config.lock().await;
        config.github_token = Some(token);
        config.username = Some(username);
    }
    app_handle
        .emit(
            EventNames::APP_CONFIG_UPDATED,
            AppConfigUpdatedPayload {
                config: state.config.lock().await.clone(),
            },
        )
        .unwrap_or_else(|e| {
            crate::log::error(&format!("Failed to emit app data updated event: {}", e));
        });
    Ok(())
}
