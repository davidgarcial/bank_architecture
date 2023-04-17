use tonic::{Request, Response, Status};
use std::str::FromStr;
use std::sync::Arc;

use mongodb::{
    Collection,
    bson::{doc, Document, oid::ObjectId},
    {options::ClientOptions, Client}
};

pub mod deposit {
    tonic::include_proto!("deposit");
}

use deposit::deposit_service_server::DepositService;
use deposit::{
    MakeDepositRequest, MakeDepositResponse, 
    CheckAccountBalanceRequest, CheckAccountBalanceResponse
};

// The `Arc` functionality in Rust allows for sharing read-only data
// safely between multiple threads. It is a thread-safe reference-counting
// smart pointer that increments the reference count when a new `Arc`
// instance points to the same data, and decrements the count when an
// `Arc` instance is dropped. When the reference count reaches zero,
// the data is deallocated.

// `Arc` is useful for sharing read-only data between threads without
// requiring explicit synchronization mechanisms like locks or mutexes.
// However, if you need to mutate the data, you must use other synchronization
// primitives, such as `Mutex` or `RwLock`, to guarantee exclusive access
// to the data when it is being modified.
#[derive(Debug, Clone)]
pub struct MyDepositService {
    db: Arc<mongodb::Database>
}

impl MyDepositService {
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
impl DepositService for MyDepositService {
    async fn make_deposit(
        &self,
        request: Request<MakeDepositRequest>,
    ) -> Result<Response<MakeDepositResponse>, Status> {
        let req = request.into_inner();
        let accounts_collection: Collection<Document> = self.db.collection("accounts");
        let transactions_collection: Collection<Document> = self.db.collection("transactions");

        let object_id = match ObjectId::from_str(&req.account_id) {
            Ok(oid) => oid,
            Err(_) => return Err(Status::invalid_argument("Invalid account id")),
        };

        let filter = doc! {
            "_id": object_id,
        };

        let account_doc_option = accounts_collection
            .find_one(filter.clone(), None)
            .await
            .map_err(|e| Status::internal(format!("Failed to get account: {}", e)))?;

        if let Some(account_doc) = account_doc_option {
            if req.is_bank_agent || account_doc.get_f64("balance").unwrap() >= req.amount {
                let new_balance = account_doc.get_f64("balance").unwrap() + req.amount;
                let update = doc! {
                    "$set": {
                        "balance": new_balance,
                    }
                };

                // Update the account balance
                accounts_collection
                    .update_one(filter, update, None)
                    .await
                    .map_err(|e| Status::internal(format!("Failed to update account balance: {}", e)))?;

                // Record the transaction
                let new_transaction = doc! {
                    "account_id": object_id,
                    "amount": req.amount,
                    "type": "deposit",
                };

                let insert_result = transactions_collection
                    .insert_one(new_transaction, None)
                    .await
                    .map_err(|e| Status::internal(format!("Failed to create transaction: {}", e)))?;

                let response = MakeDepositResponse {
                    transaction_id: insert_result.inserted_id.as_object_id().unwrap().to_string(),
                };

                Ok(Response::new(response))
            } else {
                Err(Status::failed_precondition("Insufficient balance or not a bank agent for deposit"))
            }
        } else {
            Err(Status::not_found("Account not found"))
        }
    }

    async fn check_account_balance(
        &self,
        request: Request<CheckAccountBalanceRequest>,
    ) -> Result<Response<CheckAccountBalanceResponse>, Status> {
        let req = request.into_inner();
        let accounts_collection: Collection<Document> = self.db.collection("accounts");

        let object_id = match ObjectId::from_str(&req.account_id) {
            Ok(oid) => oid,
            Err(_) => return Err(Status::invalid_argument("Invalid account id")),
        };

        let filter = doc! {
            "_id": object_id,
        };

        let account_doc_option = accounts_collection
            .find_one(filter, None)
            .await
            .map_err(|e| Status::internal(format!("Failed to get account balance: {}", e)))?;

        if let Some(account_doc) = account_doc_option {
            let response = CheckAccountBalanceResponse {
                balance: account_doc.get_f64("balance").unwrap(),
            };

            Ok(Response::new(response))
        } else {
            Err(Status::not_found("Account not found"))
        }
    }
}
