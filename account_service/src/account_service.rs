use tonic::{Request, Response, Status};
use prost_types::Timestamp;
use bson::Bson;
use uuid::Uuid;

use std::{
    fmt::{self, Display, Formatter},
    convert::Infallible,
    str::FromStr,
    sync::Arc
};

use mongodb::{
    Collection,
    bson::{doc, Document, oid::ObjectId},
    {options::ClientOptions, Client}
};

pub mod account {
    tonic::include_proto!("account");
}

use account::account_service_server::AccountService;
use account::{
    AccountType, 
    CreateAccountRequest, CreateAccountResponse,
    GetAccountRequest, GetAccountResponse,
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

#[tonic::async_trait]
impl AccountService for MyAccountService {
    async fn create_account(
        &self,
        request: Request<CreateAccountRequest>,
    ) -> Result<Response<CreateAccountResponse>, Status> {
        let req = request.into_inner();
        let accounts_collection: Collection<Document> = self.db.collection("accounts");

        let new_account = doc! {
            "uuid": Uuid::new_v4().to_string(),
            "user_id": req.user_id,
            "account_type": req.account_type.to_string(),
            "balance": 0.0
        };

        let insert_result = accounts_collection
            .insert_one(new_account, None)
            .await
            .map_err(|e| Status::internal(format!("Failed to create account: {}", e)))?;

        let response = CreateAccountResponse {
            account_id: insert_result.inserted_id.as_str().unwrap().to_string(),
        };

        Ok(Response::new(response))
    }

    async fn get_account(
        &self,
        request: Request<GetAccountRequest>,
    ) -> Result<Response<GetAccountResponse>, Status> {
        let req = request.into_inner();
        let accounts_collection: Collection<Document> = self.db.collection("accounts");

        let filter = doc! {
            "_id": &req.account_id,
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
                    account_type: AccountType::from_str(account_doc.get_str("account_type").unwrap()).unwrap() as i32,
                    balance: account_doc.get_f64("balance").unwrap(),
                    created_at: None, // We didn't store created_at and updated_at in the database, so we can't return them here.
                    updated_at: None,
                }),
            };
            Ok(Response::new(response))
        } else {
            Err(Status::not_found("Account not found"))
        }
    }
    
    async fn update_account(
        &self,
        request: Request<UpdateAccountRequest>,
    ) -> Result<Response<UpdateAccountResponse>, Status> {
        let req = request.into_inner();
        let accounts_collection: Collection<Document> = self.db.collection("accounts");
    
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
                account_type: AccountType::from_str(account_doc.get_str("account_type").unwrap()).unwrap() as i32,
                balance: account_doc.get_f64("balance").unwrap(),
                created_at: Some(Timestamp {
                    seconds: created_at.timestamp_millis() / 1_000,
                    nanos: ((created_at.timestamp_millis() % 1_000) * 1_000_000) as i32,
                }),
                updated_at: Some(Timestamp {
                    seconds: updated_at.timestamp_millis() / 1_000,
                    nanos: ((updated_at.timestamp_millis() % 1_000) * 1_000_000) as i32,
                })
            };
            response.account = Some(account);
        }
    
        Ok(Response::new(response))
    }
    
}