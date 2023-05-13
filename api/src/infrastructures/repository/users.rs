use super::super::database::schema::*;

use diesel::prelude::*;
use chrono::naive::NaiveDateTime;

#[derive(Queryable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}


#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

impl NewUser {
    pub fn new(name: String, email: String) -> Self {
        Self {
            name,
            email
        }
    }
}
