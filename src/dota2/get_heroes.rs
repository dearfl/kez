use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use crate::Transform;

/// I have no idea what this is, and we should have another parameter language
/// but I've decided to add language as a common parameter for all APIs.
#[derive(Copy, Clone, Debug, Default)]
pub struct ItemizedOnly(bool);

#[derive(Copy, Clone, Debug, Default)]
pub struct GetHeroesParameter {
    pub itemizedonly: Option<ItemizedOnly>,
}

impl From<bool> for ItemizedOnly {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl Transform<ItemizedOnly> for RequestBuilder {
    fn transform(self, value: ItemizedOnly) -> Self {
        self.query(&[("itemizedonly", u8::from(value.0))])
    }
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
        self.transform(value.itemizedonly)
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
