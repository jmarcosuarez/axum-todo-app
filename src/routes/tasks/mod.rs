pub mod create_task;
pub mod create_task_extractor;
pub mod delete_task;
pub mod get_all_tasks;
pub mod get_one_task;
pub mod update_tasks;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

// https://docs.rs/serde_with/3.1.0/serde_with/attr.skip_serializing_none.html
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct RequestTask {
    pub priority: Option<Option<String>>,
    // Always serialize next field even if None
    #[serialize_always]
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub completed_at: Option<Option<DateTime<FixedOffset>>>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseTask {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataTask {
    pub data: ResponseTask,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataTasks {
    pub data: Vec<ResponseTask>,
}
