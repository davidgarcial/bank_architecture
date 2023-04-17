pub mod withdrawal {
    tonic::include_proto!("withdrawal");
}

use withdrawal::withdrawal_service_client::WithdrawalServiceClient;
use tonic::transport::Channel;

pub async fn get_withdrawal_grpc_client(
    uri: String,
) -> Result<WithdrawalServiceClient<Channel>, Box<dyn std::error::Error>> {
    let grpc_uri = format!("http://{}", uri); 
    let static_uri = Box::leak(grpc_uri.into_boxed_str());

    let channel = tonic::transport::Channel::from_static(static_uri)
        .connect()
        .await?;

    let client = WithdrawalServiceClient::new(channel);

    Ok(client)
}
