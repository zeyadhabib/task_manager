use axum::middleware::Next;
use axum_core::{extract::Request, response::Response};
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::common::{errors::Error, Result, AUTH_TOKEN};

pub async fn mw_require_auth(cookies: Cookies, request: Request, next: Next) -> Result<Response> {
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    let (_user_id, _exp, _sign) = auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)?;
    Ok(next.run(request).await)
}

fn parse_token(token: String) -> Result<(u128, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
        .ok_or(Error::AuthFailInvalidToken)?;

    let user_id = user_id
        .parse::<u128>()
        .map_err(|_| Error::AuthFailInvalidToken)?;
    Ok((user_id, exp.to_string(), sign.to_string()))
}
