//! See [SimpleAuth] for documentation

use chrono::prelude::*;
use sqlx::FromRow;
use uuid::Uuid;

/// Contains authentication constructs for a [Simple](super::Simple) model
#[derive(FromRow, Debug, PartialEq, Eq, Clone)]
pub struct SimpleAuth {
    /// Unique simple auth primary key uuid which links directly to a
    /// [Simple::id](super::Simple::id) element
    pub id: Uuid,
    /// Access token used
    pub token_access: String,
    /// Optional refresh token is [SimpleAuth::provider] needs
    pub token_refresh: Option<String>,
    /// Optional expiry time of [SimpleAuth::token_access] if needed
    pub expiry: Option<DateTime<Utc>>,
    /// Optional name pointing to a [super::Provider] indicating if this is an
    /// external application or local
    pub provider: Option<String>,
}

impl SimpleAuth {
    /// Checks if this instance is for local or for a provider
    pub fn local(&self) -> bool {
        self.provider.is_none()
    }
}
