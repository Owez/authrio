//! See [UserProvider] for documentation

use crate::crypto::gen_id;
use chrono::prelude::*;
use sqlx::FromRow;

/// Model for users in the scope of a provider, for external logins
#[derive(FromRow, Debug, PartialEq, Eq, Clone)]
pub struct UserProvider {
    /// Randomly generated integer primary key
    pub id: i32,
    /// Access token to use in conjunction with the provider
    pub token_access: String,
    /// Optional refresh token for easy refreshing
    pub token_refresh: Option<String>,
    /// Optional expiry date of [UserProvider::token_access] if provided
    pub token_expires: Option<DateTime<Utc>>, 
    /// Foreign key to the [Provider::id](super::Provider::id) field
    pub provider_id: i32,
    /// Timestamp of creation
    pub created: DateTime<Utc>,
}

impl UserProvider {
    /// Creates new [UserProvider] from given provider
    pub fn new(token_access: impl Into<String>, token_refresh: impl Into<Option<String>>, token_expires: impl Into<Option<DateTime<Utc>>>, provider_id: i32) -> Self {
        Self {
            id: gen_id(),
            token_access: token_access.into(),
            token_refresh: token_refresh.into(),
            token_expires: token_expires.into(),
            provider_id,
            created: Utc::now(),
        }
    }
}
