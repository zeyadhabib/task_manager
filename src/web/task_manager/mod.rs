pub mod model;
pub mod mw_require_auth;

use axum::{
    routing::{delete, get, post},
    Router, middleware::from_fn,
};

use self::model::{app_state, model_handler::ModelHandler};

pub fn get_router() -> Router {
    let app_state = app_state::AppState::new();
    Router::new().nest(
        "/task_manager",
        Router::new()
            .route("/", get(ModelHandler::get_all))
            .route("/:id", get(ModelHandler::get))
            .route("/", post(ModelHandler::create))
            .route("/:id", delete(ModelHandler::delete))
            .with_state(app_state)
            .route_layer(from_fn(mw_require_auth::mw_require_auth)),
    )
}
