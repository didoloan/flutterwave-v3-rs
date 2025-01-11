use super::{Interval, PlanApiResData};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GetPlansReq {
    pub from: String,
    pub to: String,
    pub page: i32,
    pub amount: i32,
    pub currency: String,
    pub interval: Interval,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPlansRes {
    pub status: String,
    pub message: String,
    pub meta: GetPlansResMeta,
    pub data: Vec<PlanApiResData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPlansResMeta {
    pub page_info: GetPlansResMetaPageInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPlansResMetaPageInfo {
    pub total: i32,
    pub current_page: i32,
    pub total_pages: i32,
}
