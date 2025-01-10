use reqwest::RequestBuilder;

use crate::Transform;

/// Skill level of player or match.
#[non_exhaustive]
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Skill {
    Unknown(u8) = 0,
    Normal = 1,
    High = 2,
    VeryHigh = 3,
}

impl Default for Skill {
    fn default() -> Self {
        Self::Unknown(0)
    }
}

impl From<u8> for Skill {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Normal,
            2 => Self::High,
            3 => Self::VeryHigh,
            value => Self::Unknown(value),
        }
    }
}

impl From<Skill> for u8 {
    fn from(value: Skill) -> Self {
        match value {
            Skill::Unknown(value) => value,
            Skill::Normal => 1,
            Skill::High => 2,
            Skill::VeryHigh => 3,
        }
    }
}

impl Transform<Skill> for RequestBuilder {
    fn transform(self, value: Skill) -> Self {
        self.query(&[("skill", u8::from(value))])
    }
}
