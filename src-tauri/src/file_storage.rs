use tauri::{Listener, Manager};

use crate::{
    app_state::{AppConfig, AppData},
    event_names::{AppConfigUpdatedPayload, AppDataUpdatedPayload, EventNames},
    AppState,
};
use std::{collections::HashMap, path::PathBuf};

fn get_config_path() -> PathBuf {
    let config_dir = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))
        .unwrap()
        .join(".preview_app");

    config_dir.join("config.json")
}

fn get_data_path() -> PathBuf {
    let config_dir = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))
        .unwrap()
        .join(".preview_app");

    config_dir.join("data.json")
}

async fn save_data(data: AppData) {
    let data_path = get_data_path();
    std::fs::write(data_path, serde_json::to_string(&data).unwrap()).unwrap();
}

async fn save_config(config: AppConfig) {
    let config_path = get_config_path();
    std::fs::write(config_path, serde_json::to_string(&config).unwrap()).unwrap();
}

pub async fn load_config() -> Result<AppConfig, String> {
    let config_path = get_config_path();
    println!("Config path: {:?}", config_path);
    if !config_path.exists() {
        println!("Config path does not exist");
        return Ok(AppConfig {
            github_token: None,
            filters: Vec::new(),
        });
    }

    let content = std::fs::read_to_string(config_path).map_err(|e| e.to_string())?;
    println!("Content: {:?}", content);
    let config: AppConfig = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    Ok(config)
}

pub async fn load_data() -> Result<AppData, String> {
    let data_path = get_data_path();
    if !data_path.exists() {
        return Ok(AppData {
            pull_requests: HashMap::new(),
        });
    }
    let content = std::fs::read_to_string(data_path).map_err(|e| e.to_string())?;
    let data: AppData = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    Ok(data)
}

pub async fn load_state(app_handle: tauri::AppHandle) {
    let config = load_config().await.expect("Failed to load config");
    let data = load_data().await.expect("Failed to load data");
    {
        let state = app_handle.state::<AppState>();
        state.data.lock().await.pull_requests = data.pull_requests;
        state.config.lock().await.github_token = config.github_token;
        state.config.lock().await.filters = config.filters;
    }
}

pub fn init_listeners(app_handle: tauri::AppHandle) {
    app_handle.listen(EventNames::APP_DATA_UPDATED, |event| {
        let payload: AppDataUpdatedPayload = serde_json::from_str(event.payload()).unwrap();

        tauri::async_runtime::spawn(async move {
            save_data(payload.data).await;
        });
    });

    app_handle.listen(EventNames::APP_CONFIG_UPDATED, |event| {
        let payload: AppConfigUpdatedPayload = serde_json::from_str(event.payload()).unwrap();

        tauri::async_runtime::spawn(async move {
            save_config(payload.config).await;
        });
    });
}
