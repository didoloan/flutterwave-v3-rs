use super::Interval;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreatePlanReq {
    pub amount: i32,
    pub name: String,
    pub interval: Interval,
    pub duration: Option<i32>,
}
