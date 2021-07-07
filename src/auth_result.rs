use actix_web::{http::StatusCode, HttpResponse};
use std::fmt;

/// Shortcut to `Result<T, AuthError>` for model internals
pub type AuthResult<T, Id> = Result<T, AuthError<Id>>;

/// See [GetErrorCode::code] for more information
pub trait GetErrorCode {
    /// Gets a http response code as `u16` for a given error instance
    fn code(&self) -> StatusCode;
}

#[derive(Debug, PartialEq)]
pub struct AuthError<Id: fmt::Display + Clone> {
    pub kind: AuthErrorKind,
    pub id: Option<Id>,
}

impl<Id: fmt::Display + Clone> AuthError<Id> {
    pub fn new(kind: impl Into<AuthErrorKind>, id: impl Into<Option<Id>>) -> Self {
        Self {
            kind: kind.into(),
            id: id.into(),
        }
    }
}

impl<Id: fmt::Display + Clone> fmt::Display for AuthError<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.id.clone() {
            Some(id) => write!(f, "{} ({})", self.kind, id),
            None => write!(f, "{}", self.kind),
        }
    }
}

impl<Id: fmt::Display + Clone> GetErrorCode for AuthError<Id> {
    fn code(&self) -> StatusCode {
        self.kind.code()
    }
}

impl<Id: fmt::Display + Clone> Into<HttpResponse> for AuthError<Id> {
    fn into(self) -> HttpResponse {
        HttpResponse::new(self.code())
            .set_body(actix_web::dev::Body::from_message(format!("{}", self)))
    }
}

/// Encompasses errors stemming from model manipulation
#[derive(Debug, PartialEq)]
pub enum AuthErrorKind {
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

impl fmt::Display for AuthErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthErrorKind::OrgError(err) => write!(f, "{} for org", err),
            AuthErrorKind::UserError(err) => write!(f, "{} for user", err),
            AuthErrorKind::ProviderError(err) => write!(f, "{} for provider", err),
            AuthErrorKind::DatabaseError(err) => write!(f, "Database error, {}", err),
            AuthErrorKind::UnknownError(Some(err)) => write!(f, "Unknown error, {}", err),
            AuthErrorKind::UnknownError(None) => write!(f, "Unknown error, no info known"),
        }
    }
}

impl GetErrorCode for AuthErrorKind {
    fn code(&self) -> StatusCode {
        match self {
            AuthErrorKind::OrgError(err) => err.code(),
            AuthErrorKind::ProviderError(err) => err.code(),
            AuthErrorKind::UserError(err) => err.code(),
            AuthErrorKind::DatabaseError(_) | AuthErrorKind::UnknownError(_) => {
                StatusCode::from_u16(500).unwrap()
            }
        }
    }
}

/// Specific errors for the [Org] model
#[derive(Debug, PartialEq)]
pub enum OrgError {}

impl fmt::Display for OrgError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("org error string formatting")
    }
}

impl From<OrgError> for AuthErrorKind {
    fn from(err: OrgError) -> Self {
        AuthErrorKind::OrgError(err)
    }
}

impl GetErrorCode for OrgError {
    fn code(&self) -> StatusCode {
        todo!("org response code")
    }
}

/// Specific errors for the [Provider] model
#[derive(Debug, PartialEq)]
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

impl From<ProviderError> for AuthErrorKind {
    fn from(err: ProviderError) -> Self {
        AuthErrorKind::ProviderError(err)
    }
}

impl GetErrorCode for ProviderError {
    fn code(&self) -> StatusCode {
        todo!("provider response code")
    }
}

/// Specific errors for the [User] model
#[derive(Debug, PartialEq)]
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

impl From<UserError> for AuthErrorKind {
    fn from(err: UserError) -> Self {
        AuthErrorKind::UserError(err)
    }
}

impl GetErrorCode for UserError {
    fn code(&self) -> StatusCode {
        StatusCode::from_u16(match self {
            &UserError::NameTooLong => 400,
        })
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn name_too_long_user() {
        let mut err: AuthError<Uuid> = AuthError::new(UserError::NameTooLong, None);
        assert_eq!(format!("{}", err), "Name is too long for user");

        let uuid = Uuid::new_v4();
        err.id = Some(uuid.clone());
        assert_eq!(
            format!("{}", err),
            format!("Name is too long for user ({})", uuid)
        );
    }
}
