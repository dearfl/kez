/// Skill level of player or match.
#[non_exhaustive]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default)]
pub enum Skill {
    #[default]
    Any = 0,
    Normal = 1,
    High = 2,
    VeryHigh = 3,
}

impl From<u8> for Skill {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Normal,
            2 => Self::High,
            3 => Self::VeryHigh,
            _ => Self::Any,
        }
    }
}

impl From<Skill> for u8 {
    fn from(value: Skill) -> Self {
        value as Self
    }
}
