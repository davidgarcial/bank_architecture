#[derive(Debug, Clone)]
pub struct Config {
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
    pub grpc_user_management_service_url: String,
}

impl Config {
    pub fn init() -> Config {
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires_in = std::env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
        let jwt_maxage = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");
        let grpc_user_management_service_url = std::env::var("GRPC_USER_MANAGEMENT_SERVICE_URL").expect("GRPC_USER_MANAGEMENT_SERVICE_URL must be set");
        Config {
            jwt_secret,
            jwt_expires_in,
            jwt_maxage: jwt_maxage.parse::<i32>().unwrap(),
            grpc_user_management_service_url
        }
    }
}
