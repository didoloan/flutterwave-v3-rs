use super::Interval;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanApiRes {
    pub status: String,
    pub message: String,
    pub data: PlanApiResData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanApiResData {
    pub id: i32,
    pub name: String,
    pub amount: i32,
    pub interval: Interval,
    pub duration: i32,
    pub status: String,
    pub currency: String,
    pub plan_token: String,
    pub created_at: String,
}
