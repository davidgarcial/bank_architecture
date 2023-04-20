use dotenv::dotenv;
use env_logger::Env;
use log::{error, info};
use std::env;
use tonic::transport::Server;

mod account_service;
use account_service::{account::account_service_server::AccountServiceServer, MyAccountService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    dotenv().ok(); // Load environment variables from .env file

    let mongodb_uri =
        env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let addr = env::var("GRPC_SERVER_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:50051".to_string())
        .parse()
        .unwrap();

    let account_service = MyAccountService::new(&mongodb_uri).await?;

    // Test MongoDB connection
    match account_service.test_connection().await {
        Ok(_) => info!("✅ Connection to MongoDB is successful!"),
        Err(e) => {
            error!("❌ Failed to connect to MongoDB: {:?}", e);
            std::process::exit(1);
        }
    }

    info!("✅ Server started successfully");

    Server::builder()
        .add_service(AccountServiceServer::new(account_service))
        .serve(addr)
        .await?;

    Ok(())
}
