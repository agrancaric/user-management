use diesel::{result::Error, ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    common::types::{DbPool, SortProperty},
    schema::user,
    sort_by,
};

use super::user_models::{User, UserData};

pub struct UserService {
    pool: DbPool,
}

impl Clone for UserService {
    fn clone(&self) -> UserService {
        UserService::new(self.pool.clone())
    }
}

impl UserService {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn find_all(
        &self,
        offset: i64,
        limit: i64,
        sort_properties: Option<Vec<SortProperty>>,
    ) -> Result<Vec<User>, Error> {
        let mut query = user::table.into_boxed();

        query = sort_by!(
            query,
            sort_properties,
            ("id", user::id),
            ("first_name", user::first_name),
            ("last_name", user::last_name),
            ("email", user::email)
        );

        query
            .offset(offset)
            .limit(limit)
            .get_results(&mut *self.pool.get().unwrap())
    }

    pub fn find_by_id(&self, id: i32) -> Result<User, Error> {
        user::table
            .find(id)
            .get_result(&mut *self.pool.get().unwrap())
    }

    pub fn save(&self, user: UserData) -> Result<User, Error> {
        diesel::insert_into(user::table)
            .values(&user)
            .get_result(&mut *self.pool.get().unwrap())
    }

    pub fn update(&self, id: i32, user: UserData) -> Result<User, Error> {
        diesel::update(user::table.find(id))
            .set(&user)
            .get_result(&mut *self.pool.get().unwrap())
    }

    pub fn delete(&self, id: i32) -> Result<usize, Error> {
        diesel::delete(user::table.find(id)).execute(&mut *self.pool.get().unwrap())
    }
}
