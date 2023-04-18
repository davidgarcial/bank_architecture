mod handlers;
mod jwt_auth;
mod models;
mod grpc_clients;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use dotenv::dotenv;
use env_logger::{Builder, Env};
use log::{info, error};
use tonic::transport::Channel;
use std::time::{Duration};
use tokio;

use crate::models::config::Config;

use crate::grpc_clients::user_grpc_client::get_user_grpc_client;
use crate::grpc_clients::user_grpc_client::user_service::user_service_client::UserServiceClient;

use crate::grpc_clients::account_grpc_client::get_account_grpc_client;
use crate::grpc_clients::account_grpc_client::account::account_service_client::AccountServiceClient;

use crate::grpc_clients::deposit_grpc_client::get_deposit_grpc_client;
use crate::grpc_clients::deposit_grpc_client::deposit::deposit_service_client::DepositServiceClient;

use crate::grpc_clients::withdrawal_grpc_client::get_withdrawal_grpc_client;
use crate::grpc_clients::withdrawal_grpc_client::withdrawal::withdrawal_service_client::WithdrawalServiceClient;

use crate::grpc_clients::historical_grpc_client::get_historical_grpc_client;
use crate::grpc_clients::historical_grpc_client::historical::historical_service_client::HistoricalServiceClient;

pub struct AppState {
    env: Config,
    user_grpc_client: UserServiceClient<Channel>,
    account_grpc_client: AccountServiceClient<Channel>,
    deposit_grpc_client: DepositServiceClient<Channel>,
    withdrawal_grpc_client: WithdrawalServiceClient<Channel>,
    historical_grpc_client: HistoricalServiceClient<Channel>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Config::init();

    let user_grpc_client = loop {
        match get_user_grpc_client(config.user_grpc_uri.clone()).await {
            Ok(client) => {
                info!("✅ Connection to the user gRPC service is successful!");
                break client;
            }
            Err(err) => {
                error!("❌ Failed to connect to the user gRPC service: {:?}", err);
                info!("Retrying in 3 seconds...");
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        };
    };
    
    let account_grpc_client = loop {
        match get_account_grpc_client(config.account_grpc_uri.clone()).await {
            Ok(client) => {
                info!("✅ Connection to the account gRPC service is successful!");
                break client;
            }
            Err(err) => {
                error!("❌ Failed to connect to the account gRPC service: {:?}", err);
                info!("Retrying in 3 seconds...");
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        };
    };
    
    let deposit_grpc_client = loop {
        match get_deposit_grpc_client(config.deposit_grpc_uri.clone()).await {
            Ok(client) => {
                info!("✅ Connection to the deposit gRPC service is successful!");
                break client;
            }
            Err(err) => {
                error!("❌ Failed to connect to the deposit gRPC service: {:?}", err);
                info!("Retrying in 3 seconds...");
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        };
    };
    
    let withdrawal_grpc_client = loop {
        match get_withdrawal_grpc_client(config.withdrawal_grpc_uri.clone()).await {
            Ok(client) => {
                info!("✅ Connection to the withdrawal gRPC service is successful!");
                break client;
            }
            Err(err) => {
                error!("❌ Failed to connect to the withdrawal gRPC service: {:?}", err);
                info!("Retrying in 3 seconds...");
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        };
    };    

    let historical_grpc_client = loop {
        match get_historical_grpc_client(config.historical_grpc_uri.clone()).await {
            Ok(client) => {
                info!("✅ Connection to the historical gRPC service is successful!");
                break client;
            }
            Err(err) => {
                error!("❌ Failed to connect to the historical gRPC service: {:?}", err);
                info!("Retrying in 3 seconds...");
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        };
    };

    info!("✅ Server started successfully");

    // Create an Actix HTTP server instance.
    // Actix HTTP server is a high-performance, asynchronous HTTP server library 
    // built on top of the Actix framework. It provides an ergonomic way to build 
    // web applications using the Rust programming language. 
    // The server utilizes non-blocking I/O and the Tokio asynchronous runtime, 
    // enabling it to handle a large number of concurrent connections efficiently.
    // In this code, we define routes and handlers for user registration, login, 
    // logout, and fetching user data. The server handles incoming requests 
    // by routing them to the appropriate handler functions, which process 
    // the request and generate a response.

    HttpServer::new(move || {
        // Configure CORS options.
        // - Allow requests from "http://localhost:3000"
        // - Allow GET and POST methods
        // - Allow certain headers: Content-Type, Authorization, and Accept
        // - Support credentials, like cookies, for cross-origin requests
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

            // Configure the Actix Web application.
            App::new()
                // Set the shared application state, including gRPC clients and configuration.
                .app_data(web::Data::new(AppState {
                    env: config.clone(),
                    user_grpc_client: user_grpc_client.clone(),
                    account_grpc_client: account_grpc_client.clone(),
                    deposit_grpc_client: deposit_grpc_client.clone(),
                    withdrawal_grpc_client: withdrawal_grpc_client.clone(),
                    historical_grpc_client: historical_grpc_client.clone()
                }))
                // Register handlers for various routes and resources.
                .service(handlers::healt_handler::health_checker_handler)
                .configure(handlers::user_handler::config)
                .configure(handlers::account_handlers::config)
                .configure(handlers::deposit_handlers::config)
                .configure(handlers::withdrawal_handlers::config)
                // Apply CORS middleware.
                .wrap(cors)
                // Apply logging middleware.
                .wrap(Logger::default())
    })
    // Bind the server to a specific address and port.
    .bind(("0.0.0.0", 5000))?
    // Run the server.
    .run()
    .await

}
