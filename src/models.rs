use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::users;

#[derive(Debug, Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}