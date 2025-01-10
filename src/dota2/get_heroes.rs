use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use crate::Transform;

#[derive(Copy, Clone, Debug, Default)]
pub struct GetHeroesParameter {
    pub itemizedonly: Option<bool>,
}

impl From<bool> for GetHeroesParameter {
    fn from(value: bool) -> Self {
        let itemizedonly = Some(value.into());
        Self { itemizedonly }
    }
}

impl From<()> for GetHeroesParameter {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl Transform<GetHeroesParameter> for RequestBuilder {
    fn transform(self, value: GetHeroesParameter) -> Self {
        match value.itemizedonly {
            Some(value) => self.query(&[("itemizedonly", u8::from(value))]),
            None => self,
        }
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
    pub heroes: Vec<Hero>,
}
