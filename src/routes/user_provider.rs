use crate::models::UserProvider;
use crate::{models::Org, Config};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use sqlx::PgPool;

// TODO: make user cred header
// TODO: figure out error handling

/// TODO: finish
#[post("/")]
pub async fn post(_pool: web::Data<PgPool>, _config: web::Data<Config>) -> impl Responder {
    // add optional data argument which then directly adds to db instead of redirecting to a provider?
    HttpResponse::ServiceUnavailable().body("post user provider")
}

/// TODO: finish
#[get("/")]
pub async fn get(
    _pool: web::Data<PgPool>,
    _config: web::Data<Config>,
    org_auth: BasicAuth,
) -> impl Responder {
    let _org = match Org::from_auth(org_auth) {
        Ok(val) => val,
        Err(err) => return err.into(),
    };

    HttpResponse::ServiceUnavailable().body("get user provider")
}

/// TODO: finish
#[patch("/")]
pub async fn patch(_pool: web::Data<PgPool>, _config: web::Data<Config>) -> impl Responder {
    HttpResponse::ServiceUnavailable().body("patch user provider")
}

/// TODO: figure out how to auth this
/// TODO: finish
#[delete("/")]
pub async fn delete(
    _pool: web::Data<PgPool>,
    _config: web::Data<Config>,
    org_auth: BasicAuth,
) -> impl Responder {
    let _org = match Org::from_auth(org_auth) {
        Ok(val) => val,
        Err(err) => return err.into(),
    };

    HttpResponse::ServiceUnavailable().body("delete user provider")
}

/// TODO: finish
#[post("/auth")]
pub async fn authorise(_pool: web::Data<PgPool>, _config: web::Data<Config>) -> impl Responder {
    HttpResponse::ServiceUnavailable().body("authorise user provider")
}

/// TODO: finish
#[post("/refresh")]
pub async fn refresh(_pool: web::Data<PgPool>, _config: web::Data<Config>) -> impl Responder {
    HttpResponse::ServiceUnavailable().body("refresh user provider's token")
}
