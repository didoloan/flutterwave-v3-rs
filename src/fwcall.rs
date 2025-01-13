use flutterwave_models::{
    charge::{
        ach::{AchReq, AchRes},
        bank_transfer::{BankTransferReq, BankTransferRes},
        direct_charge::{CardChargeReq, CardChargeRes},
    },
    payment_plans::{
        CancelPlanReq, CreatePlanReq, GetPlanReq, GetPlansReq, GetPlansRes, PlanApiRes,
        UpdatePlanReq, UpdatePlanReqBody,
    },
    preauthorization::{
        capture_preauth_charge::{CapturePreAuthChargeReq, CapturePreAuthChargeRes},
        refund_preauth_charge::{RefundPreAuthChargeReq, RefundPreAuthChargeRes},
        void_preauth_charge::{VoidPreAuthChargeReq, VoidPreAuthChargeRes},
    },
    transactions::transaction_verify::{VerifyTransByIdReq, VerifyTransByTxRefReq, VerifyTransRes},
    validate_charge::{ValidateChargeReq, ValidateChargeRes},
    virtual_acct_number::{
        get_bulk_virtual_acct_details::{BulkVirtualAcctDetailsReq, BulkVirtualAcctDetailsRes},
        VirtualAcctBulkCreationReq, VirtualAcctBulkCreationRes, VirtualAcctCreationReq,
        VirtualAcctCreationRes,
    }
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::borrow::Cow;
use std::marker::PhantomData;

#[derive(Debug)]
pub enum Payload<T: Serialize> {
    Plain(T),
    ToEncrypt(T),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResData {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FwError400Res {
    pub status: String,
    pub message: String,
    pub data: Option<ErrorResData>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FwError500Res {
    pub status: String,
    pub message: String,
    pub data: Option<ErrorResData>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ResponseType<T> {
    Success(T),
    Error400(FwError400Res),
    Error500(FwError500Res),
}

#[derive(Debug)]
pub struct FwCall<'a, T: Serialize, R: Deserialize<'a>> {
    #[allow(unused)]
    pub(crate) path: Cow<'a, str>,
    #[allow(unused)]
    pub(crate) method: reqwest::Method,
    #[allow(unused)]
    pub(crate) payload: Option<Payload<T>>,
    _phantom: PhantomData<R>,
}

impl<'a, T: Serialize, R: Deserialize<'a>> FwCall<'a, T, R> {
    fn new(path: Cow<'a, str>, method: reqwest::Method, payload: Option<Payload<T>>) -> Self {
        Self {
            path,
            method,
            payload,
            _phantom: PhantomData,
        }
    }
}

pub trait ToFwCall<'a>
where
    Self: Deserialize<'a> + Serialize + validator::Validate,
    Self::ApiResponse: DeserializeOwned + Serialize,
    Self::ApiRequest: Deserialize<'a> + Serialize + validator::Validate,
{
    type ApiRequest;

    type ApiResponse;

    #[allow(unused)]
    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse>;

    #[allow(unused)]
    fn to_call(
        self,
    ) -> Result<
        FwCall<'a, Self::ApiRequest, Self::ApiResponse>,
        flutterwave_models::errors::FWaveError,
    > {
        self.validate()?;
        Ok(self.get_call())
    }
}

impl<'a> ToFwCall<'a> for CardChargeReq {
    type ApiRequest = Self;

    type ApiResponse = ResponseType<CardChargeRes>;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Borrowed("/v3/charges?type=card_charge"),
            reqwest::Method::POST,
            Some(Payload::ToEncrypt(self)),
        )
    }
}

impl<'a> ToFwCall<'a> for BankTransferReq {
    type ApiRequest = Self;

    type ApiResponse = ResponseType<BankTransferRes>;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Borrowed("/v3/charges?type=bank_transfer"),
            reqwest::Method::POST,
            Some(Payload::ToEncrypt(self)),
        )
    }
}

impl<'a> ToFwCall<'a> for VerifyTransByIdReq {
    type ApiRequest = Self;

    type ApiResponse = ResponseType<VerifyTransRes>;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Owned(format!("/v3/transactions/{}/verify", self.trans_id)),
            reqwest::Method::GET,
            None,
        )
    }
}

impl<'a> ToFwCall<'a> for VerifyTransByTxRefReq {
    type ApiRequest = Self;

    type ApiResponse = ResponseType<VerifyTransRes>;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Owned(format!(
                "/v3/transactions/verify_by_reference?tx_ref={}",
                self.tx_ref
            )),
            reqwest::Method::GET,
            None,
        )
    }
}

