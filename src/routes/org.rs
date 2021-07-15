use crate::{models::Org, AuthError, Config, OrgError};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
struct OrgPost {
    name: String,
    password: String,
}

// TODO: finish
#[post("/")]
async fn post(
    _pool: web::Data<PgPool>,
    config: web::Data<Config>,
    data: web::Json<OrgPost>,
) -> impl Responder {
    match Org::new(config.get_ref(), data.name.clone(), data.password.clone()) {
        Ok(_org) => HttpResponse::ServiceUnavailable().body("add org to db"),
        Err(err) => err.into(),
    }
}

/// TODO: finish
#[get("/<id>")]
async fn get(_pool: web::Data<PgPool>, query_id: web::Query<String>) -> impl Responder {
    let _id = match Uuid::parse_str(&query_id.0)
        .map_err(|err| AuthError::new(OrgError::InvalidUuidQuery(err), query_id.0))
    {
        Ok(id) => id,
        Err(err) => return err.into(),
    };

    HttpResponse::ServiceUnavailable().body("get org by id")
}

#[derive(Deserialize)]
struct OrgPatch {
    name: Option<String>,
    password: Option<String>,
}

#[patch("/")]
async fn patch(
    pool: web::Data<PgPool>,
    org_auth: BasicAuth,
    data: web::Json<OrgPatch>,
) -> impl Responder {
    match Org::auth_patch(
        pool.get_ref(),
        org_auth,
        data.name.clone(),
        data.password.clone(),
    ) {
        Ok(()) => HttpResponse::Ok().body("organisation patched successfully"),
        Err(err) => err.into(),
    }
}

#[delete("/")]
async fn delete(pool: web::Data<PgPool>, org_auth: BasicAuth) -> impl Responder {
    match Org::auth_delete(pool.get_ref(), org_auth) {
        Ok(()) => HttpResponse::Ok().body("organisation deleted successfully"),
        Err(err) => err.into(),
    }
}
