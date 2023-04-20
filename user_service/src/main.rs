use dotenv::dotenv;
use env_logger::Env;
use log::{error, info};
use std::env;
use tonic::transport::Server;

mod mongodb_client;
mod user_service;
use user_service::{user_service::user_service_server::UserServiceServer, MyUserService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    dotenv().ok();

    let addr = env::var("GRPC_SERVER_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:50051".to_string())
        .parse()
        .unwrap();

    let user_service = MyUserService::new().await?;

    // Test MongoDB connection
    match user_service.test_connection().await {
        Ok(_) => info!("✅ Connection to MongoDB is successful!"),
        Err(e) => {
            error!("❌ Failed to connect to MongoDB: {:?}", e);
            std::process::exit(1);
        }
    }

    info!("✅ Server started successfully");

    Server::builder()
        .add_service(UserServiceServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}
