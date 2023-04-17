use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub account_type: AccountType
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AccountType {
    Checking,
    Savings,
}
