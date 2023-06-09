use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccountRequestModel {
    pub account_id: String,
    pub balance: f64,
}
