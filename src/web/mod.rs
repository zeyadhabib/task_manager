use axum::Router;
use tower_cookies::CookieManagerLayer;

pub mod login;
pub mod task_manager;

pub fn get_router() -> Router {
    Router::new()
        .nest("/", task_manager::get_router().merge(login::get_router()))
        .layer(CookieManagerLayer::new())
}
