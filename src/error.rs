use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Reqwest Error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Too Many Requests")]
    TooManyRequests,
    #[error("Other Response: {0}")]
    OtherResponse(reqwest::StatusCode),
    // TODO: DecodeError
}
