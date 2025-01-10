use serde::{Deserialize, Serialize};

use crate::TransformRequest;

#[derive(Copy, Clone, Debug, Default)]
pub struct GetHeroesParameter {
    /// I have no idea what this is, and we should have another parameter language
    /// but I've decided to add language as a common parameter for all APIs.
    pub itemizedonly: Option<bool>,
}

impl From<bool> for GetHeroesParameter {
    fn from(value: bool) -> Self {
        Self {
            itemizedonly: Some(value),
        }
    }
}

impl From<()> for GetHeroesParameter {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl TransformRequest for GetHeroesParameter {
    fn transform_request(&self, mut req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(itemizedonly) = self.itemizedonly {
            req = req.query(&[("itemizedonly", u8::from(itemizedonly))]);
        }
        req
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Hero {
    pub name: String,
    pub id: u8,
    /// Only when you specified language when requesting
    pub localized_name: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Heroes {
    pub status: u8,
    pub count: u8,
    pub matches: Vec<Hero>,
}
