use bson::Bson;
use log::{error, info};
use prost_types::Timestamp;
use tonic::{Request, Response, Status};
use futures::stream::TryStreamExt;

use std::{
    convert::Infallible,
    fmt::{self, Display, Formatter},
    str::FromStr,
    sync::Arc
};

use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Collection,
    {options::ClientOptions, Client}
};

pub mod account {
    tonic::include_proto!("account");
}

use account::account_service_server::AccountService;
use account::{
    Account, AccountType, CreateAccountRequest, CreateAccountResponse, GetAccountRequest,
    GetAccountResponse, GetUserAccountsRequest, GetUserAccountsResponse,
    UpdateAccountRequest, UpdateAccountResponse
};

impl Display for AccountType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let account_type_str = match self {
            AccountType::Checking => "CHECKING",
            AccountType::Savings => "SAVINGS",
        };

        write!(f, "{}", account_type_str)
    }
}

impl FromStr for AccountType {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CHECKING" => Ok(AccountType::Checking),
            "SAVINGS" => Ok(AccountType::Savings),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MyAccountService {
    db: Arc<mongodb::Database>,
}

impl MyAccountService {
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

pub fn account_type_to_string(value: i32) -> String {
    match value {
        0 => "CHECKING".to_string(),
        1 => "SAVINGS".to_string(),
        _ => panic!("Invalid account type value: {}", value),
    }
}

#[tonic::async_trait]
impl AccountService for MyAccountService {
    async fn create_account(
        &self,
        request: Request<CreateAccountRequest>,
    ) -> Result<Response<CreateAccountResponse>, Status> {
        let req = request.into_inner();
        let accounts_collection: Collection<Document> = self.db.collection("accounts");

        // Log the account creation request
        info!("Creating account for user_id: {}", req.user_id);

        let new_account = doc! {
            "user_id": req.user_id,
            "account_type": account_type_to_string(req.account_type),
            "account_name": req.account_name,
            "balance": 0.0
        };

        let insert_result = accounts_collection
            .insert_one(new_account, None)
            .await
            .map_err(|e| Status::internal(format!("Failed to create account: {}", e)))?;

        let account_id = insert_result
            .inserted_id
            .as_object_id()
            .map(|id| id.to_hex())
            .ok_or_else(|| {
                Status::internal("Failed to create account: missing inserted_id".to_string())
            })?;

        // Log the successful account creation
        info!(
            "Account created successfully with account_id: {}",
            account_id
        );

        let response = CreateAccountResponse { account_id };
        Ok(Response::new(response))
    }

    async fn get_account(
        &self,
        request: Request<GetAccountRequest>,
    ) -> Result<Response<GetAccountResponse>, Status> {
        let req = request.into_inner();
        let accounts_collection: Collection<Document> = self.db.collection("accounts");

        // Log the account fetching request
        info!("Fetching account with account_id: {}", req.account_id);

        let account_id = match ObjectId::from_str(&req.account_id) {
            Ok(oid) => oid,
            Err(_) => return Err(Status::invalid_argument("Invalid account id")),
        };

        let filter = doc! {
            "_id": account_id,
        };

        let account_doc_option = accounts_collection
            .find_one(filter, None)
            .await
            .map_err(|e| Status::internal(format!("Failed to get account: {}", e)))?;

        if let Some(account_doc) = account_doc_option {
            let response = GetAccountResponse {
                account: Some(account::Account {
                    account_id: account_doc.get_object_id("_id").unwrap().to_string(),
                    user_id: account_doc.get_str("user_id").unwrap().to_string(),
                    account_name: account_doc.get_str("account_name").unwrap().to_string(),
                    account_type: AccountType::from_str(
                        account_doc.get_str("account_type").unwrap(),
                    )
                    .unwrap() as i32,
                    balance: account_doc.get_f64("balance").unwrap(),
                    created_at: None, // We didn't store created_at and updated_at in the database, so we can't return them here.
                    updated_at: None,
                }),
            };

            // Log the successful account fetching
            info!(
                "Account fetched successfully with account_id: {}",
                req.account_id
            );

            Ok(Response::new(response))
        } else {
            error!("Account not found with account_id: {}", req.account_id);
            Err(Status::not_found("Account not found"))
        }
    }

