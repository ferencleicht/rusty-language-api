use actix_web::{web, App, HttpServer};
mod api;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::connection::establish_pooled_connection();

    db::migrations::run_migrations(&mut pool.get().unwrap());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(api::routes::init_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
