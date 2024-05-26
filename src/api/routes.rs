use crate::api::handlers::get_sets;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").route("/sets", web::get().to(get_sets)));
}
