mod client;
mod config;
mod error;

pub use client::Client;
pub use config::Config;
pub use error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;
