use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct BankTransferReq {
    pub amount: i32,
    #[validate(email, length(max = 100))]
    pub email: String,
    #[validate(length(min = 3, max = 3))]
    pub currency: String,
    pub tx_ref: String,
    pub fullname: Option<String>,
    #[validate(phone, length(max = 50))]
    pub phone_number: Option<String>,
    pub client_ip: Option<String>,
    pub device_fingerprint: Option<String>,
    pub meta: HashMap<String, String>,
    pub subaccounts: Vec<BankTransferSubAcct>,
    pub narration: Option<String>,
    pub is_permanent: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BankTransferSubAcct {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BankTransferRes {
    pub status: String,
    pub message: String,
    pub meta: BankTransferMeta,
    pub subaccounts: Vec<SubAccount>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubAccount {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BankTransferMeta {
    pub authorization: BankTransferAuth,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BankTransferAuth {
    pub transfer_reference: String,
    pub transfer_account: String,
    pub transfer_bank: String,
    pub account_expiration: String,
    pub transfer_note: String,
    pub transfer_amount: i32,
    pub mode: String,
}
