use crate::db::connection::DbPool;
use crate::db::models::{Dictionary, NewDictionary};
use crate::db::schema::dictionaries::dsl::*;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::result::Error;
use log::error;

#[utoipa::path(
    get,
    path = "/api/v1/dictionaries",
    tag = "Dictionaries",
    responses(
        (status = 200, description = "Dictionaries found successfully", body = Vec<Dictionary>),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
pub async fn get_dictionaries(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    match dictionaries.load::<Dictionary>(&mut conn) {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(err) => {
            error!("Error querying dictionaries: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/dictionaries/{id}",
    tag = "Dictionaries",
    responses(
        (status = 200, description = "Dictionary found successfully", body = Dictionary),
        (status = NOT_FOUND, description = "Dictionary not found", body = String),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    params(
        ("id" = i32, Path, description = "The id of the dictionary")
    )
)]
pub async fn get_dictionary_by_id(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let _id = path.into_inner();

    match dictionaries.find(_id).first::<Dictionary>(&mut conn) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(Error::NotFound) => HttpResponse::NotFound()
            .json("Dictionary not found with id: ".to_string() + &_id.to_string()),
        Err(err) => {
            error!("Error querying dictionary by id: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/dictionaries",
    tag = "Dictionaries",
    request_body(content = NewDictionary, description = "The new dictionary to create", content_type = "application/json"),
    responses(
        (status = 201, description = "Dictionary created successfully", body = Dictionary),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
pub async fn create_dictionary(
    pool: web::Data<DbPool>,
    new_dictionary: web::Json<NewDictionary>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    match diesel::insert_into(dictionaries)
        .values(new_dictionary.into_inner())
        .get_result::<Dictionary>(&mut conn)
    {
        Ok(result) => HttpResponse::Created().json(result),
        Err(err) => {
            error!("Error creating dictionary: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/dictionaries/{id}",
    tag = "Dictionaries",
    responses(
        (status = 204, description = "Dictionary deleted successfully"),
        (status = NOT_FOUND, description = "Dictionary not found", body = String),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    params(
        ("id" = i32, Path, description = "The id of the dictionary to delete")
    )
)]
pub async fn delete_dictionary(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let _id = path.into_inner();

    match diesel::delete(dictionaries.find(_id)).execute(&mut conn) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(Error::NotFound) => HttpResponse::NotFound()
            .json("Dictionary not found with id: ".to_string() + &_id.to_string()),
        Err(err) => {
            error!("Error deleting dictionary: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
