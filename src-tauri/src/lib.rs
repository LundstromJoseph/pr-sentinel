// Import our custom modules
mod app_data;
mod app_data_v1;
mod app_data_v2;
mod app_state;
mod event_names;
mod file_storage;
mod github_service;
mod log;
mod notifications;
mod polling;
mod pr_predicates;
pub use app_data::{AppConfig, AppData, PullRequestItem, PullRequestsData};
pub use app_state::AppState;
pub use file_storage::{load_config, load_data};
use tauri_plugin_autostart::MacosLauncher;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = AppState::new().expect("Failed to initialize app state");

    let mut builder = tauri::Builder::default();

    #[cfg(target_os = "macos")]
    {
        builder = builder.plugin(
            tauri_plugin_autostart::Builder::new()
                .app_name("PR Sentinel")
                .macos_launcher(MacosLauncher::AppleScript)
                .build(),
        )
    }

    builder
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            app_state::get_config,
            app_state::get_data,
            verify_token,
            app_state::save_token,
            app_state::save_repo_config,
            polling::refresh,
        ])
        .setup(|app| {
            let app_handle = app.handle();
            tauri::async_runtime::spawn(init(app_handle.clone()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
pub async fn init(app_handle: tauri::AppHandle) {
    file_storage::init_listeners(app_handle.clone());
    notifications::init_listeners(app_handle.clone());
    tauri::async_runtime::spawn(file_storage::load_state(app_handle.clone()))
        .await
        .unwrap_or_else(|e| {
            crate::log::error(&format!("Failed to load state: {}", e));
        });
    tauri::async_runtime::spawn(polling::start_polling_job(app_handle.clone()))
        .await
        .unwrap_or_else(|e| {
            crate::log::error(&format!("Failed to start polling job: {}", e));
        });
}

#[tauri::command]
async fn verify_token(token: String) -> Result<String, ()> {
    let client = github_service::GithubClient::new(token);
    let user = client.get_user().await;
    match user {
        Ok(user) => Ok(user.login),
        Err(_) => Err(()),
    }
}
