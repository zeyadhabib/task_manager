pub mod errors;

pub type Result<T> = std::result::Result<T, errors::Error>;

pub type Id = u128;
