//! See [AuthExt] for documentation

use chrono::prelude::*;
use uuid::Uuid;

/// Model for external OAuth services which can link to users
pub struct AuthExt {
    /// Serial primary key id
    pub id: i32,
    /// If provided a name detailing the service this is from
    pub provider: Option<String>,
    /// Access token created
    pub token_access: String,
    /// If provided a token used to ask for a new [AuthExt::token_access] if expired
    pub token_refresh: Option<String>,
    /// If provided an expiry date of this token
    pub expiry: Option<DateTime<Utc>>,
    /// Timestamp of creation
    pub created: DateTime<Utc>,
    /// Foreign key to the user this instance belongs to
    pub user_id: Uuid,
}
