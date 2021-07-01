//! Request flows for every server request once warp has found it and parsed
//! http headers
//!
//! All functions should correspond with a single warp route in `main.rs` and
//! are imported with `flow::name` where the `name` is the route's name

use crate::{config::Config, schemas::UserProviderId, Route, RouteResult};

pub fn user_provider_delete(config: &Config, id: UserProviderId) -> RouteResult<String> {
    let uuid = id.route()?;

    todo!("get org basic auth and delete from db")
}
