//! Contains models for all database interactions

mod user;
mod authext;

pub use user::{User, AuthLocal};
pub use authext::AuthExt;