impl<'a> ToFwCall<'a> for VirtualAcctCreationReq {
    type ApiRequest = Self;

    type ApiResponse = ResponseType<VirtualAcctCreationRes>;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Borrowed("/v3/virtual-account-numbers"),
            reqwest::Method::POST,
            Some(Payload::Plain(self)),
        )
    }
}

impl<'a> ToFwCall<'a> for VirtualAcctBulkCreationReq {
    type ApiRequest = Self;

    type ApiResponse = ResponseType<VirtualAcctBulkCreationRes>;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Borrowed("/v3/bulk-virtual-account-numbers"),
            reqwest::Method::POST,
            Some(Payload::Plain(self)),
        )
    }
}

impl<'a> ToFwCall<'a> for ValidateChargeReq {
    type ApiRequest = Self;

    type ApiResponse = ResponseType<ValidateChargeRes>;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Borrowed("/v3/validate-charge"),
            reqwest::Method::POST,
            Some(Payload::Plain(self)),
        )
    }
}

impl<'a> ToFwCall<'a> for BulkVirtualAcctDetailsReq {
    type ApiRequest = Self;

    type ApiResponse = ResponseType<BulkVirtualAcctDetailsRes>;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Owned(format!(
                "/v3/bulk-virtual-account-numbers/{}",
                self.batch_id
            )),
            reqwest::Method::GET,
            Some(Payload::Plain(self)),
        )
    }
}

impl<'a> ToFwCall<'a> for CreatePlanReq {
    type ApiRequest = Self;

    type ApiResponse = ResponseType<PlanApiRes>;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Borrowed("/v3/payment-plans"),
            reqwest::Method::POST,
            Some(Payload::Plain(self)),
        )
    }
}

impl<'a> ToFwCall<'a> for GetPlanReq {
    type ApiRequest = Self;

    type ApiResponse = ResponseType<PlanApiRes>;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Owned(format!("/v3/payment-plans/{}", self.id)),
            reqwest::Method::GET,
            None,
        )
    }
}

impl<'a> ToFwCall<'a> for GetPlansReq {
    type ApiRequest = Self;

    type ApiResponse = ResponseType<GetPlansRes>;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Owned(format!(
                "/v3/payment-plans{}",
                serde_qs::to_string(&self).unwrap()
            )),
            reqwest::Method::GET,
            None,
        )
    }
}

impl<'a> ToFwCall<'a> for CancelPlanReq {
    type ApiRequest = Self;

    type ApiResponse = ResponseType<PlanApiRes>;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Owned(format!("/v3/payment-plans/{}/cancel", self.plan_id)),
            reqwest::Method::PUT,
            None,
        )
    }
}

impl<'a> ToFwCall<'a> for UpdatePlanReq {
    type ApiRequest = UpdatePlanReqBody;

    type ApiResponse = ResponseType<PlanApiRes>;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Owned(format!("/v3/payment-plans/{}", self.id)),
            reqwest::Method::PUT,
            Some(Payload::Plain(self.body)),
        )
    }
}

impl<'a> ToFwCall<'a> for AchReq {
    type ApiRequest = Self;

    type ApiResponse = ResponseType<AchRes>;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Borrowed("/v3/charges?type=ach_payment"),
            reqwest::Method::PUT,
            Some(Payload::Plain(self)),
        )
    }
}

impl<'a> ToFwCall<'a> for CapturePreAuthChargeReq {
    type ApiRequest = Self;

    type ApiResponse = ResponseType<CapturePreAuthChargeRes>;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Owned(format!("/v3/charges/{}/capture", self.flw_ref)),
            reqwest::Method::POST,
            Some(Payload::Plain(self)),
        )
    }
}

impl<'a> ToFwCall<'a> for RefundPreAuthChargeReq {
    type ApiRequest = Self;

    type ApiResponse = ResponseType<RefundPreAuthChargeRes>;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Owned(format!("/v3/charges/{}/refund", self.flw_ref)),
            reqwest::Method::POST,
            Some(Payload::Plain(self)),
        )
    }
}

impl<'a> ToFwCall<'a> for VoidPreAuthChargeReq {
    type ApiRequest = Self;

    type ApiResponse = ResponseType<VoidPreAuthChargeRes>;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Owned(format!("/v3/charges/{}/void", self.flw_ref)),
            reqwest::Method::POST,
            Some(Payload::Plain(self)),
        )
    }
}