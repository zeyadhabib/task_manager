mod login_factory;
mod mock_login;

use axum::{routing::post, Json, Router};
use dyn_clone::DynClone;
use serde::Deserialize;
use serde_json::Value;
use tower_cookies::Cookies;

use self::login_factory::login;
use crate::common::Result;

const LOGIN_ROUTE: &'static str = "/login";

#[derive(Clone, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[async_trait::async_trait]
pub trait ILoginApi: DynClone + Send + Sync {
    async fn login(
        &self,
        cookies: Option<Cookies>,
        login_payload: LoginPayload,
    ) -> Result<Json<Value>>;
}

dyn_clone::clone_trait_object!(ILoginApi);

pub fn get_router() -> Router {
    Router::new().route(LOGIN_ROUTE, post(login))
}
