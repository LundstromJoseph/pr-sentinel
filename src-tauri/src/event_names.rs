use crate::{
    app_state::{AppData, GithubFilter, PullRequestsData},
    AppConfig,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Event name constants
pub struct EventNames;

impl EventNames {
    pub const POLLING_ERROR: &'static str = "polling-error";
    pub const FILTER_DATA_UPDATED: &'static str = "filter-data-updated";
    pub const APP_CONFIG_UPDATED: &'static str = "app-config-updated";
    pub const APP_DATA_UPDATED: &'static str = "app-data-updated";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppDataUpdatedPayload {
    pub data: AppData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterDataUpdatedPayload {
    pub filter: GithubFilter,
    pub new_data: PullRequestsData,
    pub old_data: Option<PullRequestsData>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfigUpdatedPayload {
    pub config: AppConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollingErrorPayload {
    pub filter_id: Uuid,
    pub filter_name: String,
    pub error: String,
    pub timestamp: u64,
}
