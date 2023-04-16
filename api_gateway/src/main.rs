mod config;
mod handlers;
mod jwt_auth;
mod models;
mod response;
mod user_management_client;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use config::Config;
use dotenv::dotenv;
use env_logger::{Builder, Env};
use log::{info, error};

use crate::user_management_client::get_grpc_client;
use crate::user_management_client::user_management::user_service_client::UserServiceClient;
use tonic::transport::Channel;

pub struct AppState {
    env: Config,
    user_management_client: UserServiceClient<Channel>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Config::init();

    let grpc_client = match get_grpc_client(config.grpc_user_management_service_url.clone()).await {
        Ok(client) => {
            info!("✅ Connection to the gRPC service is successful!");
            client
        }
        Err(err) => {
            error!("❌ Failed to connect to the gRPC service: {:?}", err);
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
                user_management_client: grpc_client.clone()
            }))
            .service(handlers::healt_handler::health_checker_handler)
            .configure(handlers::user_handler::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
