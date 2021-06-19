//! See [UserProvider] for documentation

use crate::crypto::gen_id;
use chrono::prelude::*;
use sqlx::FromRow;

/// Model for users in the scope of a provider, for external logins
#[derive(FromRow, Debug, PartialEq, Eq, Clone)]
pub struct UserProvider {
    /// Randomly generated integer primary key
    pub id: i32,
    /// Foreign key to the [Provider::id](super::Provider::id) field
    pub provider_id: i32,
    /// Timestamp of creation
    pub created: DateTime<Utc>,
}

impl UserProvider {
    /// Creates new [UserProvider] from given provider
    pub fn new(provider_id: i32) -> Self {
        Self {
            id: gen_id(),
            provider_id,
            created: Utc::now(),
        }
    }
}
