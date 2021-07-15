mod base;
mod org;
mod provider;
mod user_provider;

/// Initializes all routes
pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(base::index);
    cfg.service(user_provider::post);
    cfg.service(user_provider::get);
    cfg.service(user_provider::patch);
    cfg.service(user_provider::delete);
    cfg.service(user_provider::authorise);
    cfg.service(user_provider::refresh);
    cfg.service(org::delete);
}
