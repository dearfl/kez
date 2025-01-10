use reqwest::RequestBuilder;

use crate::Transform;

/// The ISO639-1 language code for the language all tokenized strings should be returned in.
/// Not all strings have been translated to every language. If a language does not have a string,
/// the English string will be returned instead. If this parameter is omitted the string token
/// will be returned for the strings.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default)]
pub enum Language {
    #[default]
    English,
    Chinese,
    Korean,
    // TODO: add other language?
    // While there are a lot of other languages, these are the only ones have a proper translation
    // I think.
}

impl Transform<Language> for RequestBuilder {
    fn transform(self, value: Language) -> Self {
        let code = match value {
            Language::English => "en",
            Language::Chinese => "zh",
            Language::Korean => "ko",
        };
        self.query(&[("language", code)])
    }
}
