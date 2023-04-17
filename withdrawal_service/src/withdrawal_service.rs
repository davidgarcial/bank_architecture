use tonic::{Request, Response, Status};
use std::str::FromStr;
use std::sync::Arc;

use mongodb::{
    Collection,
    bson::{doc, Document, oid::ObjectId},
    {options::ClientOptions, Client}
};

pub mod withdrawal {
    tonic::include_proto!("withdrawal");
}

use withdrawal::withdrawal_service_server::{WithdrawalService};
use withdrawal::{
    MakeWithdrawalRequest, MakeWithdrawalResponse,
    CheckAccountBalanceRequest, CheckAccountBalanceResponse
};

#[derive(Debug, Clone)]
pub struct MyWithdrawalService {
    db: Arc<mongodb::Database>,
}

impl MyWithdrawalService {
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
impl WithdrawalService for MyWithdrawalService {
    async fn make_withdrawal(
        &self,
        request: Request<MakeWithdrawalRequest>,
    ) -> Result<Response<MakeWithdrawalResponse>, Status> {
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
    
        let account_doc = match account_doc_option {
            Some(doc) => doc,
            None => return Err(Status::not_found("Account not found")),
        };
    
        let current_balance = account_doc.get_f64("balance").unwrap();
        if current_balance < req.amount {
            return Err(Status::failed_precondition("Insufficient balance for withdrawal"));
        }
    
        let new_balance = current_balance - req.amount;
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
            "type": "withdrawal",
        };
    
        let insert_result = transactions_collection
            .insert_one(new_transaction, None)
            .await
            .map_err(|e| Status::internal(format!("Failed to create transaction: {}", e)))?;
    
        let response = MakeWithdrawalResponse {
            transaction_id: insert_result.inserted_id.as_object_id().unwrap().to_string(),
        };
    
        Ok(Response::new(response))
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
    
