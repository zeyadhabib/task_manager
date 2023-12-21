use std::sync::{Arc, Mutex};

use axum::Json;

use super::{ITaskManagerModel, NewTask, Task, UpdateTask};
use crate::common::{errors::Error, Id, Result};

#[derive(Debug, Clone, Default)]
pub struct MockTaskManagerModel {
    pub tasks: Arc<Mutex<Vec<Option<Task>>>>,
}

#[async_trait::async_trait]
impl ITaskManagerModel for MockTaskManagerModel {
    async fn get_all(&self) -> Result<Vec<Task>> {
        let tasks = self.tasks.lock().unwrap();
        let tasks = tasks.iter().filter_map(|task| task.clone()).collect();
        Ok(tasks)
    }

    async fn get(&self, id: Id) -> Result<Vec<Task>> {
        let mut tasks = self.tasks.lock().unwrap();
        let task = tasks.get_mut(id as usize);
        match task {
            Some(task) => Ok(vec![task.clone().unwrap()]),
            None => Err(Error::TaskNotFound { id }),
        }
    }

    async fn create(&self, new_task: NewTask) -> Result<Json<Task>> {
        let mut tasks = self.tasks.lock().unwrap();
        let id = tasks.len() as Id;
        let task = Task {
            id,
            title: new_task.title,
            description: new_task.description,
            status: super::TaskStatus::Todo,
        };
        tasks.push(Some(task.clone()));
        Ok(Json(task))
    }

    async fn update(&self, id: Id, update_task: UpdateTask) -> Result<Json<Task>> {
        let mut tasks = self.tasks.lock().unwrap();
        let task = tasks.get_mut(id as usize);
        match task {
            Some(task) => {
                let task = task.as_mut().unwrap();
                if let Some(title) = update_task.title {
                    task.title = title;
                }
                if let Some(description) = update_task.description {
                    task.description = description;
                }
                task.status = update_task.status;
                Ok(Json(task.clone()))
            }
            None => Err(Error::TaskNotFound { id }),
        }
    }

    async fn delete(&self, id: Id) -> Result<Json<Task>> {
        let mut tasks = self.tasks.lock().unwrap();
        let task = tasks.get_mut(id as usize).and_then(|task| task.take());
        match task {
            Some(task) => Ok(Json(task)),
            None => Err(Error::TaskNotFound { id }),
        }
    }
}
