mod config;
mod handlers;
mod jwt_auth;
mod models;
mod response;
mod grpc_clients;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use config::Config;
use dotenv::dotenv;
use env_logger::{Builder, Env};
use log::{info, error};

use crate::grpc_clients::user_grpc_client::get_user_grpc_client;
use crate::grpc_clients::user_grpc_client::user_management::user_service_client::UserServiceClient;

use crate::grpc_clients::account_grpc_client::get_account_grpc_client;
use crate::grpc_clients::account_grpc_client::account::account_service_client::AccountServiceClient;

use tonic::transport::Channel;

pub struct AppState {
    env: Config,
    user_grpc_client: UserServiceClient<Channel>,
    account_grpc_client: AccountServiceClient<Channel>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Config::init();

    let user_grpc_client = match get_user_grpc_client(config.user_grpc_uri.clone()).await {
        Ok(client) => {
            info!("✅ Connection to the user gRPC service is successful!");
            client
        }
        Err(err) => {
            error!("❌ Failed to connect to the user gRPC service: {:?}", err);
            std::process::exit(1);
        }
    };

    let account_grpc_client = match get_account_grpc_client(config.account_grpc_uri.clone()).await {
        Ok(client) => {
            info!("✅ Connection to the  account gRPC service is successful!");
            client
        }
        Err(err) => {
            error!("❌ Failed to connect to the account gRPC service: {:?}", err);
            std::process::exit(1);
        }
    };

    info!("✅ Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState {
                env: config.clone(),
                user_grpc_client: user_grpc_client.clone(),
                account_grpc_client: account_grpc_client.clone()
            }))
            .service(handlers::healt_handler::health_checker_handler)
            .configure(handlers::user_handler::config)
            .configure(handlers::account_handlers::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
