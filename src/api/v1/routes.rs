use crate::api::v1::handlers::{
    create_dictionary, delete_dictionary, get_dictionaries, get_dictionary_by_id,
};
use actix_web::web::{self};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1").service(
            web::scope("/dictionaries")
                .route("", web::get().to(get_dictionaries))
                .route("", web::post().to(create_dictionary))
                .route("{id}", web::get().to(get_dictionary_by_id))
                .route("{id}", web::delete().to(delete_dictionary)),
        ),
    );
}
