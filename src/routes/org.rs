use crate::models::Org;
use actix_web::{delete, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use sqlx::PgPool;

#[delete("/")]
async fn delete(pool: web::Data<PgPool>, org_auth: BasicAuth) -> impl Responder {
    match Org::auth_delete(pool.get_ref(), org_auth) {
        Ok(()) => HttpResponse::Ok().body("organisation deleted successfully"),
        Err(err) => err.into(),
    }
}
