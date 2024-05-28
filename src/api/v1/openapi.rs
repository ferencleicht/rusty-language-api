use serde::Deserialize;
use utoipa::OpenApi;

use crate::api::v1::handlers;
use crate::db::models::Dictionary;

#[derive(OpenApi, Deserialize)]
#[openapi(
    paths(
        handlers::get_dictionary_by_id,
        handlers::get_dictionaries,
        handlers::create_dictionary,
        handlers::delete_dictionary
    ),
    components(schemas(Dictionary))
)]
pub struct ApiDoc;
