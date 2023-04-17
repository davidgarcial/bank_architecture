#[derive(Debug, Clone)]
pub struct Config {
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
    pub user_grpc_uri: String,
    pub account_grpc_uri: String,
    pub deposit_grpc_uri: String,
    pub withdrawal_grpc_uri: String,
}

impl Config {
    pub fn init() -> Config {
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires_in = std::env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
        let jwt_maxage = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");
        let user_grpc_uri = std::env::var("USER_GRPC_SERVICE_URL").expect("USER_GRPC_SERVICE_URL must be set");
        let account_grpc_uri = std::env::var("ACCOUNT_GRPC_SERVICE_URL").expect("ACCOUNT_GRPC_SERVICE_URL must be set");
        let deposit_grpc_uri = std::env::var("DEPOSIT_GRPC_SERVICE_URL").expect("DEPOSIT_GRPC_SERVICE_URL must be set");
        let withdrawal_grpc_uri = std::env::var("WITHDRAWAL_GRPC_SERVICE_URL").expect("WITHDRAWAL_GRPC_SERVICE_URL must be set");
        
        Config {
            jwt_secret,
            jwt_expires_in,
            jwt_maxage: jwt_maxage.parse::<i32>().unwrap(),
            user_grpc_uri,
            account_grpc_uri,
            deposit_grpc_uri,
            withdrawal_grpc_uri
        }
    }
}
