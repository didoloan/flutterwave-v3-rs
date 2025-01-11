use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct BulkVirtualAcctDetailsReq {
    pub batch_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkVirtualAcctDetailsRes {
    pub status: String,
    pub message: String,
    pub data: BulkVirtAcctDetailsResData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkVirtAcctDetailsResData {
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
    pub amount: String,
}
