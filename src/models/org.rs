//! See [Org] for documentation

use super::IntoModel;
use crate::crypto::Hash;
use crate::{AuthError, AuthErrorKind, AuthResult, Config, OrgError, UserError};
use actix_web_httpauth::extractors::basic::BasicAuth;
use chrono::prelude::*;
use sqlx::{FromRow, PgPool};
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

impl Org {
    /// Creates a new [Org] and validates contents, does not add to db
    pub fn new(
        config: &Config,
        name: impl Into<String>,
        password: impl AsRef<[u8]>,
    ) -> AuthResult<Self, Uuid> {
        let id = Uuid::new_v4();

        Ok(Self {
            id: id.clone(),
            name: validate_name(name.into(), &id)?,
            password: match Hash::from_password(config, password) {
                Ok(hash) => hash,
                Err(err) => return Err(AuthError::new(AuthErrorKind::HashError(err), id)),
            },
            created: Utc::now(),
        })
    }

    /// Get an organisation from provided basic auth
    pub fn from_auth(_auth: BasicAuth) -> AuthResult<Self, Uuid> {
        todo!("get org from auth")
    }

    /// Authorizes organisation and deletes all in one
    pub fn auth_delete(_pool: &PgPool, _auth: BasicAuth) -> AuthResult<(), Uuid> {
        todo!("get org from auth and delete at same time")
    }

    /// Authorizes organisation and patches with given values all in one
    pub fn auth_patch(
        _pool: &PgPool,
        _auth: BasicAuth,
        new_name: Option<String>,
        new_password: Option<String>,
    ) -> AuthResult<(), Uuid> {
        let mut changed = false;

        if let Some(name) = new_name {
            changed = true;
            todo!("name")
        }

        if let Some(password) = new_password {
            changed = true;
            todo!("password")
        }

        if !changed {
            Err(AuthError::new(OrgError::NothingToPatch, Uuid::new_v4())) // TODO: put auth in uuid slot
        } else {
            Ok(())
        }
    }
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
    fn into_model(self) -> AuthResult<Org, Uuid> {
        Ok(Org {
            id: self.id,
            name: self.name,
            password: Hash {
                inner: self.pw_hash,
                salt: match self.pw_salt.try_into() {
                    Ok(salt) => salt,
                    Err(_) => {
                        return Err(AuthError::new(
                            AuthErrorKind::DatabaseError("salt length invalid for org".to_string()),
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
    fn into_model(self) -> AuthResult<OrgInternal, Uuid> {
        Ok(OrgInternal {
            id: self.id,
            name: validate_name(self.name, &self.id)?,
            pw_hash: self.password.inner,
            pw_salt: self.password.salt.to_vec(),
            pw_created: self.password.created,
            created: self.created,
        })
    }
}

/// Validates [Org::name]/[OrgInternal::name] element
fn validate_name(name: String, id: &Uuid) -> AuthResult<String, Uuid> {
    if name.len() > MAX_NAME {
        Err(AuthError::new(UserError::NameTooLong, *id))
    } else {
        Ok(name)
    }
}
