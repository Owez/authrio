//! Defines schemas for query deserializations

use crate::{Route, RouteError, RouteResult};
use serde::Deserialize;
use uuid::Uuid;

/// Allows fetching of a given user by their id with verification with the [Route] implementation
#[derive(Debug, Deserialize)]
pub struct UserProviderId {
    pub id: String,
}

impl Route<Uuid> for UserProviderId {
    fn route(self) -> RouteResult<Uuid> {
        Uuid::parse_str(&self.id).map_err(|_| RouteError::BadUuid(self.id))
    }
}
