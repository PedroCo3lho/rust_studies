use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use grpc_crud::create_pool;
use tonic::{Request, Response, Status, transport::Server};
use user::user_service_server::{UserService, UserServiceServer};
use user::{
    CreateUserRequest, CreateUserResponse, DeleteUserRequest, DeleteUserResponse, GetUserRequest,
    GetUserResponse, UpdateUserRequest, UpdateUserResponse,
};

use crate::queries::create_user::create_user;

pub mod user {
    tonic::include_proto!("user");
}

mod queries;

#[derive(Debug)]
pub struct CrudSystem {
    pool: Pool<AsyncPgConnection>,
}

impl CrudSystem {
    pub fn new(pool: Pool<AsyncPgConnection>) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl UserService for CrudSystem {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let req = request.into_inner();

        match create_user(&self.pool, &req.username, &req.email).await {
            Ok(user) => {
                let proto_user = user::User {
                    id: user.id.to_string(),
                    username: user.username,
                    email: user.email,
                };

                let response = CreateUserResponse {
                    user: Some(proto_user),
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                eprintln!("Database error: {:?}", e);
                Err(Status::internal("Failed to create user"))
            }
        }
    }
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        todo!()
    }
    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UpdateUserResponse>, Status> {
        todo!()
    }
    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let crud_system = CrudSystem::new(create_pool().await);

    Server::builder()
        .add_service(UserServiceServer::new(crud_system))
        .serve(addr)
        .await?;

    Ok(())
}
