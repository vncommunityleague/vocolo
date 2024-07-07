mod auth;
pub use auth::*;
mod error;
pub use error::*;

pub type Result<T> = std::result::Result<T, Error>;
