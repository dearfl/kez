use crate::Config;

/// Client is what used to request an API.
#[derive(Debug, Clone)]
pub struct Client {
    config: Config,
    client: reqwest::Client,
}

impl Client {
    /// Create a new client from config.
    /// # Example:
    /// ```
    /// use kez::Client;
    /// let client = Client::new("MY_STEAM_API_KEY");
    /// ```
    pub fn new<C: Into<Config>>(config: C) -> Self {
        let config = config.into();
        let client = reqwest::Client::new();
        Self { config, client }
    }

    /// Create a new client from existing client and config in case you want to reuse client.
    /// # Example:
    /// ```
    /// use kez::Client;
    /// let client = reqwest::Client::new();
    /// let client = Client::with_client(client, "MY_STEAM_API_KEY");
    /// ```
    pub fn with_client<C: Into<Config>>(client: reqwest::Client, config: C) -> Self {
        let config = config.into();
        Self { config, client }
    }
}
