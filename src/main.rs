use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use log::info;

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
            .configure(api::routes::init_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
