use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DepositRequest {
    pub from_account_id: String,
    pub to_account_id: String,
    pub amount: f64,
    pub is_bank_agent: bool
}
 