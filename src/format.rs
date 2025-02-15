use reqwest::RequestBuilder;

use crate::Transform;

/// The format specified for the response, available options from server:
///  - json
///  - xml
///  - vdf
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default)]
pub enum Format {
    /// Currently we only support json.
    #[default]
    Json,
}

impl Transform<Format> for RequestBuilder {
    fn transform(self, value: Format) -> Self {
        let code = match value {
            Format::Json => "json",
        };
        self.query(&[("format", code)])
    }
}
