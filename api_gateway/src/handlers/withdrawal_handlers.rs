use crate::{
    AppState,
    jwt_auth,
    grpc_clients::withdrawal_grpc_client::withdrawal::MakeWithdrawalRequest,
    models::withdrawal_request::WithdrawalRequest
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

#[post("/withdraw")]
async fn withdraw_handler(
    body: web::Json<WithdrawalRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut grpc_client = data.withdrawal_grpc_client.clone();

    let withdrawal_request = MakeWithdrawalRequest {
        account_id: body.account_id.clone(),
        amount: body.amount,
    };

    let result = grpc_client
        .make_withdrawal(tonic::Request::new(withdrawal_request))
        .await;

    match result {
        Ok(response) => {
            let withdrawal_response = serde_json::json!({"status": "success", "transaction_id": response.into_inner().transaction_id});
            HttpResponse::Ok().json(withdrawal_response)
        }
        Err(e) => {
            HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}))
        }
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/bank/withdraw")
        .service(health_checker_handler)
        .service(withdraw_handler);

    conf.service(scope);
}
