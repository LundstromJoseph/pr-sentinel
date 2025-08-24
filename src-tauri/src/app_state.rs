use anyhow::Result;
use fractional_index::FractionalIndex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::Emitter;
use tauri::Manager;

use tokio::sync::Mutex;
use uuid::Uuid;

use crate::event_names::AppConfigUpdatedPayload;
use crate::event_names::AppDataUpdatedPayload;
use crate::event_names::{EventNames, FilterDataUpdatedPayload};
use crate::github_service::GithubResponse;
use crate::github_service::PullRequestItem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub github_token: Option<String>,
    pub filters: Vec<GithubFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppData {
    pub pull_requests: HashMap<Uuid, PullRequestsData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestsData {
    pub last_updated: u64,
    pub pull_requests: Vec<PullRequestItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubFilter {
    pub id: Uuid,
    pub query: String,
    pub notify: bool,
    pub name: String,
    pub fractional_index: FractionalIndex,
}

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
                github_token: None,
                filters: Vec::new(),
            })),
            data: Arc::new(Mutex::new(AppData {
                pull_requests: HashMap::new(),
            })),
        })
    }
}

pub async fn new_pull_request_response(
    app_handle: tauri::AppHandle,
    github_filter: GithubFilter,
    response: GithubResponse,
) {
    let old_data = app_handle.state::<AppState>().data.lock().await.clone();

    let old_pr_data = old_data.pull_requests.get(&github_filter.id);

    let new_pr_data = PullRequestsData {
        last_updated: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        pull_requests: response.items,
    };

    app_handle
        .state::<AppState>()
        .data
        .lock()
        .await
        .pull_requests
        .insert(github_filter.id, new_pr_data.clone());

    let payload = FilterDataUpdatedPayload {
        filter: github_filter,
        new_data: new_pr_data.clone(),
        old_data: match old_pr_data {
            Some(data) => Some(data.clone()),
            None => None,
        },
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
        filters: config.filters.clone(),
        github_token: config.github_token.clone(),
    })
}

#[tauri::command]
pub async fn get_data(state: tauri::State<'_, AppState>) -> Result<AppData, String> {
    let data = state.data.lock().await;
    Ok(data.clone())
}

#[tauri::command]
pub async fn update_filter(
    id: Uuid,
    filter: GithubFilterUpdate,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    if filter.query.is_empty() {
        return Err("Query cannot be empty".to_string());
    }
    if filter.name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }

    let state = app_handle.state::<AppState>();
    {
        let read_config = state.config.lock().await.clone();
        let current_filter = read_config.filters.iter().find(|r| r.id == id);
        if current_filter.is_none() {
            return Err("Filter not found".to_string());
        }

        let current_filter = current_filter.unwrap();

        let mut config = state.config.lock().await;
        config.filters.retain(|r| r.id != id);
        config.filters.push(GithubFilter {
            id: id,
            query: filter.query,
            notify: filter.notify,
            name: filter.name,
            fractional_index: current_filter.fractional_index.clone(),
        });
        config
            .filters
            .sort_by_key(|r| r.fractional_index.to_string());
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

#[tauri::command]
pub async fn reorder_filter(
    filter_id: Uuid,
    direction: String,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    if direction != "up" && direction != "down" {
        return Err("Invalid direction".to_string());
    }

    let state = app_handle.state::<AppState>();
    {
        let mut config = state.config.lock().await;
        if config.filters.len() <= 1 {
            return Ok(());
        }

        config.filters.sort_by_key(|r| r.fractional_index.clone());
        let index = config
            .filters
            .iter()
            .position(|r| r.id == filter_id)
            .unwrap();
        if index == 0 && direction == "up" {
            return Ok(());
        }
        if index == config.filters.len() - 1 && direction == "down" {
            return Ok(());
        }

        if direction == "up" {
            let before = if index >= 2 {
                config.filters.get(index - 2)
            } else {
                None
            };
            let after = if index >= 1 {
                config.filters.get(index - 1)
            } else {
                None
            };
            config.filters[index].fractional_index = fractional_index::FractionalIndex::new(
                before.map(|r| &r.fractional_index),
                after.map(|r| &r.fractional_index),
            )
            .unwrap();
        } else {
            let before = if index + 1 < config.filters.len() {
                config.filters.get(index + 1)
            } else {
                None
            };
            let after = if index + 2 < config.filters.len() {
                config.filters.get(index + 2)
            } else {
                None
            };
            config.filters[index].fractional_index = fractional_index::FractionalIndex::new(
                before.map(|r| &r.fractional_index),
                after.map(|r| &r.fractional_index),
            )
            .unwrap();
        }

        config
            .filters
            .sort_by_key(|r| r.fractional_index.to_string());
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

#[tauri::command]
pub async fn add_filter(
    filter: GithubFilterUpdate,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    if filter.query.is_empty() {
        return Err("Query cannot be empty".to_string());
    }
    if filter.name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }

    let state = app_handle.state::<AppState>();
    {
        let mut config = state.config.lock().await;
        let last_index = config.filters.last().unwrap().fractional_index.clone();
        config.filters.push(GithubFilter {
            id: Uuid::new_v4(),
            query: filter.query,
            notify: filter.notify,
            name: filter.name,
            fractional_index: FractionalIndex::new_after(&last_index),
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
            eprintln!("Failed to emit app data updated event: {}", e);
        });
    Ok(())
}

#[tauri::command]
pub async fn save_token(token: String, app_handle: tauri::AppHandle) -> Result<(), String> {
    let state = app_handle.state::<AppState>();
    {
        let mut config = state.config.lock().await;
        config.github_token = Some(token);
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

#[tauri::command]
pub async fn remove_filter(id: String, app_handle: tauri::AppHandle) -> Result<(), String> {
    let state = app_handle.state::<AppState>();
    {
        let mut config = state.config.lock().await;
        config
            .filters
            .retain(|r| r.id != Uuid::parse_str(&id).unwrap());
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
