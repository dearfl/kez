use reqwest::{IntoUrl, StatusCode};
use serde::Deserialize;

use crate::{
    dota2::{
        get_match_history::{MatchHistory, MatchHistoryParameter},
        get_match_history_by_seq_num::{MatchHistoryBySeqNum, MatchHistoryBySeqNumParameter},
    },
    Config, Error, Response, Result, TransformRequest,
};

/// Client is what used to request an API.
#[derive(Debug, Clone)]
pub struct Client {
    config: Config,
    client: reqwest::Client,
}

impl Client {
    /// Create a new client from config.
    /// # Example:
    /// ```rust,no_run
    /// use kez::Client;
    /// let client: Client = Client::new("MY_STEAM_API_KEY").expect("Failed to construct client");
    /// ```
    pub fn new<C: Into<Config>>(config: C) -> Result<Self> {
        let config = config.into();
        let client = reqwest::ClientBuilder::new().build()?;
        Ok(Self { config, client })
    }

    /// Create a new client from existing client and config in case you want to reuse client.
    /// # Example:
    /// ```rust,no_run
    /// use kez::Client;
    /// let client = reqwest::Client::new();
    /// let client: Client = Client::with_client(client, "MY_STEAM_API_KEY");
    /// ```
    pub fn with_client<C: Into<Config>>(client: reqwest::Client, config: C) -> Self {
        let config = config.into();
        Self { config, client }
    }

    /// This function is a general function for varies Valve HTTP API.
    pub(crate) async fn get<T>(&self, url: impl IntoUrl, para: impl TransformRequest) -> Result<T>
    where
        T: for<'a> Deserialize<'a>,
    {
        let req = self.client.get(url);
        let req = self.config.transform_request(req);
        let req = para.transform_request(req);

        let resp = req.send().await.map_err(reqwest::Error::without_url)?;
        let status = resp.status();

        if !matches!(status, StatusCode::OK) {
            return Err(Error::OtherResponse(resp.status()));
        }

        let content = resp.text().await.map_err(reqwest::Error::without_url)?;
        serde_json::from_str(&content)
            .map(|result: Response<T>| result.result)
            .map_err(|err| Error::DecodeError(err, content))
    }

    /// Request a sequence of matches by a start match sequence number and count.
    /// # Example:
    /// ```rust,no_run
    /// use kez::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     
    ///   let client: Client = Client::new("MY_STEAM_API_KEY").expect("Failed to create client");
    ///   // request 100 matches starting from match sequence number 0
    ///   let result = client.get_match_history_by_seq_num((0, 100)).await?;
    ///   println!("{:?}", result);
    ///   Ok(())
    /// }
    /// ```
    pub async fn get_match_history_by_seq_num<P>(&self, para: P) -> Result<MatchHistoryBySeqNum>
    where
        P: Into<MatchHistoryBySeqNumParameter>,
    {
        const URL: &str =
            "https://api.steampowered.com/IDOTA2Match_570/GetMatchHistoryBySequenceNum/v1";
        self.get(URL, para.into()).await
    }

    /// Request some match history by applying some filters.
    /// # Example:
    /// ```rust,no_run
    /// use kez::Client;
    /// use kez::dota2::get_match_history::MatchHistoryParameter;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///   let client: Client = Client::new("MY_STEAM_API_KEY").expect("Failed to create client");
    ///   // request 100 matches contains hero Kez starting from match id 0
    ///   let filter = MatchHistoryParameter::new().with_hero_id(145).with_matches_requested(100);
    ///   let result = client.get_match_history(filter).await?;
    ///   println!("{:?}", result);
    ///   Ok(())
    /// }
    /// ```
    pub async fn get_match_history<P>(&self, para: P) -> Result<MatchHistory>
    where
        P: Into<MatchHistoryParameter>,
    {
        const URL: &str = "https://api.steampowered.com/IDOTA2Match_570/GetMatchHistory/v1";
        self.get(URL, para.into()).await
    }
}
