use diesel_async::{AsyncPgConnection, RunQueryDsl, pooled_connection::bb8::Pool};
use grpc_crud::{
    models::{NewUser, User},
    schema::users,
};

pub async fn create_user(
    pool: &Pool<AsyncPgConnection>,
    username: &str,
    email: &str,
) -> Result<User, diesel::result::Error> {
    let mut conn = pool.get().await.expect("Failed to get connection");

    let new_user = NewUser { username, email };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(&mut conn)
        .await
}
