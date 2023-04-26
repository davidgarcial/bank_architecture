use crate::{
    grpc_clients::withdrawal_grpc_client::withdrawal::MakeWithdrawalRequest, jwt_auth,
    models::withdrawal_request::WithdrawalRequest, 
    AppState
};

use actix_web::{get, post, web, HttpResponse, Responder};
use log::{error, info};
use serde_json::json;

#[get("healthchecker")]
async fn health_checker_handler(_: jwt_auth::JwtMiddleware) -> impl Responder {
    const MESSAGE: &str = "JWT Authentication in Rust using Actix-web and Mongodb";
    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE}))
}

#[post("")]
async fn withdraw_handler(
    body: web::Json<WithdrawalRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!(
        "Received withdrawal request for account: {} and amount: {}",
        body.account_id, body.amount
    );

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
            let transaction_id = response.into_inner().transaction_id;
            info!("Withdrawal successful, transaction_id: {}", transaction_id);
            let withdrawal_response =
                serde_json::json!({"status": "success", "transaction_id": transaction_id});
            HttpResponse::Ok().json(withdrawal_response)
        }
        Err(e) => {
            error!("Error processing withdrawal: {:?}", e);
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
