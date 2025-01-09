/// Config is the common parameters for steam api requests.
#[derive(Debug, Default, Clone)]
pub struct Config {
    key: String,
    // TODO:  add optional parameter format & language
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
