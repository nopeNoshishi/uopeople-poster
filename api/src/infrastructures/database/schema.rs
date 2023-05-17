// @generated automatically by Diesel CLI.

diesel::table! {
    informations (id) {
        id -> Int8,
        user_id -> Int8,
        url -> Varchar,
        tag -> Varchar,
        title -> Varchar,
        create_at -> Timestamp,
    }
}

diesel::table! {
    likes (id) {
        id -> Int8,
        user_id -> Int8,
        information_id -> Int8,
        create_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        create_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(informations, likes, users,);
