use mongodb::{
    Collection,
    bson::{doc, Document},
    {options::ClientOptions, Client}
};
use std::env;

lazy_static::lazy_static! {
    static ref DATABASE: String = env::var("DATABASE").unwrap_or_else(|_| "bank".to_string());
}

pub async fn get_collection(name: &str) -> Result<Collection<Document>, mongodb::error::Error> {
    let mongodb_uri = env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let client_options = ClientOptions::parse(&mongodb_uri).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database(&*DATABASE);
    let collection = db.collection(name);
    Ok(collection)
}

pub async fn test_connection() -> Result<(), mongodb::error::Error> {
    let mongodb_uri = env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let client_options = ClientOptions::parse(&mongodb_uri).await?;
    let client = Client::with_options(client_options)?;
    let _ = client.database(&*DATABASE).run_command(doc! { "ping": 1 }, None).await?;
    Ok(())
}
 