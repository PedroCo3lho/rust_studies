use tonic::{Request, Response, Status, transport::Server};
use user::{CreateUserRequest, CreateUserResponse, DeleteUserRequest, DeleteUserResponse, GetUserRequest, GetUserResponse,UpdateUserRequest, UpdateUserResponse};
use user::user_service_server::{UserService, UserServiceServer};

pub mod user {
    tonic::include_proto!("user");
}

#[derive(Debug, Default)]
pub struct CrudSystem {}

#[tonic::async_trait]
impl UserService for CrudSystem {
    async fn create_user(&self, request: Request<CreateUserRequest>, ) -> Result<Response<CreateUserResponse>, Status>{
        todo!()
    }
    async fn get_user(&self, request: Request<GetUserRequest>, ) -> Result<Response<GetUserResponse>, Status>{
        todo!()
    }
    async fn update_user(&self, request: Request<UpdateUserRequest>, ) -> Result<Response<UpdateUserResponse>, Status>{
        todo!()
    }
    async fn delete_user(&self, request: Request<DeleteUserRequest>, ) -> Result<Response<DeleteUserResponse>, Status>{
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let crud_system = CrudSystem::default();

    Server::builder()
        .add_service(UserServiceServer::new(crud_system))
        .serve(addr)
        .await?;

    Ok(())
}
