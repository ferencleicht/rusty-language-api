use crate::db::connection::DbPool;
use crate::db::models::{NewStudySet, StudySet};
use crate::db::schema::study_sets::dsl::*;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::result::Error;
use log::error;

pub async fn get_sets(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    match study_sets.load::<StudySet>(&mut conn) {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(err) => {
            error!("Error querying sets: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_set_by_id(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let _id = path.into_inner();

    match study_sets.find(_id).first::<StudySet>(&mut conn) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(Error::NotFound) => {
            HttpResponse::NotFound().json("Set not found with id: ".to_string() + &_id.to_string())
        }
        Err(err) => {
            error!("Error querying set by id: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn create_set(
    pool: web::Data<DbPool>,
    new_set: web::Json<NewStudySet>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    match diesel::insert_into(study_sets)
        .values(new_set.into_inner())
        .get_result::<StudySet>(&mut conn)
    {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => {
            error!("Error creating set: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
