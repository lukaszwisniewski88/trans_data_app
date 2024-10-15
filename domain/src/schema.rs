// @generated automatically by Diesel CLI.

diesel::table! {
    sessions (id) {
        id -> Text,
        user_id -> Nullable<Uuid>,
        created_at -> Timestamp,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        email -> Text,
    }
}

diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(sessions, users,);
