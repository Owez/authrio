//! See [Simple] for documentation

use super::{IntoModel, ModelError, ModelResult};
use crate::crypto::Hash;
use chrono::prelude::*;
use sqlx::FromRow;
use std::convert::TryInto;
use uuid::Uuid;

/// Model for all basic simple (user) related storage
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Simple {
    /// Unique simple (user) primary key uuid
    pub id: Uuid,
    /// Hashed password and salt contained in the [struct@Hash] structure
    pub password: Hash,
    /// Timestamp of creation
    pub created: DateTime<Utc>,
}

/// Internal sqlx mapping for the [Simple] model
#[derive(FromRow)]
struct SimpleInternal {
    id: Uuid,
    pw_hash: Vec<u8>,
    pw_salt: Vec<u8>,
    pw_created: DateTime<Utc>,
    created: DateTime<Utc>,
}

impl IntoModel<Simple> for SimpleInternal {
    fn into(self) -> ModelResult<Simple> {
        Ok(Simple {
            id: self.id,
            password: Hash {
                inner: self.pw_hash,
                salt: match self.pw_salt.try_into() {
                    Ok(salt) => salt,
                    Err(_) => {
                        return Err(ModelError::DatabaseError(format!(
                            "Salt length invalid for simple (user) {}",
                            self.id
                        )))
                    }
                },
                created: self.pw_created,
            },
            created: self.created,
        })
    }
}
