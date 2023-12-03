pub use crate::error::ApiError;

pub type Result<T> = core::result::Result<T, ApiError>;
