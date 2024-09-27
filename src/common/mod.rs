pub mod errors;

pub type Result<T> = std::result::Result<T, errors::Error>;

pub type Id = u128;

pub const AUTH_TOKEN: &str = "auth_token";
