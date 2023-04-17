use core::fmt;
use std::future::{ready, Ready};

use actix_web::error::ErrorUnauthorized;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, web, FromRequest, HttpMessage, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;

use crate::models::token_claims::TokenClaims;
use crate::AppState;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    status: String,
    message: String,
}

// Implement `fmt::Display` for `ErrorResponse` to enable conversion to string.
impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

// Define the `JwtMiddleware` struct that will store the authenticated user's ID.
pub struct JwtMiddleware {
    pub user_id: uuid::Uuid,
}

// Implement the `FromRequest` trait for `JwtMiddleware`.
impl FromRequest for JwtMiddleware {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    // This function is called when the middleware is applied to a request.
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Get the application state.
        let data = req.app_data::<web::Data<AppState>>().unwrap();

        // Try to extract the JWT from a cookie or the Authorization header.
        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        // If there's no token, return an error.
        if token.is_none() {
            let json_error = ErrorResponse {
                status: "fail".to_string(),
                message: "You are not logged in, please provide token".to_string(),
            };
            return ready(Err(ErrorUnauthorized(json_error)));
        }

        // Decode and validate the JWT.
        let claims = match decode::<TokenClaims>(
            &token.unwrap(),
            &DecodingKey::from_secret(data.env.jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(c) => c.claims,
            Err(_) => {
                let json_error = ErrorResponse {
                    status: "fail".to_string(),
                    message: "Invalid token".to_string(),
                };
                return ready(Err(ErrorUnauthorized(json_error)));
            }
        };

        // Extract the user's ID from the JWT and store it in the request's extensions.
        let user_id = uuid::Uuid::parse_str(claims.sub.as_str()).unwrap();
        req.extensions_mut()
            .insert::<uuid::Uuid>(user_id.to_owned());

        // Return an instance of `JwtMiddleware` containing the user's ID.
        ready(Ok(JwtMiddleware { user_id }))
    }
}