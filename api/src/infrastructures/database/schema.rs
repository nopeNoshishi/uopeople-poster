// @generated automatically by Diesel CLI.

diesel::table! {
    informations (id) {
        id -> Int8,
        url -> Text,
        tag -> Nullable<Varchar>,
        title -> Nullable<Varchar>,
        create_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    informations,
    users,
);
