use diesel::{result::Error, ExpressionMethods, QueryDsl};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

use crate::{
    common::models::{Page, SortProperty},
    schema::user,
    sort_by,
};

use super::user_model::{User, UserData};

pub struct UserService {
    pool: Pool<AsyncPgConnection>,
}

impl UserService {
    pub fn new(pool: Pool<AsyncPgConnection>) -> Self {
        Self { pool }
    }

    pub async fn find_all(
        &self,
        offset: i64,
        limit: i64,
        sort_properties: Option<&Vec<SortProperty>>,
    ) -> Result<Page<User>, Error> {
        let mut query = user::table.into_boxed();

        query = sort_by!(
            query,
            sort_properties,
            ("id", user::id),
            ("first_name", user::first_name),
            ("last_name", user::last_name),
            ("email", user::email)
        );

        let users = query
            .offset(offset)
            .limit(limit)
            .get_results(&mut *self.pool.get().await.unwrap())
            .await?;

        let total_count;

        if users.is_empty() {
            total_count = 0;
        } else {
            total_count = user::table
                .count()
                .get_result(&mut *self.pool.get().await.unwrap())
                .await?
        }

        Result::Ok(Page::new(total_count, users))
    }

    pub async fn find_by_id(&self, id: i32) -> Result<User, Error> {
        user::table
            .find(id)
            .get_result(&mut *self.pool.get().await.unwrap())
            .await
    }

    pub async fn save(&self, user: &UserData) -> Result<User, Error> {
        diesel::insert_into(user::table)
            .values(user)
            .get_result(&mut *self.pool.get().await.unwrap())
            .await
    }

    pub async fn update(&self, id: i32, user: &UserData) -> Result<User, Error> {
        diesel::update(user::table.find(id))
            .set(user)
            .get_result(&mut *self.pool.get().await.unwrap())
            .await
    }

    pub async fn delete(&self, id: i32) -> Result<usize, Error> {
        diesel::delete(user::table.find(id))
            .execute(&mut *self.pool.get().await.unwrap())
            .await
    }
}
