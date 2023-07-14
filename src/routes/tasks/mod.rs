use serde::{Deserialize, Serialize};

pub mod create_task;
pub mod create_task_extractor;

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
