// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
    }
}
