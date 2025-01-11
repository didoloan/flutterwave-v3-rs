use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

use crate::common::card_data_res::ResCardData;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CardChargeReq {
    #[validate(credit_card)]
    pub card_number: String,
    #[validate(length(min = 3, max = 4))]
    pub cvv: String,
    #[validate(length(min = 2, max = 2))]
    pub expiry_month: String,
    #[validate(length(min = 2, max = 2))]
    pub expiry_year: String,
    #[validate(length(min = 3, max = 3))]
    pub currency: Option<String>,
    pub amount: String,
    #[validate(email, length(max = 100))]
    pub email: String,
    pub fullname: Option<String>,
    #[validate(phone, length(max = 50))]
    pub phone_number: Option<String>,
    #[validate(length(max = 100))]
    pub tx_ref: String,
    pub preauthorize: Option<bool>,
    #[validate(url)]
    pub redirect_url: Option<String>,
    pub client_ip: Option<String>,
    pub device_fingerprint: Option<String>,
    pub payment_plan: Option<String>,
    pub a_statusreasoncode: String,
    pub is_custom_3ds_enabled: bool,
    pub a_time: NaiveDateTime,
    pub meta: HashMap<String, String>,
    pub subaccounts: Vec<SubAccount>,
}

#[derive(Debug)]
pub enum SubAccountChargeType {
    Flat,
    Percentage,
    FlatSubAccount,
}

impl<'de> serde::Deserialize<'de> for SubAccountChargeType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SubAccountChargeTypeVisitor;

        impl serde::de::Visitor<'_> for SubAccountChargeTypeVisitor {
            type Value = SubAccountChargeType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Expecting one of flat, percentage or flatsubaccount!")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v.to_ascii_lowercase().as_str() {
                    "flat" => Ok(SubAccountChargeType::Flat),
                    "percentage" => Ok(SubAccountChargeType::Percentage),
                    "flat_subaccount" => Ok(SubAccountChargeType::FlatSubAccount),
                    _ => Err(serde::de::Error::custom("Invalid Charge Type!")),
                }
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v.as_str() {
                    "flat" => Ok(SubAccountChargeType::Flat),
                    "percentage" => Ok(SubAccountChargeType::Percentage),
                    "flat_subaccount" => Ok(SubAccountChargeType::FlatSubAccount),
                    _ => Err(serde::de::Error::custom("Invalid Charge Type!")),
                }
            }
        }
        deserializer.deserialize_str(SubAccountChargeTypeVisitor)
    }
}

impl serde::Serialize for SubAccountChargeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            SubAccountChargeType::Flat => serializer.serialize_str("flat"),
            SubAccountChargeType::Percentage => serializer.serialize_str("percentage"),
            SubAccountChargeType::FlatSubAccount => serializer.serialize_str("flat_subaccount"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubAccount {
    pub id: String,
    pub transaction_split_ratio: u8,
    pub transaction_charge_type: SubAccountChargeType,
    pub transaction_charge: u64,
}

#[derive(Serialize, Deserialize)]
pub struct CardChargeRes {
    pub status: String,
    pub message: String,
    pub data: Data,
    pub meta: Meta,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
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
    pub currency: String,
    pub ip: String,
    pub narration: String,
    pub status: String,
    pub auth_url: String,
    pub payment_type: String,
    pub fraud_status: String,
    pub charge_type: String,
    pub created_at: String,
    pub account_id: i64,
    pub customer: Customer,
    pub card: ResCardData,
}

#[derive(Serialize, Deserialize)]
pub struct Customer {
    pub id: i64,
    pub phone_number: Option<String>,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct Meta {
    pub authorization: Authorization,
}

#[derive(Serialize, Deserialize)]
pub struct Authorization {
    pub mode: String,
    pub pin: i32,
    pub city: String,
    pub address: String,
    pub state: String,
    pub country: String,
    pub zipcode: i32,
    pub endpoint: String,
}
