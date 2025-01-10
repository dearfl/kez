use crate::TransformRequest;

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

impl TransformRequest for League {
    fn transform_request(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        req.query(&[("league_id", u64::from(*self))])
    }
}
