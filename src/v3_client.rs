use crate::fwcall::{Payload, ToFwCall};
use async_symm_crypto::AsyncEncryption;
use flutterwave_models::{
    charge::{ach::AchReq, bank_transfer::BankTransferReq, direct_charge::CardChargeReq},
    encrypted_payload::EncryptedPayload,
    errors::FWaveError,
    transactions::transaction_verify::{VerifyTransByIdReq, VerifyTransByTxRefReq},
    validate_charge::ValidateChargeReq,
    virtual_acct_number::{
        get_bulk_virtual_acct_details::BulkVirtualAcctDetailsReq, VirtualAcctBulkCreationReq,
        VirtualAcctCreationReq,
    },
};
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE},
    Url,
};
use std::str::FromStr;

macro_rules! generate_client_method {
    ($t:ty, $func_name:ident) => {
        #[allow(unused)]
        async fn $func_name(&self, req: $t) -> Result<<$t as ToFwCall>::ApiResponse, FWaveError> {
            self.make_v3_request(req).await
        }
    };
}

static BASE_URL: &str = "https://api.flutterwave.com/";

pub struct FWV3Client<'a> {
    #[allow(unused)]
    enc_key: &'a str,
    #[allow(unused)]
    public: &'a str,
    #[allow(unused)]
    secret: &'a str,
    client: reqwest::Client,
    crypt: AsyncEncryption<'a>,
}

impl<'a> FWV3Client<'a> {
    pub fn new(secret_key: &'a str, public_key: &'a str, encryption_key: &'a str) -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.append(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        default_headers.append(ACCEPT, HeaderValue::from_static("application/json"));

        let client = reqwest::ClientBuilder::new()
            .https_only(true)
            .default_headers(default_headers)
            .build()
            .unwrap();

        Self {
            secret: secret_key,
            public: public_key,
            enc_key: encryption_key,
            client,
            crypt: AsyncEncryption::new(
                openssl::symm::Cipher::des_ede3_ecb(),
                secret_key.as_bytes(),
                None,
            ),
        }
    }

    async fn make_v3_request<'b, C: ToFwCall<'b> + 'b>(
        &'b self,
        call: C,
    ) -> Result<C::ApiResponse, FWaveError> {
        let call = call.get_call();

        let mut request = self
            .client
            .request(
                call.method.clone(),
                Url::from_str(BASE_URL)
                    .unwrap()
                    .join(call.path.as_ref())
                    .unwrap(),
            )
            .bearer_auth(self.secret)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json");

        if [reqwest::Method::PUT, reqwest::Method::POST].contains(&call.method) {
            match &call.payload {
                Some(Payload::Plain(pload)) => {
                    request = request.json(&pload);
                }
                Some(Payload::ToEncrypt(pload)) => {
                    let to_enc_bytes = serde_json::to_string(pload)?.as_bytes().to_vec();
                    let encrypted_bytes = self.crypt.encrypt(&to_enc_bytes).await.unwrap();
                    request = request.json(&EncryptedPayload::new(openssl::base64::encode_block(
                        &encrypted_bytes,
                    )));
                }
                None => {}
            }
        }

        Ok(request.send().await?.json::<C::ApiResponse>().await?)
    }

    generate_client_method!(VirtualAcctCreationReq, create_virtual_acct_no);
    generate_client_method!(VirtualAcctBulkCreationReq, create_bulk_virtual_acct_no);
    generate_client_method!(CardChargeReq, initiate_card_charge);
    generate_client_method!(BankTransferReq, initiate_bank_transfer);
    generate_client_method!(VerifyTransByIdReq, verify_trans_by_id);
    generate_client_method!(VerifyTransByTxRefReq, verify_trans_by_txref);
    generate_client_method!(ValidateChargeReq, validate_charge);
    generate_client_method!(BulkVirtualAcctDetailsReq, get_bulk_virtual_acct_details);
    generate_client_method!(AchReq, initiate_ach_payment);
}

#[tokio::test]
async fn test_creat_acct() {
    let client = FWV3Client::new(
        "FLWSECK-266126f0ebb8c833c2852fdc5a21eec5-194272cd837vt-X",
        "FLWPUBK-03d00084cd0fd4f65a56017d2423b9fc-X",
        "266126f0ebb836dbcff05110",
    );

    let response = client
        .create_virtual_acct_no(VirtualAcctCreationReq {
            amount: 0.into(),
            bvn: "".into(),
            email: "test@email.com".into(),
            is_permanent: None,
            narration: None,
            tx_ref: None,
        })
        .await;

    match response {
        Ok(res) => {
            println!("{:?}", res);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}
