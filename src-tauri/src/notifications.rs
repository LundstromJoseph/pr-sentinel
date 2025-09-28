use crate::{
    app_data::PullRequestCategory,
    event_names::{EventNames, FilterDataUpdatedPayload},
    PullRequestItem,
};
use serde::{Deserialize, Serialize};
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
    let mut prs_missing_review: Vec<PullRequestItem> = Vec::new();
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
        if old_pr.map_or(true, |existing_pr| existing_pr.category != pr.category) {
            if pr.category == PullRequestCategory::Rereview {
                prs_rereview.push(pr);
            } else if pr.category == PullRequestCategory::MineApproved {
                prs_approved.push(pr);
            } else if pr.category == PullRequestCategory::MineChangesRequested {
                prs_changes_requested.push(pr);
            } else if pr.category == PullRequestCategory::ReviewMissing {
                prs_missing_review.push(pr);
            }
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
    if prs_missing_review.len() > 0 {
        send_pull_request_notification(
            prs_missing_review,
            app_handle.clone(),
            "PRs missing review",
        );
    }
}

fn send_pull_request_notification(
    pull_requests: Vec<PullRequestItem>,
    app_handle: tauri::AppHandle,
    title: &str,
) {
    let body = if pull_requests.len() > 3 {
        format!("{} {}", &pull_requests.len(), title)
    } else {
        format!("{}", format_titles(&pull_requests))
    };

    let formatted_title = format!("- {} -", title);

    let result = send_notification(app_handle.clone(), &formatted_title, &body);
    match result {
        Ok(s) => crate::log::info(&format!("Notification sent: {}", s)),
        Err(e) => crate::log::error(&format!("Failed to send notification: {}", e)),
    }
}

fn send_notification(
    app_handle: tauri::AppHandle,
    title: &str,
    body: &str,
) -> Result<String, String> {
    return if cfg!(target_os = "linux") {
        send_notification_linux(title, body)
    } else {
        send_notification_macos(app_handle.clone(), title, body)
    };
}

fn send_notification_macos(
    app_handle: tauri::AppHandle,
    title: &str,
    body: &str,
) -> Result<String, String> {
    let result = app_handle
        .notification()
        .builder()
        .title(title)
        .body(body)
        .auto_cancel()
        .show();

    return match result {
        Ok(_) => Ok("Successfully sent notification".to_string()),
        Err(e) => Err("Failed to send notification".to_string() + &e.to_string()),
    };
}

fn send_notification_linux(title: &str, body: &str) -> Result<String, String> {
    let result = std::process::Command::new("notify-send")
        .args([
            "--app-name=pr-sentinel",
            "--urgency=normal",
            "--expire-time=5000",
            "--hint=string:sound-name:message-new-instant",
            &title,
            &body,
        ])
        .output();

    match result {
        Ok(output) if output.status.success() => {
            return Ok("Successfully sent notification with status: ".to_string()
                + &output.status.to_string());
        }
        Ok(o) => {
            return Err(
                "Failed to send notification with status: ".to_string() + &o.status.to_string()
            );
        }
        Err(e) => {
            return Err(e.to_string());
        }
    }
}

fn format_titles(pull_requests: &Vec<PullRequestItem>) -> String {
    let titles: Vec<&str> = pull_requests
        .iter()
        .take(3)
        .map(|pr| pr.title.as_str())
        .collect();
    return titles.join("\n\n");
}

#[tauri::command]
pub fn test_notification(app_handle: tauri::AppHandle) -> Result<NotificationResult, ()> {
    let result = send_notification(
        app_handle.clone(),
        "Test Notification",
        "This is a test notification",
    );

    return match result {
        Ok(s) => Ok(NotificationResult { status: s }),
        Err(e) => Ok(NotificationResult { status: e }),
    };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationResult {
    pub status: String,
}
