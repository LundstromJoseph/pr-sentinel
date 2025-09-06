use crate::{
    app_data::PullRequestCategory,
    event_names::{EventNames, FilterDataUpdatedPayload},
    PullRequestItem,
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
    let mut prs_new: Vec<PullRequestItem> = Vec::new();
    let mut prs_rereview: Vec<PullRequestItem> = Vec::new();
    let mut prs_approved: Vec<PullRequestItem> = Vec::new();
    let mut prs_changes_requested: Vec<PullRequestItem> = Vec::new();

    let new_data = payload.new_data;

    let old_data = payload.old_data;

    for pr in new_data.pull_requests {
        let old_pr = old_data
            .pull_requests
            .iter()
            .find(|old_pr| old_pr.id == pr.id);
        if old_pr.is_some() {
            if old_pr.unwrap().category != pr.category {
                if pr.category == PullRequestCategory::Rereview {
                    prs_rereview.push(pr);
                } else if pr.category == PullRequestCategory::MineApproved {
                    prs_approved.push(pr);
                } else if pr.category == PullRequestCategory::MineChangesRequested {
                    prs_changes_requested.push(pr);
                }
            }
        } else {
            prs_new.push(pr);
        }
    }

    if prs_rereview.len() > 0 {
        send_pull_request_notification(prs_rereview, app_handle.clone(), "PRs to re-review");
    }
    if prs_approved.len() > 0 {
        send_pull_request_notification(prs_approved, app_handle.clone(), "PRs approved");
    }
    if prs_changes_requested.len() > 0 {
        send_pull_request_notification(prs_changes_requested, app_handle.clone(), "PRs rejected");
    }
    if prs_new.len() > 0 {
        send_pull_request_notification(prs_new, app_handle.clone(), "New PRs");
    }
}

fn send_pull_request_notification(
    pull_requests: Vec<PullRequestItem>,
    app_handle: tauri::AppHandle,
    title: &str,
) {
    let notification_result = if pull_requests.len() > 3 {
        // Send summary notification for many PRs
        app_handle
            .notification()
            .builder()
            .title(&format!("{} {}", pull_requests.len(), title))
            .show()
    } else {
        app_handle
            .notification()
            .builder()
            .title(&format!("- {} -", title))
            .body(&format!("{}", format_titles(&pull_requests)))
            .show()
    };

    if let Err(e) = notification_result {
        eprintln!("Failed to send notification: {}", e);
    }
}

fn format_titles(pull_requests: &Vec<PullRequestItem>) -> String {
    let titles: Vec<&str> = pull_requests
        .iter()
        .take(3)
        .map(|pr| pr.title.as_str())
        .collect();
    return titles.join("\n");
}
