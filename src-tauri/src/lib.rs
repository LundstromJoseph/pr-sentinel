// Import our custom modules
mod app_state;
mod event_names;
mod file_storage;
mod github_service;
mod notifications;
mod polling;

pub use app_state::{AppConfig, AppState};
pub use file_storage::{load_config, load_data};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = AppState::new().expect("Failed to initialize app state");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            app_state::get_config,
            app_state::get_data,
            app_state::add_filter,
            app_state::update_filter,
            app_state::remove_filter,
            app_state::reorder_filter,
            verify_token,
            app_state::save_token,
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
            eprintln!("Failed to load state: {}", e);
        });
    tauri::async_runtime::spawn(polling::start_polling_job(app_handle.clone()))
        .await
        .unwrap_or_else(|e| {
            eprintln!("Failed to start polling job: {}", e);
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
