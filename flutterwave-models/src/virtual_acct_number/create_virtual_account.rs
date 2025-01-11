use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct VirtualAcctCreationReq {
    #[validate(email)]
    pub email: String,
    pub bvn: String,
    pub amount: Option<i32>,
    pub tx_ref: Option<String>,
    pub is_permanent: Option<bool>,
    pub narration: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct VirtualAcctCreateResData {
    pub response_code: String,
    pub response_message: String,
    pub flw_ref: String,
    pub order_ref: String,
    pub account_number: String,
    pub frequency: String,
    pub bank_name: String,
    pub created_at: String,
    pub expiry_date: String,
    pub note: String,
    pub amount: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct VirtualAcctCreationRes {
    pub status: String,
    pub message: String,
    pub data: VirtualAcctCreateResData,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct VirtualAcctBulkCreationReq {
    pub accounts: i32,
    #[validate(email)]
    pub email: String,
    pub is_permanent: bool,
    pub frequency: Option<i32>,
    pub tx_ref: Option<String>,
    pub amount: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualAcctBulkCreationRes {
    pub status: String,
    pub message: String,
    pub data: VirtualAcctBulkCreateResData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualAcctBulkCreateResData {
    pub batch_id: String,
    pub response_code: String,
    pub response_message: String,
}
