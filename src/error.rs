use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Reqwest Error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Other Response: {0}")]
    OtherResponse(reqwest::StatusCode),
    #[error("DecodeError: {0}, content: {1}")]
    DecodeError(serde_json::Error, String),
}
