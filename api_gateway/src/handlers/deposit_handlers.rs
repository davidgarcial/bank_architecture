use crate::{
    AppState,
    jwt_auth,
    grpc_clients::deposit_grpc_client::deposit::MakeDepositRequest,
    models::deposit_request::DepositRequest
};

use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;

#[get("healthchecker")]
async fn health_checker_handler(
    _: jwt_auth::JwtMiddleware
) -> impl Responder {
    const MESSAGE: &str = "JWT Authentication in Rust using Actix-web and Mongodb";
    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE}))
}

#[post("")]
async fn deposit_handler(
    body: web::Json<DepositRequest>,
    data: web::Data<AppState>,
    _: jwt_auth::JwtMiddleware
) -> impl Responder {
    let mut grpc_client = data.deposit_grpc_client.clone();

    let deposit_request = MakeDepositRequest {
        from_account_id: body.from_account_id.clone(),
        to_account_id: body.to_account_id.clone(),
        amount: body.amount,
        is_bank_agent: body.is_bank_agent
    };

    let result = grpc_client
        .make_deposit(tonic::Request::new(deposit_request))
        .await;

    match result {
        Ok(response) => {
            let status = response.into_inner().success;
            
            let deposit_response = serde_json::json!({"status": status});

            HttpResponse::Ok().json(deposit_response)
        }
        Err(e) => {
            HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}))
        }
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/bank/deposit")
        .service(health_checker_handler)
        .service(deposit_handler);

    conf.service(scope);
}
