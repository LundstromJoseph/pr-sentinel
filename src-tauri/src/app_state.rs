use anyhow::Result;
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubFilterCreate {
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
    Ok(config.clone())
}

#[tauri::command]
pub async fn get_data(state: tauri::State<'_, AppState>) -> Result<AppData, String> {
    let data = state.data.lock().await;
    Ok(data.clone())
}

#[tauri::command]
pub async fn add_filter(
    filter: GithubFilterCreate,
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
        config.filters.push(GithubFilter {
            id: Uuid::new_v4(),
            query: filter.query,
            notify: filter.notify,
            name: filter.name,
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
