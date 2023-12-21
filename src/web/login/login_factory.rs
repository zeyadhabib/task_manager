use axum::Json;
use serde_json::Value;
use tower_cookies::Cookies;

use super::{mock_login::MockLoginApi, ILoginApi, LoginPayload};
use crate::common::Result;

#[derive(Clone)]
pub struct LoginApi {
    login_api: Box<dyn ILoginApi>,
}

#[async_trait::async_trait]
impl ILoginApi for LoginApi {
    async fn login(
        &self,
        cookies: Option<Cookies>,
        login_payload: LoginPayload,
    ) -> Result<Json<Value>> {
        self.login_api.login(cookies, login_payload).await
    }
}

impl LoginApi {
    pub fn new() -> Self {
        Self {
            login_api: Box::new(MockLoginApi::default()),
        }
    }
}

pub async fn login(
    cookies: Option<Cookies>,
    Json(login_payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
    let api = LoginApi::new();
    api.login(cookies, login_payload).await
}
