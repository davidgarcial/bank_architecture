// Import necessary libraries and modules
use crate::{
    grpc_clients::user_grpc_client::user_service::{
        CreateUserRequest, GetUserByIdRequest, GetUserByUserNameRequest,
    },
    jwt_auth,
    models::{
        login_user::LoginUserSchema, registrer_user::RegisterUserSchema, token_claims::TokenClaims,
    },
    AppState,
};

use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use chrono::{prelude::Utc, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use log::{error, info};
use serde_json::json;
use uuid::Uuid;

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "JWT Authentication in Rust using Actix-web and Mongodb";
    info!("Health check requested");
    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE}))
}

#[post("/register")]
async fn register_user_handler(
    body: web::Json<RegisterUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("Register user request received");

    let mut grpc_client = data.user_grpc_client.clone();

    let create_user_request = CreateUserRequest {
        username: body.email.clone(),
        password: body.password.clone(),
    };

    let result = grpc_client
        .create_user(tonic::Request::new(create_user_request))
        .await;

    match result {
        Ok(response) => {
            info!("User registration successful");
            let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "user": {
                    "id": response.into_inner().id,
                    "username": body.email.clone(),
                    "password": body.password.clone(),
                }
            })});

            HttpResponse::Ok().json(user_response)
        }
        Err(e) => {
            error!("Error during user registration: {:?}", e);
            HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}))
        }
    }
}

#[post("/login")]
async fn login_user_handler(
    body: web::Json<LoginUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("Login request received");

    let mut grpc_client = data.user_grpc_client.clone();

    let get_user_request = GetUserByUserNameRequest {
        username: body.email.clone(),
    };

    let query_result = grpc_client
        .get_user_by_email(tonic::Request::new(get_user_request))
        .await;

    let user = match query_result {
        Ok(response) => response.into_inner(),
        Err(err) => {
            error!("Error during login: {}", err);
            return HttpResponse::BadRequest()
                .json(json!({"status": "fail", "message": "Invalid email or password"}));
        }
    };

    if !(body.password == user.password) {
        error!("Invalid email or password");
        return HttpResponse::BadRequest()
            .json(json!({"status": "fail", "message": "Invalid email or password"}));
    }

    info!("User authenticated");

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;

    let user_uuid = match Uuid::parse_str(&user.uuid) {
        Ok(uuid) => uuid,
        Err(err) => {
            error!("Error parsing user UUID: {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let claims: TokenClaims = TokenClaims {
        sub: user_uuid.to_string(),
        exp,
        iat,
    };

    info!("Claims generated");

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(ActixWebDuration::new(60 * 60, 0))
        .http_only(true)
        .finish();

    info!("Token and cookie created");
    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success", "token": token}))
}

#[get("/logout")]
async fn logout_handler(_: jwt_auth::JwtMiddleware) -> impl Responder {
    info!("Logout request received");

    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    info!("Cookie cleared");
    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success"}))
}

#[get("/users/me")]
async fn get_me_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    _: jwt_auth::JwtMiddleware,
) -> impl Responder {
    info!("User data request received");
    let ext = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();

    let mut grpc_client = data.user_grpc_client.clone();

    let get_user_request = GetUserByIdRequest {
        id: user_id.to_string(),
    };

    info!("User ID: {}", user_id.to_string());

    let user_result = grpc_client
        .get_user_by_id(tonic::Request::new(get_user_request))
        .await;

    let user = match user_result {
        Ok(response) => response.into_inner(),
        Err(err) => {
            error!("Error fetching user data: {}", err);
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Error fetching user data"}));
        }
    };

    info!("User data retrieved");

    let json_response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": {
                "id": user.id,
                "username": user.username
            }
        })
    });

    HttpResponse::Ok().json(json_response)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/auth")
        .service(health_checker_handler)
        .service(register_user_handler)
        .service(login_user_handler)
        .service(logout_handler)
        .service(get_me_handler);
    conf.service(scope);
}
