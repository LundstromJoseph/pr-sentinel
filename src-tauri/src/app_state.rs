use anyhow::Result;
use octocrab::models::pulls::Review;
use octocrab::models::pulls::ReviewState;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::Emitter;
use tauri::Manager;

use tokio::sync::Mutex;

use crate::app_data::AppConfig;
use crate::app_data::AppData;
use crate::app_data::PullRequestCategory;
use crate::app_data::PullRequestItem;
use crate::app_data::PullRequestsData;
use crate::event_names::AppConfigUpdatedPayload;
use crate::event_names::AppDataUpdatedPayload;
use crate::event_names::{EventNames, FilterDataUpdatedPayload};
use crate::github_service::GithubPRWithReviews;

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

fn group_by_user(reviews: Vec<Review>) -> HashMap<String, Vec<Review>> {
    let mut reviews_by_user = HashMap::new();
    for review in reviews {
        if review.user.is_none() {
            continue;
        }
        reviews_by_user
            .entry(review.user.clone().unwrap().login)
            .or_insert(Vec::new())
            .push(review);
    }
    reviews_by_user
}

fn get_latest_review(reviews: &Vec<Review>) -> Review {
    reviews
        .iter()
        .max_by_key(|r| r.submitted_at)
        .unwrap()
        .clone()
}

fn is_user_review_requested(pr_with_reviews: &GithubPRWithReviews, username: &String) -> bool {
    pr_with_reviews
        .reviewers
        .users
        .iter()
        .any(|r| r.login.clone() == username.clone())
}

fn review_is(review: &(Review, bool), state: ReviewState) -> bool {
    if review.1 {
        return false;
    }
    return review.0.state == Some(state);
}

fn get_category_from_reviews(
    pr_with_reviews: &GithubPRWithReviews,
    config: &AppConfig,
) -> PullRequestCategory {
    let reviews_by_user = group_by_user(pr_with_reviews.reviews.clone());
    let all_latest_reviews: Vec<(Review, bool)> = reviews_by_user
        .iter()
        .map(|(key, value)| {
            let is_review_requested = is_user_review_requested(pr_with_reviews, &key);

            let latest_review = get_latest_review(value);
            (latest_review, is_review_requested)
        })
        .collect();

    let needed_approvals = if pr_with_reviews
        .pr
        .repository_url
        .to_string()
        .contains("web-app")
    {
        2
    } else {
        1
    };

    let pr_is_mine =
        pr_with_reviews.pr.user.login == config.username.clone().unwrap_or("".to_string());

    if pr_is_mine {
        if all_latest_reviews
            .iter()
            .any(|r| review_is(r, ReviewState::ChangesRequested))
        {
            return PullRequestCategory::MineChangesRequested;
        } else if all_latest_reviews
            .iter()
            .filter(|r| review_is(r, ReviewState::Approved))
            .count()
            >= needed_approvals
        {
            return PullRequestCategory::MineApproved;
        } else {
            return PullRequestCategory::MinePending;
        }
    }

    let Some(username) = config.username.clone() else {
        return PullRequestCategory::ReviewRequested;
    };

    let amount_of_reviews = all_latest_reviews
        .iter()
        .filter(|r| r.0.user.as_ref().map(|u| u.login.clone()) != Some(username.clone()))
        .count();

    let someone_else_has_reviewed = amount_of_reviews >= needed_approvals;

    let user_has_reviewed = reviews_by_user.contains_key(&username);
    if user_has_reviewed {
        let user_latest_review = get_latest_review(&reviews_by_user[&username]);
        if is_user_review_requested(pr_with_reviews, &username)
            || user_latest_review.state == Some(ReviewState::Dismissed)
        {
            return PullRequestCategory::Rereview;
        }
    }

    return if someone_else_has_reviewed {
        PullRequestCategory::ReviewRequested
    } else {
        PullRequestCategory::ReviewMissing
    };
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
        category: get_category_from_reviews(github_pr_with_reviews, &config),
    }
}

pub async fn new_pull_request_response(
    app_handle: tauri::AppHandle,
    response: Vec<GithubPRWithReviews>,
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

    let payload = FilterDataUpdatedPayload {
        new_data: new_pr_data.clone(),
        old_data: old_pr_data.clone(),
    };
    app_handle
        .emit(EventNames::FILTER_DATA_UPDATED, payload)
        .unwrap_or_else(|e| {
            eprintln!("Failed to emit app data updated event: {}", e);
        });

    let new_app_data = app_handle.state::<AppState>().data.lock().await.clone();
    let payload = AppDataUpdatedPayload { data: new_app_data };
    app_handle
        .emit(EventNames::APP_DATA_UPDATED, payload)
        .unwrap_or_else(|e| {
            eprintln!("Failed to emit app data updated event: {}", e);
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
    })
}

#[tauri::command]
pub async fn get_data(state: tauri::State<'_, AppState>) -> Result<AppData, String> {
    let data = state.data.lock().await;
    Ok(data.clone())
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
            eprintln!("Failed to emit app data updated event: {}", e);
        });
    Ok(())
}
