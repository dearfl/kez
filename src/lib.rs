mod client;
mod config;
pub mod dota2;
mod error;
mod response;

pub use client::Client;
pub use config::Config;
pub use error::Error;
use response::Response;

use reqwest::RequestBuilder;

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// TransformRequest is trait for literally transform request like add parameters.
/// Implement this trait for Config and varies Parameters make the inner get method
/// Generic.
trait TransformRequest {
    fn transform_request(&self, req: RequestBuilder) -> RequestBuilder;
}

/// impl TransformRequest for Option<T> where T: Transform for ease of use
impl<T: TransformRequest> TransformRequest for Option<T> {
    fn transform_request(&self, req: RequestBuilder) -> RequestBuilder {
        match self {
            Some(value) => value.transform_request(req),
            None => req,
        }
    }
}
