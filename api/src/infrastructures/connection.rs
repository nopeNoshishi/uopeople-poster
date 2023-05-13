use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

pub fn establish_connection() -> PgConnection {

    let database = "postgres";
    let user_name = env::var("POSTGRES_USER").unwrap();
    let password = env::var("POSTGRES_PASSWORD").unwrap();
    let port = env::var("POSTGRES_PORT").unwrap();
    let db_name = env::var("POSTGRES_DB").unwrap();

    let database_url = format!(
        "{}://{}:{}@db:{}/{}", 
        database, 
        user_name,
        password,
        port,
        db_name
    );
    PgConnection::establish(&database_url)
        .unwrap_or_else(|error| panic!("{}", error))
}
