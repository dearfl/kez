mod client;
mod config;
pub mod dota2;
mod error;
mod language;
mod response;

pub use client::Client;
pub use config::Config;
pub use error::Error;
pub use language::Language;
use response::Response;

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Transform is trait for literally transform request like add parameters.
/// Implement this trait for Config and varies Parameters make the inner get method
/// Generic.
trait Transform<T> {
    fn transform(self, value: T) -> Self;
}

/// generic impl Transform<Option<T>> for ease of use
impl<T, S> Transform<Option<T>> for S
where
    S: Transform<T>,
{
    fn transform(self, value: Option<T>) -> Self {
        match value {
            Some(value) => self.transform(value),
            None => self,
        }
    }
}

/// generic impl Transform<(T1, T2)> for ease of use
impl<T1, T2, S> Transform<(T1, T2)> for S
where
    S: Transform<T1> + Transform<T2>,
{
    fn transform(self, (v1, v2): (T1, T2)) -> Self {
        self.transform(v1).transform(v2)
    }
}
