use reqwest::RequestBuilder;

use crate::Transform;

/// Game Mode enum
#[non_exhaustive]
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Unknown(u8) = 0,
    AllPick = 1,
    CaptainsMode = 2,
    RandomDraft = 3,
    SingleDraft = 4,
    AllRandom = 5,
    Intro = 6,
    Diretide = 7,
    ReverseCaptainsMode = 8,
    TheGreeviling = 9,
    Tutorial = 10,
    MidOnly = 11,
    LeastPlayed = 12,
    NewPlayerPool = 13,
    CompendiumMatchmaking = 14,
    CaptainsDraft = 16,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Unknown(0)
    }
}

impl From<u8> for Mode {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::AllPick,
            2 => Self::CaptainsMode,
            3 => Self::RandomDraft,
            4 => Self::SingleDraft,
            5 => Self::AllRandom,
            6 => Self::Intro,
            7 => Self::Diretide,
            8 => Self::ReverseCaptainsMode,
            9 => Self::TheGreeviling,
            10 => Self::Tutorial,
            11 => Self::MidOnly,
            12 => Self::LeastPlayed,
            13 => Self::NewPlayerPool,
            14 => Self::CompendiumMatchmaking,
            16 => Self::CaptainsDraft,
            value => Self::Unknown(value),
        }
    }
}

impl From<Mode> for u8 {
    fn from(value: Mode) -> Self {
        match value {
            Mode::Unknown(value) => value,
            Mode::AllPick => 1,
            Mode::CaptainsMode => 2,
            Mode::RandomDraft => 3,
            Mode::SingleDraft => 4,
            Mode::AllRandom => 5,
            Mode::Intro => 6,
            Mode::Diretide => 7,
            Mode::ReverseCaptainsMode => 8,
            Mode::TheGreeviling => 9,
            Mode::Tutorial => 10,
            Mode::MidOnly => 11,
            Mode::LeastPlayed => 12,
            Mode::NewPlayerPool => 13,
            Mode::CompendiumMatchmaking => 14,
            Mode::CaptainsDraft => 16,
        }
    }
}

impl Transform<Mode> for RequestBuilder {
    fn transform(self, value: Mode) -> Self {
        self.query(&[("game_mode", u8::from(value))])
    }
}
