use axum::Json;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use super::{ILoginApi, LoginPayload};
use crate::common::{errors::Error, Result};

#[derive(Debug, Clone, Default)]
pub struct MockLoginApi {}

#[async_trait::async_trait]
impl ILoginApi for MockLoginApi {
    async fn login(
        &self,
        cookies: Option<Cookies>,
        login_payload: LoginPayload,
    ) -> Result<Json<Value>> {
        let username = login_payload.username;
        let password = login_payload.password;
        
        if username == "admin" && password == "admin" {
            match cookies {
                Some(cookies) => cookies.add(Cookie::new("auth_token", "admin.exp.sign")),
                None => {}
            }
            Ok(Json(json!(
                {
                    "sucess": true,
                }
            )))
        } else {
            Err(Error::LoginFailed)
        }
    }
}
