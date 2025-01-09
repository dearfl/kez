use serde::{Deserialize, Serialize};

/// Response is a thin wrapper around valve's web api return type.
/// Basically anything we care about returned from valve's api is
/// wrapped inside this struct.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Response<T> {
    pub result: T,
}
