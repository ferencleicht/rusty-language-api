// @generated automatically by Diesel CLI.

diesel::table! {
    study_sets (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
    }
}
