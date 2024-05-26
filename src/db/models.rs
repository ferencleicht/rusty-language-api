use crate::db::schema::{dictionaries, translations};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Queryable, Selectable, ToSchema)]
#[diesel(table_name = dictionaries, check_for_backend(diesel::pg::Pg))]
pub struct Dictionary {
    pub id: i32,
    pub language: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, ToSchema)]
#[diesel(table_name = dictionaries, check_for_backend(diesel::pg::Pg))]
pub struct NewDictionary {
    pub language: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Queryable, Selectable)]
#[diesel(table_name = translations, check_for_backend(diesel::pg::Pg))]
pub struct Translation {
    pub id: i32,
    pub dictionary_id: i32,
    pub source: String,
    pub target: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = translations, check_for_backend(diesel::pg::Pg))]
pub struct NewTranslation {
    pub dictionary_id: i32,
    pub source: String,
    pub target: String,
}
