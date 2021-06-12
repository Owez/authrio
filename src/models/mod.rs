//! Contains models for all database interactions

mod user;
mod extauth;

pub(crate) use user::{User, UserAuth};
pub(crate) use extauth::ExtAuth;
