//! Contains models for all database interactions

mod org;
mod provider;

pub use org::Org;
pub use provider::Provider;

use std::fmt;

/// Shortcut to `Result<T, ModelError>` for model internals
pub type ModelResult<T, Id> = Result<T, ModelError<Id>>;

/// Simple trait defining a `.into()` call for models to be made into other models
pub trait IntoModel<T, Id: fmt::Display + Clone> {
    /// Converts into other model and verifies information
    fn into(self) -> ModelResult<T, Id>;
}

pub struct ModelError<Id: fmt::Display + Clone> {
    pub kind: ModelErrorKind,
    pub id: Option<Id>,
}

impl<Id: fmt::Display + Clone> ModelError<Id> {
    pub fn new(kind: impl Into<ModelErrorKind>, id: impl Into<Option<Id>>) -> Self {
        Self {
            kind: kind.into(),
            id: id.into(),
        }
    }
}

impl<Id: fmt::Display + Clone> fmt::Display for ModelError<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.id.clone() {
            Some(id) => write!(f, "{} ({})", self.kind, id),
            None => write!(f, "{}", self.kind),
        }
    }
}

/// Encompasses errors stemming from model manipulation
#[derive(Debug)]
pub enum ModelErrorKind {
    /// See [OrgError] for documentation
    OrgError(OrgError),
    /// See [ProviderError] for documentation
    ProviderError(ProviderError),
    /// See [UserError] for documentation
    UserError(UserError),
    /// Database error whilst handling a request, should not be exposed publicly
    DatabaseError(String),
    /// Unknown error occurred with optional extra info given, should not be
    /// exposed publicly
    UnknownError(Option<String>),
}

impl fmt::Display for ModelErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModelErrorKind::OrgError(err) => write!(f, "{} for org", err),
            ModelErrorKind::UserError(err) => write!(f, "{} for user", err),
            ModelErrorKind::ProviderError(err) => write!(f, "{} for provider", err),
            ModelErrorKind::DatabaseError(err) => write!(f, "Database error, {}", err),
            ModelErrorKind::UnknownError(Some(err)) => write!(f, "Unknown error, {}", err),
            ModelErrorKind::UnknownError(None) => write!(f, "Unknown error, no info known"),
        }
    }
}

/// Specific errors for the [Org] model
#[derive(Debug)]
pub enum OrgError {}

impl fmt::Display for OrgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Org error string formatting")
    }
}

impl From<OrgError> for ModelErrorKind {
    fn from(err: OrgError) -> Self {
        ModelErrorKind::OrgError(err)
    }
}

/// Specific errors for the [Provider] model
#[derive(Debug)]
pub enum ProviderError {
    IdTooLong,
    SecretTooLong,
    DomainTooLong,
    RedirectUriTooLong,
    ScopeTooLong,
}

impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ProviderError::IdTooLong => "Id (client_id) is too long",
                ProviderError::SecretTooLong => "Secret (client_secret) is too long",
                ProviderError::DomainTooLong => "Domain is too long",
                ProviderError::RedirectUriTooLong => "Redirect URI is too long",
                ProviderError::ScopeTooLong => "Scope is too long",
            }
        )
    }
}

impl From<ProviderError> for ModelErrorKind {
    fn from(err: ProviderError) -> Self {
        ModelErrorKind::ProviderError(err)
    }
}

/// Specific errors for the [User] model
#[derive(Debug)]
pub enum UserError {
    /// User's name is too long
    NameTooLong,
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UserError::NameTooLong => "Name is too long",
            }
        )
    }
}

impl From<UserError> for ModelErrorKind {
    fn from(err: UserError) -> Self {
        ModelErrorKind::UserError(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn name_too_long_user() {
        let mut err: ModelError<Uuid> = ModelError::new(UserError::NameTooLong, None);
        assert_eq!(format!("{}", err), "Name is too long for user");

        let uuid = Uuid::new_v4();
        err.id = Some(uuid.clone());
        assert_eq!(
            format!("{}", err),
            format!("Name is too long for user ({})", uuid)
        );
    }
}
