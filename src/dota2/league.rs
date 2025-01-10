use reqwest::RequestBuilder;

use crate::Transform;

/// League repr
#[non_exhaustive]
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum League {
    Unknown,
    League(u64),
}

impl Default for League {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<u64> for League {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Unknown,
            id => Self::League(id),
        }
    }
}

impl From<League> for u64 {
    fn from(value: League) -> Self {
        match value {
            League::Unknown => 0,
            League::League(id) => id,
        }
    }
}

impl Transform<League> for RequestBuilder {
    fn transform(self, value: League) -> Self {
        self.query(&[("league_id", u64::from(value))])
    }
}
