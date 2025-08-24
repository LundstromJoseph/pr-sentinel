use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};
use tokio::time;

use crate::{
    app_state::{self, AppState},
    event_names::EventNames,
    github_service::GithubClient,
};

const POLLING_INTERVAL_SECONDS: u64 = 600;

// Polling job
pub async fn start_polling_job(app_handle: AppHandle) {
    let mut interval = time::interval(Duration::from_secs(POLLING_INTERVAL_SECONDS));

    println!("Polling interval: {:?}", POLLING_INTERVAL_SECONDS);

    loop {
        interval.tick().await;

        refresh_all_filters(app_handle.clone()).await;
    }
}

pub async fn refresh_all_filters(app_handle: AppHandle) {
    let state = app_handle
        .try_state::<AppState>()
        .expect("Failed to get app state");

    let (filters, github_token) = {
        let config = state.config.lock().await;
        (config.filters.clone(), config.github_token.clone())
    };

    let Some(ok_token) = github_token else {
        eprintln!("No github token found");
        return;
    };

    let client = GithubClient::new(ok_token);

    for filter in filters {
        match client.search_pull_requests(filter.query.clone()).await {
            Ok(response) => {
                app_state::new_pull_request_response(app_handle.clone(), filter, response).await;
            }
            Err(e) => {
                // Emit error event
                app_handle
                    .emit(
                        EventNames::POLLING_ERROR,
                        serde_json::json!({
                            "filter_id": filter.id,
                            "filter_name": filter.name,
                            "error": e.to_string()
                        }),
                    )
                    .unwrap_or_else(|e| {
                        eprintln!("Failed to emit error event: {}", e);
                    });
            }
        }
    }
}

#[tauri::command]
pub async fn refresh(app_handle: AppHandle) {
    refresh_all_filters(app_handle).await;
}
