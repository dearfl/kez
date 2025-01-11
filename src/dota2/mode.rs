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
    Greeviling = 9,
    Tutorial = 10,
    MidOnly = 11,
    LeastPlayed = 12,
    LimitedHeroes = 13,
    CompendiumMatchmaking = 14,
    Custom = 15,
    CaptainsDraft = 16,
    BalancedDraft = 17,
    AbilityDraft = 18,
    Event = 19,
    AllRandomDeathMatch = 20,
    SoloMid = 21,
    AllDraft = 22,
    Turbo = 23,
    Mutation = 24,
    CoachesChallenge = 25,
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
            9 => Self::Greeviling,
            10 => Self::Tutorial,
            11 => Self::MidOnly,
            12 => Self::LeastPlayed,
            13 => Self::LimitedHeroes,
            14 => Self::CompendiumMatchmaking,
            15 => Self::Custom,
            16 => Self::CaptainsDraft,
            17 => Self::BalancedDraft,
            18 => Self::AbilityDraft,
            19 => Self::Event,
            20 => Self::AllRandomDeathMatch,
            21 => Self::SoloMid,
            22 => Self::AllDraft,
            23 => Self::Turbo,
            24 => Self::Mutation,
            25 => Self::CoachesChallenge,
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
            Mode::Greeviling => 9,
            Mode::Tutorial => 10,
            Mode::MidOnly => 11,
            Mode::LeastPlayed => 12,
            Mode::LimitedHeroes => 13,
            Mode::CompendiumMatchmaking => 14,
            Mode::Custom => 15,
            Mode::CaptainsDraft => 16,
            Mode::BalancedDraft => 17,
            Mode::AbilityDraft => 18,
            Mode::Event => 19,
            Mode::AllRandomDeathMatch => 20,
            Mode::SoloMid => 21,
            Mode::AllDraft => 22,
            Mode::Turbo => 23,
            Mode::Mutation => 24,
            Mode::CoachesChallenge => 25,
        }
    }
}

impl Transform<Mode> for RequestBuilder {
    fn transform(self, value: Mode) -> Self {
        self.query(&[("game_mode", u8::from(value))])
    }
}
