use serde::Deserialize;
use tauri::{Listener, Manager};

use crate::{
    app_data::{AppConfig, AppData},
    app_data_v1::{AppConfigV1, AppDataV1},
    app_data_v2::{convert_config_to_v2, convert_data_to_v2},
    event_names::{AppConfigUpdatedPayload, AppDataUpdatedPayload, EventNames},
    AppState, PullRequestsData,
};
use std::path::PathBuf;

fn default_version() -> u8 {
    1
}

#[derive(Deserialize)]
pub struct VersionOnly {
    #[serde(default = "default_version")]
    pub version: u8,
}

const FOLDER_NAME: &str = if cfg!(dev) {
    ".pr_sentinel_dev"
} else {
    ".pr_sentinel"
};

pub fn get_logs_path() -> PathBuf {
    let logs_dir = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))
        .unwrap()
        .join(FOLDER_NAME);

    logs_dir.join("out.log")
}

fn get_config_path() -> PathBuf {
    let config_dir = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))
        .unwrap()
        .join(FOLDER_NAME);

    config_dir.join("config.json")
}

fn get_data_path() -> PathBuf {
    let config_dir = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))
        .unwrap()
        .join(FOLDER_NAME);

    config_dir.join("data.json")
}

async fn save_data(data: AppData) {
    let data_path = get_data_path();
    if !data_path.exists() {
        let parent_dir = data_path.parent().unwrap();
        std::fs::create_dir_all(parent_dir).unwrap();
    }

    std::fs::write(data_path, serde_json::to_string(&data).unwrap()).unwrap();
}

async fn save_config(config: AppConfig) {
    let config_path = get_config_path();
    if !config_path.exists() {
        let parent_dir = config_path.parent().unwrap();
        std::fs::create_dir_all(parent_dir).unwrap();
    }

    std::fs::write(config_path, serde_json::to_string(&config).unwrap()).unwrap();
}

pub async fn load_config() -> Result<AppConfig, String> {
    let config_path = get_config_path();
    if !config_path.exists() {
        crate::log::error("Config path does not exist");
        return Ok(AppConfig {
            version: 2,
            github_token: None,
            username: None,
            repo_config: Vec::new(),
        });
    }

    let content = std::fs::read_to_string(config_path).map_err(|e| e.to_string())?;

    let version_only: VersionOnly = serde_json::from_str(&content).unwrap();

    crate::log::info(&format!("Config version: {:?}", version_only.version));

    if version_only.version == 1 {
        let config_v1: AppConfigV1 = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        let config_v2 = convert_config_to_v2(&config_v1).await;
        save_config(config_v2.clone()).await;
        return Ok(config_v2);
    } else if version_only.version == 2 {
        let config_v2: AppConfig = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        return Ok(config_v2);
    } else {
        return Err("Unsupported config version".to_string());
    }
}

pub async fn load_data() -> Result<AppData, String> {
    let data_path = get_data_path();
    if !data_path.exists() {
        return Ok(AppData {
            version: 2,
            pull_requests: PullRequestsData {
                last_updated: 0,
                pull_requests: Vec::new(),
            },
        });
    }

    let content = std::fs::read_to_string(data_path).map_err(|e| e.to_string())?;

    let version_only: VersionOnly = serde_json::from_str(&content).unwrap();

    if version_only.version == 1 {
        let data_v1: AppDataV1 = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        let data_v2 = convert_data_to_v2(data_v1);
        save_data(data_v2.clone()).await;
        return Ok(data_v2);
    } else if version_only.version == 2 {
        let data_v2: AppData = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        return Ok(data_v2);
    } else {
        return Err("Unsupported data version".to_string());
    }
}

pub async fn load_state(app_handle: tauri::AppHandle) {
    let config = load_config().await.expect("Failed to load config");
    let data = load_data().await.expect("Failed to load data");
    {
        let state = app_handle.state::<AppState>();
        *state.data.lock().await = data;
        *state.config.lock().await = config;
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
