// use std::cell::RefCell;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdatePlanReq {
    pub id: i32,
    pub body: UpdatePlanReqBody, // pub body: RefCell<UpdatePlanReqBody>
}

#[derive(Debug, Serialize, Deserialize, Validate, Default)]
pub struct UpdatePlanReqBody {
    pub name: String,
    pub status: PlanStatus,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum PlanStatus {
    #[default]
    Active,
    Inactive,
}
