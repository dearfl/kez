/// Config is the common parameters for steam api requests.
/// Most of steam http api have 3 common parameters:
///   - key: String, the API KEY used for requesting these APIs.
///   - format: json/xml/vdf, format specified for response, default value: json.
///   - language: language specified for response, by default all response is in english.
/// # Example
/// ```
/// use kez::Config;
/// let config = Config::from("MY_STEAM_API_KEY");
/// ```
#[derive(Debug, Clone)]
pub struct Config {
    pub(crate) key: String,
    // TODO:  add optional parameter: format & language
}

impl<S> From<S> for Config
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        let key = value.into();
        Self { key }
    }
}