    async fn update_account(
        &self,
        request: Request<UpdateAccountRequest>,
    ) -> Result<Response<UpdateAccountResponse>, Status> {
        let req = request.into_inner();
        let accounts_collection: Collection<Document> = self.db.collection("accounts");

        // Log the account updating request
        info!("Updating account with account_id: {}", req.account_id);

        let object_id = match ObjectId::from_str(&req.account_id) {
            Ok(oid) => oid,
            Err(_) => return Err(Status::invalid_argument("Invalid account id")),
        };

        let filter_update = doc! {
            "_id": object_id,
        };

        let update = doc! {
            "$set": {
                "balance": req.balance,
                "updated_at": Bson::DateTime(bson::DateTime::now())
            }
        };

        let _ = accounts_collection
            .update_one(filter_update, update, None)
            .await
            .map_err(|e| Status::internal(format!("Failed to update account: {}", e)))?;

        let filter_find = doc! {
            "_id": object_id,
        };

        let account_doc_option = accounts_collection
            .find_one(filter_find, None)
            .await
            .map_err(|e| Status::internal(format!("Failed to get account: {}", e)))?;

        let mut response = UpdateAccountResponse { account: None };

        if let Some(account_doc) = account_doc_option {
            let created_at = account_doc.get_datetime("created_at").unwrap();
            let updated_at = account_doc.get_datetime("updated_at").unwrap();

            let account = account::Account {
                account_id: account_doc.get_object_id("_id").unwrap().to_string(),
                user_id: account_doc.get_str("user_id").unwrap().to_string(),
                account_type: AccountType::from_str(account_doc.get_str("account_type").unwrap())
                    .unwrap() as i32,
                account_name: account_doc.get_str("account_name").unwrap().to_string(),
                balance: account_doc.get_f64("balance").unwrap(),
                created_at: Some(Timestamp {
                    seconds: created_at.timestamp_millis() / 1_000,
                    nanos: ((created_at.timestamp_millis() % 1_000) * 1_000_000) as i32,
                }),
                updated_at: Some(Timestamp {
                    seconds: updated_at.timestamp_millis() / 1_000,
                    nanos: ((updated_at.timestamp_millis() % 1_000) * 1_000_000) as i32,
                }),
            };
            response.account = Some(account);
            // Log the successful account update
            info!(
                "Account updated successfully with account_id: {}",
                req.account_id
            );
        } else {
            error!("Account not found with account_id: {}", req.account_id);
        }

        Ok(Response::new(response))
    }

    async fn get_user_accounts(
        &self,
        request: Request<GetUserAccountsRequest>,
    ) -> Result<Response<GetUserAccountsResponse>, Status> {
        let req = request.into_inner();
        let accounts_collection: Collection<Document> = self.db.collection("accounts");

        // Log the account creation request
        info!("Getting accounts for user_id: {}", req.user_id);

        let filter = doc! {
            "user_id": req.user_id,
        };

        let mut cursor = accounts_collection
            .find(filter, None)
            .await
            .map_err(|e| Status::internal(format!("Failed to get accounts: {}", e)))?;

        let mut accounts = Vec::new();
        while let Some(result) = cursor
            .try_next()
            .await
            .map_err(|e| Status::internal(format!("Failed to get historical: {}", e)))?
        {
            let account = Account {
                account_id: result.get_str("account_id").unwrap().to_string(),
                user_id: result.get_str("user_id").unwrap().to_string(),
                account_type: match result.get_i32("transaction_type").unwrap() {
                    0 => AccountType::Checking as i32,
                    1 => AccountType::Savings as i32,
                    _ => return Err(Status::internal("Invalid account type")),
                },
                balance: result.get_f64("balance").unwrap(),
                created_at: None,
                updated_at: None,
                account_name: result.get_str("account_name").unwrap().to_string()
            };
            accounts.push(account);
        }

        let response = GetUserAccountsResponse { accounts };
        Ok(Response::new(response))
    }
}
