pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use self::models::{NewUser, User};

use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn create_pool() -> Pool<AsyncPgConnection> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    Pool::builder()
        .max_size(10)
        .min_idle(Some(5))
        .build(manager)
        .await
        .expect("Database connection pooling failed")
}

pub async fn create_user(
    pool: &Pool<AsyncPgConnection>,
    username: &str,
    email: &str,
) -> Result<User, diesel::result::Error> {
    use crate::schema::users;
    let mut conn = pool.get().await.expect("Failed to get connection");

    let new_user = NewUser { username, email };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await
}
