use reqwest::RequestBuilder;

use crate::{Transform, dota2::define_dota2_enum};

define_dota2_enum! {
    /// Game Mode enum
    pub enum Mode: u8 {
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
}

impl Transform<Mode> for RequestBuilder {
    fn transform(self, value: Mode) -> Self {
        self.query(&[("game_mode", u8::from(value))])
    }
}
