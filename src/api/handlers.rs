use crate::db::connection::DbPool;
use crate::db::models::Set;
use crate::db::schema::sets::dsl::*;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use log::error;

pub async fn get_sets(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    match sets.load::<Set>(&mut conn) {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(err) => {
            error!("Error querying sets: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
