use axum::Json;

use super::mock_state::MockTaskManagerModel;
use crate::common::{Id, Result};
use crate::web::task_manager::model::{ITaskManagerModel, NewTask, Task, UpdateTask};

#[derive(Debug, Clone)]
pub struct AppState {
    pub task_manager_model: Box<dyn ITaskManagerModel>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            task_manager_model: Box::new(MockTaskManagerModel::default()),
        }
    }
}

#[async_trait::async_trait]
impl ITaskManagerModel for AppState {
    async fn get_all(&self) -> Result<Vec<Task>> {
        self.task_manager_model.get_all().await
    }

    async fn get(&self, id: Id) -> Result<Vec<Task>> {
        self.task_manager_model.get(id).await
    }

    async fn create(&self, new_task: NewTask) -> Result<Json<Task>> {
        self.task_manager_model.create(new_task).await
    }

    async fn update(&self, id: Id, update_task: UpdateTask) -> Result<Json<Task>> {
        self.task_manager_model.update(id, update_task).await
    }

    async fn delete(&self, id: Id) -> Result<Json<Task>> {
        self.task_manager_model.delete(id).await
    }
}
