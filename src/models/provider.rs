//! See [Provider] for documentation

use crate::{AuthError, AuthResult, ProviderError};
use chrono::prelude::*;
use sqlx::FromRow;
use uuid::Uuid;

/// Maximum allowed size for general medium strings
const MAX_MED: usize = 64;

/// Maximum allowed uri size
const MAX_URI: usize = 2000;

/// Provider explaining the relationship to a service from an org
#[derive(FromRow, Debug, PartialEq, Eq, Clone)]
pub struct Provider {
    /// The `client_id` unique primary key id
    pub id: String,
    /// Backchannel `client_secret` identifier
    pub secret: String,
    /// Domain of provider to connect to
    pub domain: String,
    /// Optional but recommended uri to redirect to
    pub redirect_uri: Option<String>,
    /// Scope(s) for oauth
    pub scope: Option<String>,
    /// The [Org](super::Org) this is related to
    pub org_id: Uuid,
    /// Timestamp of creation
    pub created: DateTime<Utc>,
}

impl Provider {
    /// Creates a new [Provider] and validates contents, does not add to db
    pub fn new<S: Into<String>, Os: Into<Option<String>>>(
        id: S,
        secret: S,
        domain: S,
        redirect_uri: Os,
        scope: Os,
        org_id: Uuid,
    ) -> AuthResult<Self, String> {
        // create
        let got = Self {
            id: id.into(),
            secret: secret.into(),
            domain: domain.into(),
            redirect_uri: redirect_uri.into(),
            scope: scope.into(),
            org_id: org_id.into(),
            created: Utc::now(),
        };

        // validate
        validate(&got.id, MAX_MED, ProviderError::IdTooLong, &got.id)?;
        validate(&got.secret, MAX_MED, ProviderError::SecretTooLong, &got.id)?;
        validate(&got.domain, MAX_URI, ProviderError::DomainTooLong, &got.id)?;

        if let Some(val) = &got.redirect_uri {
            validate(val, MAX_URI, ProviderError::RedirectUriTooLong, &got.id)?;
        }

        if let Some(val) = &got.scope {
            validate(val, MAX_MED, ProviderError::ScopeTooLong, &got.id)?;
        }

        // return
        Ok(got)
    }
}

/// Validates a section or errors
fn validate(part: &str, max: usize, err: ProviderError, id: &str) -> AuthResult<(), String> {
    if part.len() > max {
        Err(AuthError::new(err, id.to_string()))
    } else {
        Ok(())
    }
}
