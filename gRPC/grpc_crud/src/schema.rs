// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}
