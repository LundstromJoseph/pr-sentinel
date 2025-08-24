use crate::{
    app_state::GithubFilter,
    event_names::{EventNames, FilterDataUpdatedPayload},
    github_service::PullRequestItem,
};
use tauri::Listener;
use tauri_plugin_notification::NotificationExt;

pub fn init_listeners(app_handle: tauri::AppHandle) {
    let my_app_handle = app_handle.clone();
    app_handle.listen(EventNames::FILTER_DATA_UPDATED, move |event| {
        let payload: FilterDataUpdatedPayload = serde_json::from_str(event.payload()).unwrap();
        notify_new_pull_requests(payload, my_app_handle.clone());
    });
}

fn notify_new_pull_requests(payload: FilterDataUpdatedPayload, app_handle: tauri::AppHandle) {
    let filter = payload.filter;

    if !filter.notify {
        return;
    }

    let not_seen_prs: Vec<PullRequestItem> = match payload.old_data {
        Some(old_prs) => payload
            .new_data
            .pull_requests
            .iter()
            .filter(|pr| {
                !old_prs
                    .pull_requests
                    .iter()
                    .any(|old_pr| old_pr.id == pr.id)
            })
            .cloned()
            .collect(),
        None => payload.new_data.pull_requests,
    };

    if !not_seen_prs.is_empty() {
        send_pull_request_notification(filter, not_seen_prs, app_handle);
    }
}

fn send_pull_request_notification(
    filter: GithubFilter,
    pull_requests: Vec<PullRequestItem>,
    app_handle: tauri::AppHandle,
) {
    let notification_result = if pull_requests.len() > 3 {
        // Send summary notification for many PRs
        app_handle
            .notification()
            .builder()
            .title(&format!("{} - New PRs", filter.name))
            .body(&format!(
                "{} new pull requests available",
                pull_requests.len()
            ))
            .show()
    } else if pull_requests.len() == 1 {
        // Send detailed notification for single PR
        let pr = pull_requests[0].clone();
        app_handle
            .notification()
            .builder()
            .title(&format!("{} - New PR", filter.name))
            .body(&format!("{} - {}", pr.title, pr.user.login))
            .show()
    } else {
        // Send summary for 2-3 PRs
        let titles: Vec<&str> = pull_requests
            .iter()
            .take(3)
            .map(|pr| pr.title.as_str())
            .collect();
        app_handle
            .notification()
            .builder()
            .title(&format!("{} - New PRs", filter.name))
            .body(&format!("New PRs:\n{}", titles.join("\n")))
            .show()
    };

    if let Err(e) = notification_result {
        eprintln!("Failed to send notification: {}", e);
    }
}
