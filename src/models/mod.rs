//! Contains models for all database interactions

mod user_basic;

pub use user_basic::{AuthBasic, UserBasic};

use std::fmt;

/// Shortcut to `Result<T, ModelError>` for model internals
pub type ModelResult<T> = Result<T, ModelError>;

/// Encompasses errors stemming from model manipulation
#[derive(Debug)]
pub enum ModelError {
    /// See [UserError] for documentation
    UserError(UserError),
    /// Database error whilst handling a request, should not be exposed publicly
    DatabaseError(String),
    /// Unknown error occurred with optional extra info given, should not be
    /// exposed publicly
    UnknownError(Option<String>),
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModelError::UserError(err) => write!(f, "{}", err),
            ModelError::DatabaseError(_) => write!(f, "Database error"),
            &ModelError::UnknownError(_) => write!(f, "Unknown error"),
        }
    }
}

/// Specific errors for the [User] model
#[derive(Debug)]
pub enum UserError {}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("User error string formatting")
    }
}

impl From<UserError> for ModelError {
    fn from(err: UserError) -> Self {
        ModelError::UserError(err)
    }
}
