use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::Serialize;

use crate::schema::user;

#[derive(Debug, Identifiable, Serialize, Queryable)]
#[diesel(table_name = user)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = user)]
pub struct UserData {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl UserData {
    pub fn new<T: Into<String>>(first_name: T, last_name: T, email: T) -> Self {
        Self {
            first_name: first_name.into(),
            last_name: last_name.into(),
            email: email.into(),
        }
    }
}
