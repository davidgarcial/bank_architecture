pub mod deposit {
    tonic::include_proto!("deposit");
}

use deposit::deposit_service_client::DepositServiceClient;
use tonic::transport::Channel;

pub async fn get_deposit_grpc_client(
    uri: String,
) -> Result<DepositServiceClient<Channel>, Box<dyn std::error::Error>> {
    let grpc_uri = format!("http://{}", uri); 
    let static_uri = Box::leak(grpc_uri.into_boxed_str());

    let channel = tonic::transport::Channel::from_static(static_uri)
        .connect()
        .await?;

    let client = DepositServiceClient::new(channel);

    Ok(client)
}
