use reqwest::RequestBuilder;

use crate::{Format, Language, Transform};

/// A simple wrapper type for API KEY.
#[derive(Debug, Clone)]
pub struct Key(String);

impl<S> From<S> for Key
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        Self(value.into())
    }
}

impl Transform<&Key> for RequestBuilder {
    fn transform(self, value: &Key) -> Self {
        self.query(&[("key", &value.0)])
    }
}

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
    pub(crate) key: Key,
    pub(crate) language: Option<Language>,
    pub(crate) format: Option<Format>,
}

impl Config {
    pub fn new(key: impl Into<String>) -> Self {
        let key = key.into().into();
        Self {
            key,
            language: None,
            format: None,
        }
    }

    pub fn with_language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    pub fn with_format(mut self, format: Format) -> Self {
        self.format = Some(format);
        self
    }
}

impl<S> From<S> for Config
where
    S: Into<Key>,
{
    fn from(value: S) -> Self {
        let key = value.into();
        let language = None;
        let format = None;
        Self {
            key,
            language,
            format,
        }
    }
}

impl Transform<&Config> for RequestBuilder {
    fn transform(self, value: &Config) -> Self {
        self.transform(&value.key)
            .transform(value.language)
            .transform(value.format)
    }
}
