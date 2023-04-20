use crate::{
    grpc_clients::historical_grpc_client::historical::GetTransactionHistoryRequest, jwt_auth,
    AppState
};

use actix_web::{get, web, HttpResponse, Responder};
use log::{error, info};
use serde_json::json;

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "JWT Authentication in Rust using Actix-web and Mongodb";
    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE}))
}

#[get("/transactions")]
async fn get_transaction_history_handler(
    account: web::Path<String>,
    data: web::Data<AppState>,
    _: jwt_auth::JwtMiddleware,
) -> impl Responder {
    let account_id = account.into_inner();
    info!(
        "Received transaction history request for account: {}",
        account_id
    );

    // Clone the gRPC client
    let mut grpc_client = data.historical_grpc_client.clone();

    // Prepare the GetTransactionHistoryRequest
    let get_transaction_history_request = GetTransactionHistoryRequest { account_id };

    // Call the gRPC GetTransactionHistory method
    let transaction_history_result = grpc_client
        .get_transaction_history(tonic::Request::new(get_transaction_history_request))
        .await;

    match transaction_history_result {
        Ok(response) => {
            let transactions = response.into_inner().transactions;
            info!(
                "Transaction history retrieved, count: {}",
                transactions.len()
            );

            let transactions_json: Vec<serde_json::Value> = transactions
                .into_iter()
                .map(|transaction| {
                    serde_json::json!({
                        "transaction_id": transaction.transaction_id,
                        "account_id": transaction.account_id,
                        "transaction_type": transaction.transaction_type,
                        "amount": transaction.amount,
                        "timestamp": transaction.timestamp
                    })
                })
                .collect();

            HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "data": serde_json::json!({ "transactions": transactions_json })
            }))
        }
        Err(err) => {
            error!("Error retrieving transaction history: {:?}", err);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": format!("{:?}", err)
            }))
        }
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/history")
        .service(health_checker_handler)
        .service(get_transaction_history_handler);

    conf.service(scope);
}
