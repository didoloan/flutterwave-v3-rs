use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AchReq {
    pub amount: i32,
    pub currency: String,
    pub email: String,
    pub tx_ref: String,
    pub fullname: String,
    pub phone_number: String,
    pub client_ip: String,
    pub device_fingerprint: String,
    pub meta: HashMap<String, String>,
    pub redirect_url: String,
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchRes {
    pub status: String,
    pub message: String,
    pub data: AchResData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchResData {
    pub id: i64,
    pub tx_ref: String,
    pub flw_ref: String,
    pub device_fingerprint: String,
    pub amount: i64,
    pub charged_amount: i64,
    pub app_fee: f64,
    pub merchant_fee: i64,
    pub processor_response: String,
    pub auth_model: String,
    pub auth_url: String,
    pub currency: String,
    pub ip: String,
    pub narration: String,
    pub status: String,
    pub payment_type: String,
    pub fraud_status: String,
    pub charge_type: String,
    pub created_at: String,
    pub account_id: i64,
    pub redirect_url: String,
    pub customer: AchResCustomer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchResCustomer {
    pub id: i64,
    pub phone_number: Option<String>,
    pub name: String,
    pub email: String,
    pub created_at: String,
}
