pub mod app_state;
pub mod mock_state;
pub mod model_handler;

use axum::Json;
use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::common::{Id, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    Doing,
    Done,
}

#[derive(Debug, Clone, Serialize)]
pub struct Task {
    pub id: Id,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewTask {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: TaskStatus,
}

#[async_trait::async_trait]
pub trait ITaskManagerModel: DynClone + Send + Sync + Debug {
    async fn get_all(&self) -> Result<Vec<Task>>;
    async fn get(&self, id: Id) -> Result<Vec<Task>>;
    async fn create(&self, new_task: NewTask) -> Result<Json<Task>>;
    async fn update(&self, id: Id, update_task: UpdateTask) -> Result<Json<Task>>;
    async fn delete(&self, id: Id) -> Result<Json<Task>>;
}

dyn_clone::clone_trait_object!(ITaskManagerModel);
