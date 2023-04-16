use actix_web::{
    get, HttpResponse, Responder
};

use serde_json::json;

#[get("healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "JWT Authentication in Rust using Actix-web and Mongodb";
    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE}))
}
