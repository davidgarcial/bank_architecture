use crate::{
    AppState,
    jwt_auth,
    models::{
        account::{Account, AccountType as AccountTypeModel},
        account_update_request::UpdateAccountRequestModel
    },
    grpc_clients::account_grpc_client::account::{
        AccountType, CreateAccountRequest, GetAccountRequest, UpdateAccountRequest
    }
};

use actix_web::{get, post, put, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde_json::json;

#[get("healthchecker")]
async fn health_checker_handler(
    _: jwt_auth::JwtMiddleware
) -> impl Responder {
    const MESSAGE: &str = "JWT Authentication in Rust using Actix-web and Mongodb";
    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE}))
}

#[post("/create")]
async fn create_account_handler(
    req: HttpRequest,
    body: web::Json<Account>,
    data: web::Data<AppState>,
    _: jwt_auth::JwtMiddleware,
) -> impl Responder {
    let ext = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();

    let mut grpc_client = data.account_grpc_client.clone();

    let account_type = match body.account_type {
        AccountTypeModel::Checking => AccountType::Checking,
        AccountTypeModel::Savings => AccountType::Savings,
    };

    let result = grpc_client
        .create_account(tonic::Request::new(CreateAccountRequest { 
            user_id: user_id.to_string(), 
            account_type: account_type as i32
        }))
        .await;

    match result {
        Ok(response) => {
            let account_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "account": {
                    "id": response.into_inner().account_id
                }
            })});
            HttpResponse::Ok().json(account_response)
        },
        Err(e) => {
            HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}))
        }
    }
}

#[get("/{account_id}")]
async fn get_account_handler(
    account: web::Path<String>,
    data: web::Data<AppState>,
    _: jwt_auth::JwtMiddleware,
) -> impl Responder {
    let mut grpc_client = data.account_grpc_client.clone();

    let account_id  = account.into_inner();
    println!("{}", account_id .clone());
    let result = grpc_client
        .get_account(tonic::Request::new(GetAccountRequest { account_id: account_id .clone()}))
        .await;

    match result {
        Ok(response) => {
            let account = response.into_inner().account.unwrap();
            let account_response = serde_json::json!({"status": "success","account": serde_json::json!({
                "id": account.account_id,
                "user_id": account.user_id,
                "account_type": account.account_type,
                "balance": account.balance 
            })});
            HttpResponse::Ok().json(account_response)
        },
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({ "status": "error", "message": format!("{:?}", e) }))
        }
    }
}

#[put("/update")]
async fn update_account_handler(
    account: web::Json<UpdateAccountRequestModel>,
    data: web::Data<AppState>,
    _: jwt_auth::JwtMiddleware,
) -> impl Responder {
    let mut grpc_client = data.account_grpc_client.clone();

    let result = grpc_client
        .update_account(UpdateAccountRequest { 
            account_id: account.account_id.clone(),
            balance: account.balance.clone()
        })
        .await;

    match result {
        Ok(response) => {
            let account = response.into_inner().account.unwrap();
            let account_response = serde_json::json!({"status": "success","account": serde_json::json!({
                "id": account.account_id,
                "user_id": account.user_id,
                "account_type": account.account_type,
                "balance": account.balance
            })});
            HttpResponse::Ok().json(account_response)
        },
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({ "status": "error", "message": format!("{:?}", e) }))
        }
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/account")
        .service(health_checker_handler)
        .service(create_account_handler)
        .service(update_account_handler)
        .service(get_account_handler);

    conf.service(scope);
}
