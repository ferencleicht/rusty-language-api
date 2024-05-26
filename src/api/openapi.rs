use serde::Deserialize;
use utoipa::OpenApi;

use crate::api::handlers;
use crate::db::models::Dictionary;

#[derive(OpenApi, Deserialize)]
#[openapi(
    paths(handlers::get_dictionary_by_id, handlers::get_dictionaries),
    components(schemas(Dictionary))
)]
pub struct ApiDoc;
