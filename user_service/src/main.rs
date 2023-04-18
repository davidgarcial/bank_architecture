use std::env;
use dotenv::dotenv;
use tonic::transport::Server;

mod mongodb_client;
mod user_service;
use user_service::{MyUserService, user_service::user_service_server::UserServiceServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let addr = env::var("GRPC_SERVER_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:50051".to_string())
        .parse()
        .unwrap();
    
    let user_service = MyUserService::new().await?;

    // Test MongoDB connection
    match user_service.test_connection().await {
        Ok(_) => println!("✅ Connection to MongoDB is successful!"),
        Err(e) => {
            println!("❌ Failed to connect to MongoDB: {:?}", e);
            std::process::exit(1);
        }
    }

    println!("✅ Server started successfully");

    Server::builder()
        .add_service(UserServiceServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}
