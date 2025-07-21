pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use self::models::{User, NewUser};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|e| panic!("Failed to connect, error: {e}"))
}

pub fn create_user(conn: &mut PgConnection, username: &str, email: &str) -> User {
    use crate::schema::users;

    let new_user = NewUser { username, email };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error while creating the user")
}