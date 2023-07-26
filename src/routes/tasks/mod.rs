pub mod create_task;
pub mod create_task_extractor;
pub mod get_all_tasks;
pub mod get_one_task;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestTask {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
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
