use tonic::{Request, Response, Status};
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

use mongodb::{
    Collection,
    bson::{doc, Document, oid::ObjectId},
    {options::ClientOptions, Client}
};

pub mod user_management {
    tonic::include_proto!("user_management");
}

use user_management::user_service_server::{UserService};
use user_management::{
    CreateUserRequest, CreateUserResponse, 
    DeleteUserRequest, DeleteUserResponse, 
    GetUserByIdRequest, GetUserByUserNameRequest, GetUserResponse, 
    UpdateUserRequest, UpdateUserResponse
};

#[derive(Debug, Clone)]
pub struct MyUserService {
    db: Arc<mongodb::Database>,
}

impl MyUserService {
    pub async fn new(uri: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database("bank");
        Ok(Self { db: Arc::new(db) })
    }

    pub async fn test_connection(&self) -> Result<(), mongodb::error::Error> {
        let _ = self.db.run_command(doc! { "ping": 1 }, None).await?;
        Ok(())
    }
}

#[tonic::async_trait]
impl UserService for MyUserService {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let req = request.into_inner();
        let users_collection: Collection<Document> = self.db.collection("users");
    
        let new_user = doc! {
            "uuid": Uuid::new_v4().to_string(),
            "username": req.username,
            "password": req.password
        };
    
        let insert_result = users_collection
            .insert_one(new_user, None)
            .await
            .map_err(|e| Status::internal(format!("Failed to create user: {}", e)))?;
    
        let response = CreateUserResponse {
            id: insert_result.inserted_id.as_object_id().unwrap().to_string(),
        };
    
        Ok(Response::new(response))
    }

    async fn get_user_by_email(
        &self,
        request: Request<GetUserByUserNameRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let req = request.into_inner();
        let users_collection: Collection<Document> = self.db.collection("users");

        let filter = doc! {
            "username": &req.username,
        };
    
        let user_doc_option = users_collection
            .find_one(filter, None)
            .await
            .map_err(|e| Status::internal(format!("Failed to get user: {}", e)))?;
    
        if let Some(user_doc) = user_doc_option {
            let response = GetUserResponse {
                id: user_doc.get_object_id("_id").unwrap().to_string(),
                uuid: user_doc.get_str("uuid").unwrap().to_string(),
                username: user_doc.get_str("username").unwrap().to_string(),
                password: user_doc.get_str("password").unwrap().to_string(),
            };
    
            Ok(Response::new(response))
        } else {
            Err(Status::not_found("User not found"))
        }
    }

    async fn get_user_by_id(
        &self,
        request: Request<GetUserByIdRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let req = request.into_inner();
        let users_collection: Collection<Document> = self.db.collection("users");
    
        let filter = doc! {
            "uuid": &req.id
        };
    
        let user_doc_option = users_collection
            .find_one(filter, None)
            .await
            .map_err(|e| Status::internal(format!("Failed to get user: {}", e)))?;
    
        if let Some(user_doc) = user_doc_option {
            let response = GetUserResponse {
                id: user_doc.get_object_id("_id").unwrap().to_string(),
                username: user_doc.get_str("username").unwrap().to_string(),
                password: user_doc.get_str("password").unwrap().to_string(),
                uuid: user_doc.get_str("uuid").unwrap().to_string()
            };
    
            Ok(Response::new(response))
        } else {
            Err(Status::not_found("User not found"))
        }
    }
    
    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UpdateUserResponse>, Status> {
        let req = request.into_inner();
        let users_collection: Collection<Document> = self.db.collection("users");
    
        let object_id = match ObjectId::from_str(&req.id) {
            Ok(oid) => oid,
            Err(_) => return Err(Status::invalid_argument("Invalid user id")),
        };
    
        let filter = doc! {
            "_id": object_id,
        };
    
        let update = doc! {
            "$set": {
                "username": req.username,
                "password": req.password,
            }
        };
    
        let update_result = users_collection
            .update_one(filter, update, None)
            .await
            .map_err(|e| Status::internal(format!("Failed to update user: {}", e)))?;
    
        let success = update_result.modified_count > 0;
        let response = UpdateUserResponse { success };
    
        Ok(Response::new(response))
    }
    
    // DeleteUser
    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, Status> {
        let req = request.into_inner();
        let users_collection: Collection<Document> = self.db.collection("users");
    
        let object_id = match ObjectId::from_str(&req.id) {
            Ok(oid) => oid,
            Err(_) => return Err(Status::invalid_argument("Invalid user id")),
        };
    
        let filter = doc! {
            "_id": object_id,
        };
    
        let delete_result = users_collection
            .delete_one(filter, None)
            .await
            .map_err(|e| Status::internal(format!("Failed to delete user: {}", e)))?;
    
        let success = delete_result.deleted_count > 0;
        let response = DeleteUserResponse { success };
    
        Ok(Response::new(response))
    }
}
