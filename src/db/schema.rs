// @generated automatically by Diesel CLI.

diesel::table! {
    dictionaries (id) {
        id -> Int4,
        language -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    translations (id) {
        id -> Int4,
        dictionary_id -> Int4,
        source -> Text,
        target -> Text,
        created_at -> Timestamp,
    }
}

diesel::joinable!(translations -> dictionaries (dictionary_id));

diesel::allow_tables_to_appear_in_same_query!(dictionaries, translations,);
