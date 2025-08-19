use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct NewTask {
    pub title: String,
}

#[derive(Debug, Serialize)]
pub struct TaskResponse {
    pub status: String,
    pub task: Option<Task>,
}

#[derive(Debug, Serialize)]
pub struct DeleteResponse {
    pub status: String,
    pub count: u64,
}