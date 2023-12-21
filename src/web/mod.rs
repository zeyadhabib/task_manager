use axum::Router;

pub mod login;
pub mod task_manager;

pub fn get_router() -> Router {
    Router::new()
        .nest("/login", login::get_router())
        .nest("/", task_manager::get_router().merge(login::get_router()))
}
