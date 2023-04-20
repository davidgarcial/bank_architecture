use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub account_type: AccountType,
    pub account_name: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AccountType {
    Checking,
    Savings,
}
