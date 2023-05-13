diesel::table! {
    users (id) {
        id -> Integer,
        name -> VarChar,
        email -> VarChar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    informations (id) {
        id -> Integer,
        url -> VarChar,
        tag -> VarChar,
        created_at -> Timestamp,
    }
}
