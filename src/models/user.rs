//! See [User] or [UserAuth] for documentation

use chrono::prelude::*;
use uuid::Uuid;
use crate::crypto::Hash;

/// Model for all user related storage
pub struct User {
    /// Unique user primary key uuid
    pub id: Uuid,
    /// Hashed password and salt contained in the [Hash] structure
    pub password: Hash,
    /// Optional auth information (token, etc) if created
    pub auth: Option<UserAuth>,
    /// Timestamp of creation
    pub created: DateTime<Utc>,
}

/// Contains user-specific authorization information
pub struct UserAuth {
    /// Access token created
    pub token_access: String,
    /// Expiry date of token
    pub expiry: DateTime<Utc>,
}
