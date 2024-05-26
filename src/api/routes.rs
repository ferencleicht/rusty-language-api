use crate::api::handlers::{create_set, get_set_by_id, get_sets};
use actix_web::web::{self};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/sets")
                .route("", web::get().to(get_sets))
                .route("", web::post().to(create_set))
                .route("{id}", web::get().to(get_set_by_id)),
        ),
    );
}
