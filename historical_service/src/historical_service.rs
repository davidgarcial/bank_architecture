use std::{str::FromStr, ptr::null};
use std::sync::Arc;
use tonic::{Request, Response, Status};

use futures::stream::TryStreamExt;
use log::{info};

use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::FindOptions,
    Collection,
    {options::ClientOptions, Client}
};

use historical::historical_service_server::HistoricalService;
use historical::{
    GetTransactionHistoryRequest, GetTransactionHistoryResponse, Transaction, TransactionType
};

pub mod historical {
    tonic::include_proto!("historical");
}

#[derive(Debug, Clone)]
pub struct MyHistoricalService {
    db: Arc<mongodb::Database>,
}

impl MyHistoricalService {
    pub async fn new(uri: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database("bank");
        Ok(Self { db: Arc::new(db) })
    }

    pub async fn test_connection(&self) -> Result<(), mongodb::error::Error> {
        let _ = self.db.run_command(doc! { "ping": 1 }, None).await?;
        Ok(())
    }
}

#[tonic::async_trait]
impl HistoricalService for MyHistoricalService {
    async fn get_transaction_history(
        &self,
        request: Request<GetTransactionHistoryRequest>,
    ) -> Result<Response<GetTransactionHistoryResponse>, Status> {
        let account_id = request.into_inner().account_id;
        let transactions_collection: Collection<Document> = self.db.collection("transactions");

        let object_id = match ObjectId::from_str(&account_id) {
            Ok(oid) => oid,
            Err(_) => return Err(Status::invalid_argument("Invalid account id")),
        };

        let filter = doc! {
            "account_id": object_id,
        };

        let mut options = FindOptions::default();
        options.sort = Some(doc! {
            "timestamp": -1, // sort by timestamp in descending order
        });

        let mut cursor = transactions_collection
            .find(filter, options)
            .await
            .map_err(|e| Status::internal(format!("Failed to get historical: {}", e)))?;

        let mut transactions = Vec::new();
        while let Some(result) = cursor
            .try_next()
            .await
            .map_err(|e| Status::internal(format!("Failed to get historical: {}", e)))?
        {
            let transaction = Transaction {
                transaction_id: result.get_object_id("_id").unwrap().to_string(),
                account_id: result.get_object_id("account_id").unwrap().to_string(),
                transaction_type: match result.get_str("type").unwrap() {
                    "Deposit" => TransactionType::Deposit as i32,
                    "Withdrawal" => TransactionType::Withdrawal as i32,
                    _ => return Err(Status::internal("Invalid account type")),
                },
                amount: result.get_f64("amount").unwrap(),
                timestamp: Default::default()
            };
            transactions.push(transaction);
        }

        info!("Fetched transaction history for account {}", account_id);

        let response = GetTransactionHistoryResponse { transactions };
        Ok(Response::new(response))
    }
}
