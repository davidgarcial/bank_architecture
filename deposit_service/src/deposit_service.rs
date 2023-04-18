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
    
        let from_account_id = ObjectId::from_str(&req.from_account_id)
            .map_err(|_| Status::invalid_argument("Invalid from account id"))?;
        let to_account_id = ObjectId::from_str(&req.to_account_id)
            .map_err(|_| Status::invalid_argument("Invalid to account id"))?;
    
        let from_filter = doc! {
            "_id": from_account_id.clone(),
        };
        let to_filter = doc! {
            "_id": to_account_id.clone(),
        };
    
        let from_account_doc_option = accounts_collection
            .find_one(from_filter.clone(), None)
            .await
            .map_err(|e| Status::internal(format!("Failed to get from account: {}", e)))?;
    
        let to_account_doc_option = accounts_collection
            .find_one(to_filter.clone(), None)
            .await
            .map_err(|e| Status::internal(format!("Failed to get to account: {}", e)))?;
    
        if let (Some(from_account_doc), Some(to_account_doc)) =
            (from_account_doc_option, to_account_doc_option)
        {
            if req.is_bank_agent || from_account_doc.get_f64("balance").unwrap() >= req.amount {
                let new_from_balance = from_account_doc.get_f64("balance").unwrap() - req.amount;
                let new_to_balance = to_account_doc.get_f64("balance").unwrap() + req.amount;
    
                let from_update = doc! {
                    "$set": {
                        "balance": new_from_balance,
                    }
                };
                let to_update = doc! {
                    "$set": {
                        "balance": new_to_balance,
                    }
                };
    
                // Update the account balances
                accounts_collection
                    .update_one(from_filter, from_update, None)
                    .await
                    .map_err(|e| {
                        Status::internal(format!("Failed to update from account balance: {}", e))
                    })?;
                accounts_collection
                    .update_one(to_filter, to_update, None)
                    .await
                    .map_err(|e| {
                        Status::internal(format!("Failed to update to account balance: {}", e))
                    })?;
    
                // Record the transaction
                let new_transaction = doc! {
                    "from_account_id": from_account_id,
                    "to_account_id": to_account_id,
                    "amount": req.amount,
                    "type": "Deposit",
                };
    
                let transactions_collection: Collection<Document> = self.db.collection("transactions");
                let _insert_result = transactions_collection
                    .insert_one(new_transaction, None)
                    .await
                    .map_err(|e| {
                        Status::internal(format!("Failed to create transaction: {}", e))
                    })?;
    
                let response = MakeDepositResponse { success: true };
    
                Ok(Response::new(response))
            } else {
                Err(Status::failed_precondition(
                    "Insufficient balance or not a bank agent for deposit",
                ))
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
