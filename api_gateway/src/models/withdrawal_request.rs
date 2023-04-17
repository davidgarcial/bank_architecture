use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WithdrawalRequest {
    pub account_id: String,
    pub amount: f64,
}
