pub mod historical {
    tonic::include_proto!("historical");
}

use historical::historical_service_client::HistoricalServiceClient;
use tonic::transport::Channel;

pub async fn get_historical_grpc_client(
    uri: String,
) -> Result<HistoricalServiceClient<Channel>, Box<dyn std::error::Error>> {
    let grpc_uri = format!("http://{}", uri); 
    let static_uri = Box::leak(grpc_uri.into_boxed_str());

    let channel = tonic::transport::Channel::from_static(static_uri)
        .connect()
        .await?;

    let client = HistoricalServiceClient::new(channel);

    Ok(client)
}
