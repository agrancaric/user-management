use crate::schema::user;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::Serialize;

#[derive(Identifiable, Serialize, Queryable)]
#[diesel(table_name = user)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = user)]
pub struct UserData<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
}
