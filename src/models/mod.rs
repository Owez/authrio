//! Contains models for all database interactions

mod org;
mod provider;
mod simple;
mod simple_auth;

pub use org::Org;
pub use provider::Provider;
pub use simple::Simple;
pub use simple_auth::SimpleAuth;

use std::fmt;

/// Shortcut to `Result<T, ModelError>` for model internals
pub type ModelResult<T> = Result<T, ModelError>;

/// Simple trait defining a `.into()` call for models to be made into other models
pub trait IntoModel<T> {
    /// Converts into other model and verifies information
    fn into(self) -> ModelResult<T>;
}

/// Encompasses errors stemming from model manipulation
#[derive(Debug)]
pub enum ModelError {
    /// See [OrgError] for documentation
    OrgError(OrgError),
    /// See [SimpleError] for documentation
    SimpleError(SimpleError),
    /// Database error whilst handling a request, should not be exposed publicly
    DatabaseError(String),
    /// Unknown error occurred with optional extra info given, should not be
    /// exposed publicly
    UnknownError(Option<String>),
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModelError::OrgError(err) => write!(f, "{}", err),
            ModelError::SimpleError(err) => write!(f, "{}", err),
            ModelError::DatabaseError(_) => write!(f, "Database error"),
            ModelError::UnknownError(_) => write!(f, "Unknown error"),
        }
    }
}

/// Specific errors for the [Org] model
#[derive(Debug)]
pub enum OrgError {}

impl fmt::Display for OrgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Org error string formatting")
    }
}

impl From<OrgError> for ModelError {
    fn from(err: OrgError) -> Self {
        ModelError::OrgError(err)
    }
}

/// Specific errors for the [Simple] model
#[derive(Debug)]
pub enum SimpleError {}

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Simple error string formatting")
    }
}

impl From<SimpleError> for ModelError {
    fn from(err: SimpleError) -> Self {
        ModelError::SimpleError(err)
    }
}
