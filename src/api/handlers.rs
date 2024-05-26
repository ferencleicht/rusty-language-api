use crate::db::connection::DbPool;
use crate::db::models::Set;
use crate::db::schema::sets::dsl::*;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;

pub async fn get_sets(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let results = sets.load::<Set>(&mut conn).expect("Error loading sets");

    HttpResponse::Ok().json(results)
}
