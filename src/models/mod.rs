//! Contains models for all database interactions

mod org;
mod provider;
mod user_provider;

pub use org::Org;
pub use provider::Provider;
pub use user_provider::UserProvider;

use crate::AuthResult;
use std::fmt;

/// Simple trait defining a `.into()` call for models to be made into other models
pub trait IntoModel<T, Id: fmt::Display + Clone> {
    /// Converts into other model and verifies information
    fn into_model(self) -> AuthResult<T, Id>;
}
