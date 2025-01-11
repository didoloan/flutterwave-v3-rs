use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResCardData {
    #[serde(rename = "first_6digits")]
    pub first_6_digits: String,
    #[serde(rename = "last_4digits")]
    pub last_4_digits: String,
    pub issuer: String,
    pub country: String,
    #[serde(rename = "type")]
    pub card_type: String,
    pub expiry: String,
}
