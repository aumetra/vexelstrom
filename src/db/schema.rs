// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
