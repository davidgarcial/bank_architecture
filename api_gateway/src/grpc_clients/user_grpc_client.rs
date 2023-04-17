pub mod user_service {
    tonic::include_proto!("user_service");
}

use user_service::{user_service_client::UserServiceClient};
use tonic::transport::Channel;

pub async fn get_user_grpc_client(
    uri: String,
) -> Result<UserServiceClient<Channel>, Box<dyn std::error::Error>> {
    let grpc_uri = format!("http://{}", uri); 
    let static_uri = Box::leak(grpc_uri.into_boxed_str());

    let channel = tonic::transport::Channel::from_static(static_uri)
        .connect()
        .await?;

    let client = UserServiceClient::new(channel);

    Ok(client)
}
