// @generated automatically by Diesel CLI.

diesel::table! {
    sets (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
    }
}
