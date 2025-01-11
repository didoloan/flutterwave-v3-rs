use std::fmt::Display;

#[derive(Debug, thiserror::Error)]
pub enum FWaveError {
    SerDe(serde_json::Error),
    Request(reqwest::Error),
    RequestValidation(validator::ValidationErrors),
}

impl From<reqwest::Error> for FWaveError {
    fn from(value: reqwest::Error) -> Self {
        FWaveError::Request(value)
    }
}

impl From<validator::ValidationErrors> for FWaveError {
    fn from(value: validator::ValidationErrors) -> Self {
        FWaveError::RequestValidation(value)
    }
}

impl From<serde_json::Error> for FWaveError {
    fn from(value: serde_json::Error) -> Self {
        FWaveError::SerDe(value)
    }
}

impl Display for FWaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FWaveError::SerDe(error) => f.write_str(&error.to_string()),
            FWaveError::Request(error) => f.write_str(&error.to_string()),
            FWaveError::RequestValidation(validation_errors) => {
                f.write_str(&validation_errors.to_string())
            }
        }
    }
}
