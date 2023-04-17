use std::env;
use dotenv::dotenv;
use tonic::transport::Server;

mod deposit_service;
use deposit_service::{MyDepositService, deposit::deposit_service_server::DepositServiceServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load environment variables from .env file

    let mongodb_uri = env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let addr = env::var("GRPC_SERVER_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:50053".to_string())
        .parse()
        .unwrap();
    
    let user_service = MyDepositService::new(&mongodb_uri).await?;

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
        .add_service(DepositServiceServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}
