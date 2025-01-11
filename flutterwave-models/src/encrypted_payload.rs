use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct EncryptedPayload {
    pub client: String,
}

impl EncryptedPayload {
    pub fn new(b64_string: String) -> Self {
        Self { client: b64_string }
    }
}
