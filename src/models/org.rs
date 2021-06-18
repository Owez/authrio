//! See [Org] for documentation

use super::{IntoModel, ModelError, ModelErrorKind, ModelResult, UserError};
use crate::crypto::Hash;
use chrono::prelude::*;
use sqlx::FromRow;
use std::convert::TryInto;
use uuid::Uuid;

/// Max length for [Org::name] before erroring
const MAX_NAME: usize = 32;

/// Top-level organisation which groups users into logical units
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Org {
    /// Unique org primary key uuid
    pub id: Uuid,
    /// Org name to show publicly for ease-of-use
    pub name: String,
    /// Hashed password and salt contained in the [struct@Hash] structure
    pub password: Hash,
    /// Timestamp of creation
    pub created: DateTime<Utc>,
}

/// Internal sqlx mapping for the [Org] model
#[derive(FromRow)]
struct OrgInternal {
    id: Uuid,
    name: String,
    pw_hash: Vec<u8>,
    pw_salt: Vec<u8>,
    pw_created: DateTime<Utc>,
    created: DateTime<Utc>,
}

impl IntoModel<Org, Uuid> for OrgInternal {
    fn into(self) -> ModelResult<Org, Uuid> {
        Ok(Org {
            id: self.id,
            name: self.name,
            password: Hash {
                inner: self.pw_hash,
                salt: match self.pw_salt.try_into() {
                    Ok(salt) => salt,
                    Err(_) => {
                        return Err(ModelError::new(
                            ModelErrorKind::DatabaseError(
                                "salt length invalid for org".to_string(),
                            ),
                            self.id,
                        ))
                    }
                },
                created: self.pw_created,
            },
            created: self.created,
        })
    }
}

impl IntoModel<OrgInternal, Uuid> for Org {
    fn into(self) -> ModelResult<OrgInternal, Uuid> {
        Ok(OrgInternal {
            id: self.id,
            name: verify_name(self.name, &self.id)?,
            pw_hash: self.password.inner,
            pw_salt: self.password.salt.to_vec(),
            pw_created: self.password.created,
            created: self.created,
        })
    }
}

/// Verifies [Org::name]/[OrgInternal::name] element
fn verify_name(name: String, id: &Uuid) -> ModelResult<String, Uuid> {
    if name.len() > MAX_NAME {
        Err(ModelError::new(UserError::NameTooLong, *id))
    } else {
        Ok(name)
    }
}
