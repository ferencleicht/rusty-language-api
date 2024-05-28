use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use log::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::v1::openapi::ApiDoc;

mod api;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let pool = db::connection::establish_pooled_connection();

    db::migrations::run_migrations(&mut pool.get().unwrap());

    info!("Starting server...");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(api::v1::routes::init_routes)
            .service(SwaggerUi::new("/docs/{_:.*}").url("/openapi.json", ApiDoc::openapi()))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
