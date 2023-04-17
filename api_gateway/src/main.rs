mod config;
mod handlers;
mod jwt_auth;
mod models;
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

use crate::grpc_clients::deposit_grpc_client::get_deposit_grpc_client;
use crate::grpc_clients::deposit_grpc_client::deposit::deposit_service_client::DepositServiceClient;

use crate::grpc_clients::withdrawal_grpc_client::get_withdrawal_grpc_client;
use crate::grpc_clients::withdrawal_grpc_client::withdrawal::withdrawal_service_client::WithdrawalServiceClient;

use tonic::transport::Channel;

pub struct AppState {
    env: Config,
    user_grpc_client: UserServiceClient<Channel>,
    account_grpc_client: AccountServiceClient<Channel>,
    deposit_grpc_client: DepositServiceClient<Channel>,
    withdrawal_grpc_client: WithdrawalServiceClient<Channel>
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
            info!("✅ Connection to the account gRPC service is successful!");
            client
        }
        Err(err) => {
            error!("❌ Failed to connect to the account gRPC service: {:?}", err);
            std::process::exit(1);
        }
    };

    let deposit_grpc_client = match get_deposit_grpc_client(config.deposit_grpc_uri.clone()).await {
        Ok(client) => {
            info!("✅ Connection to the deposit gRPC service is successful!");
            client
        }
        Err(err) => {
            error!("❌ Failed to connect to the deposit gRPC service: {:?}", err);
            std::process::exit(1);
        }
    };

    let withdrawal_grpc_client = match get_withdrawal_grpc_client(config.withdrawal_grpc_uri.clone()).await {
        Ok(client) => {
            info!("✅ Connection to the withdrawal gRPC service is successful!");
            client
        }
        Err(err) => {
            error!("❌ Failed to connect to the withdrawal gRPC service: {:?}", err);
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
                account_grpc_client: account_grpc_client.clone(),
                deposit_grpc_client: deposit_grpc_client.clone(),
                withdrawal_grpc_client: withdrawal_grpc_client.clone()
            }))
            .service(handlers::healt_handler::health_checker_handler)
            .configure(handlers::user_handler::config)
            .configure(handlers::account_handlers::config)
            .configure(handlers::deposit_handlers::config)
            .configure(handlers::withdrawal_handlers::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
