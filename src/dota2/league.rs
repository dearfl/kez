use reqwest::RequestBuilder;

use crate::Transform;

/// League repr
#[non_exhaustive]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum League {
    Unknown,
    League(u32),
}

impl Default for League {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<u32> for League {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Unknown,
            id => Self::League(id),
        }
    }
}

impl From<League> for u32 {
    fn from(value: League) -> Self {
        match value {
            League::Unknown => 0,
            League::League(id) => id,
        }
    }
}

impl Transform<League> for RequestBuilder {
    fn transform(self, value: League) -> Self {
        self.query(&[("league_id", u32::from(value))])
    }
}
