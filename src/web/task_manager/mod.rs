use axum::{
    routing::{delete, get, post},
    Router,
};

use self::model::{app_state, model_handler::ModelHandler};

pub mod model;

pub fn get_router() -> Router {
    let app_state = app_state::AppState::new();
    Router::new().nest(
        "/task_manager",
        Router::new()
            .route("/", get(ModelHandler::get_all))
            .route("/:id", get(ModelHandler::get))
            .route("/", post(ModelHandler::create))
            .route("/:id", delete(ModelHandler::delete))
            .with_state(app_state),
    )
}
