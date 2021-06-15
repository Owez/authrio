//! See [UserBasic] or [AuthBasic] for documentation

use super::{ModelError, ModelResult};
use crate::crypto::Hash;
use chrono::prelude::*;
use sqlx::FromRow;
use std::convert::TryInto;
use uuid::Uuid;

/// Model for all basic user related storage
pub struct UserBasic {
    /// Unique user primary key uuid
    pub id: Uuid,
    /// Hashed password and salt contained in the [Hash] structure
    pub password: Hash,
    /// Optional auth information (token, etc) if created
    pub auth: Option<AuthBasic>,
    /// Timestamp of creation
    pub created: DateTime<Utc>,
}

/// Contains user-specific authorization information
pub struct AuthBasic {
    /// Access token created
    pub token: String,
    /// Expiry date of token
    pub expiry: DateTime<Utc>,
}

/// Internal sqlx mapping for the [UserBasic] model
#[derive(FromRow)]
struct UserBasicInternal {
    id: Uuid,
    pw_hash: Vec<u8>,
    pw_salt: Vec<u8>,
    pw_created: DateTime<Utc>,
    token_access: Option<String>,
    expiry: Option<DateTime<Utc>>,
    created: DateTime<Utc>,
}

impl UserBasicInternal {
    /// Attempts to convert [UserBasicInternal] from sqlx into a final well-kept [UserBasic]
    fn into_user(self) -> ModelResult<UserBasic> {
        Ok(UserBasic {
            id: self.id,
            password: Hash {
                inner: self.pw_hash,
                salt: match self.pw_salt.try_into() {
                    Ok(salt) => salt,
                    Err(_) => {
                        return Err(ModelError::DatabaseError(format!(
                            "Salt length invalid for user {}",
                            self.id
                        )))
                    }
                },
                created: self.pw_created,
            },
            auth: match self.token_access {
                Some(token) => match self.expiry {
                    Some(expiry) => Ok(Some(AuthBasic { token, expiry })),
                    None => Err(ModelError::DatabaseError(format!(
                        "Only access token exists for user {}",
                        self.id
                    ))),
                },
                None => match self.expiry {
                    Some(_) => Err(ModelError::DatabaseError(format!(
                        "Only expiry date exists for user {}",
                        self.id
                    ))),
                    None => Ok(None),
                },
            }?,
            created: self.created,
        })
    }
}
