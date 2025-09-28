use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};
use tokio::time;

use crate::{
    app_state::{self, AppState},
    event_names::EventNames,
    github_service::GithubClient,
};

const POLLING_INTERVAL_SECONDS: u64 = 600;
const REFRESH_TIMEOUT_SECONDS: u64 = 30;

// Polling job
pub async fn start_polling_job(app_handle: AppHandle) {
    let mut interval = time::interval(Duration::from_secs(POLLING_INTERVAL_SECONDS));

    crate::log::info("Starting polling job");
    crate::log::info(&format!(
        "Polling interval: {:?}s, timeout: {:?}s",
        POLLING_INTERVAL_SECONDS, REFRESH_TIMEOUT_SECONDS
    ));

    loop {
        // Wait for the next interval tick
        interval.tick().await;

        let app_handle_clone = app_handle.clone();

        // Create a timeout future for the refresh operation
        match time::timeout(
            Duration::from_secs(REFRESH_TIMEOUT_SECONDS),
            refresh_all_filters(app_handle_clone),
        )
        .await
        {
            Ok(_) => {
                // Refresh completed successfully within timeout
            }
            Err(_) => {
                // Timeout occurred
                crate::log::error(&format!(
                    "Refresh operation timed out after {}s",
                    REFRESH_TIMEOUT_SECONDS
                ));
                app_handle
                    .emit(
                        EventNames::POLLING_ERROR,
                        serde_json::json!({
                            "error": format!("Refresh operation timed out after {}s", REFRESH_TIMEOUT_SECONDS)
                        }),
                    )
                    .unwrap_or_else(|e| {
                        crate::log::error(&format!("Failed to emit error event: {}", e));
                    });
            }
        }
    }
}

pub async fn refresh_all_filters(app_handle: AppHandle) {
    let state = app_handle
        .try_state::<AppState>()
        .expect("Failed to get app state");

    let github_token = {
        let config = state.config.lock().await;
        config.github_token.clone()
    };

    let Some(ok_token) = github_token else {
        crate::log::error("No github token found");
        return;
    };

    let client = GithubClient::new(ok_token);

    match client
        .search_pull_requests("is:pr is:open involves:@me draft:false".to_string())
        .await
    {
        Ok(response) => {
            app_state::new_pull_request_response(&app_handle, &response).await;
        }
        Err(e) => {
            crate::log::error(&format!("Error polling pull requests: {}", e));
            // Emit error event
            app_handle
                .emit(
                    EventNames::POLLING_ERROR,
                    serde_json::json!({
                        "error": e.to_string()
                    }),
                )
                .unwrap_or_else(|e| {
                    crate::log::error(&format!("Failed to emit error event: {}", e));
                });
        }
    }
}

#[tauri::command]
pub async fn refresh(app_handle: AppHandle) {
    refresh_all_filters(app_handle).await;
}
