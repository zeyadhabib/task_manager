use axum::response::IntoResponse;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    LoginFailed,
    AuthFailInvalidToken,
    AuthFailNoAuthTokenCookie,
    TaskNotFound { id: u128 },
    TaskAlreadyExists { id: u128 },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::LoginFailed => {
                (axum::http::StatusCode::UNAUTHORIZED, "LOGIN_FAILED").into_response()
            }
            Self::AuthFailNoAuthTokenCookie => {
                (axum::http::StatusCode::UNAUTHORIZED, "AUTH_FAIL_NO_AUTH_TOKEN_COOKIE")
                    .into_response()
            }
            Self::AuthFailInvalidToken => {
                (axum::http::StatusCode::UNAUTHORIZED, "AUTH_FAIL_INVALID_AUTH_TOKEN_COOKIE")
                    .into_response()
            }
            Self::TaskNotFound { id } => {
                (axum::http::StatusCode::NOT_FOUND, format!("TASK_NOT_FOUND: {}", id))
                    .into_response()
            }
            _ => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_SERVER_ERROR",
            )
                .into_response(),
        }
    }
}
