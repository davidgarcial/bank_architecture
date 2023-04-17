pub mod account {
    tonic::include_proto!("account");
}

use account::account_service_client::AccountServiceClient;
use tonic::transport::Channel;

pub async fn get_account_grpc_client(
    uri: String,
) -> Result<AccountServiceClient<Channel>, Box<dyn std::error::Error>> {
    let grpc_uri = format!("http://{}", uri); 
    let static_uri = Box::leak(grpc_uri.into_boxed_str());

    let channel = tonic::transport::Channel::from_static(static_uri)
        .connect()
        .await?;

    let client = AccountServiceClient::new(channel);

    Ok(client)
}
