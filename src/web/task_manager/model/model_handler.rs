use axum::extract::Path;
use axum::{extract::State, Json};

use super::app_state::AppState;
use crate::common::Result;
use crate::web::task_manager::model::Task;

pub struct ModelHandler {}

impl ModelHandler {
    pub async fn get_all(State(app_state): State<AppState>) -> Result<Json<Vec<Task>>> {
        let raw_vec = app_state.task_manager_model.get_all().await;
        let vec = raw_vec?;
        Ok(Json(vec))
    }

    pub async fn get(
        State(app_state): State<AppState>,
        Path(id): Path<u128>,
    ) -> Result<Json<Vec<Task>>> {
        let raw_vec = app_state.task_manager_model.get(id).await;
        let vec = raw_vec?;
        Ok(Json(vec))
    }

    pub async fn create(
        State(app_state): State<AppState>,
        Json(new_task): Json<super::NewTask>,
    ) -> Result<Json<Task>> {
        app_state.task_manager_model.create(new_task).await
    }

    #[allow(dead_code)]
    pub async fn update(
        State(app_state): State<AppState>,
        id: u128,
        Json(update_task): Json<super::UpdateTask>,
    ) -> Result<Json<Task>> {
        app_state.task_manager_model.update(id, update_task).await
    }

    pub async fn delete(
        State(app_state): State<AppState>,
        Path(id): Path<u128>,
    ) -> Result<Json<Task>> {
        app_state.task_manager_model.delete(id).await
    }
}
