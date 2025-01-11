use std::time::Duration;

use reqwest::RequestBuilder;

use crate::{
    dota2::{League, Lobby, Mode, Player, Side},
    Transform,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct MatchesRequested(u8);

impl From<u8> for MatchesRequested {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<MatchesRequested> for u8 {
    fn from(value: MatchesRequested) -> Self {
        value.0
    }
}

impl Transform<MatchesRequested> for RequestBuilder {
    fn transform(self, value: MatchesRequested) -> Self {
        self.query(&[("matches_requested", value.0)])
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MatchId(u64);

impl From<u64> for MatchId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<MatchId> for u64 {
    fn from(value: MatchId) -> Self {
        value.0
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MatchSeqNum(u64);

impl From<u64> for MatchSeqNum {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<MatchSeqNum> for u64 {
    fn from(value: MatchSeqNum) -> Self {
        value.0
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StartAt<T>(T);

impl<T> From<T> for StartAt<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl Transform<StartAt<MatchId>> for RequestBuilder {
    fn transform(self, value: StartAt<MatchId>) -> Self {
        self.query(&[("start_at_match_id", u64::from(value.0))])
    }
}

impl Transform<StartAt<MatchSeqNum>> for RequestBuilder {
    fn transform(self, value: StartAt<MatchSeqNum>) -> Self {
        self.query(&[("start_at_match_seq_num", u64::from(value.0))])
    }
}

#[derive(Clone, Debug)]
pub struct Match {
    pub players: Vec<Player>,
    pub winner: Side,
    pub duration: Duration,
    // pub pre_game_duration: u16,
    // pub start_time: u64,
    pub match_id: MatchId,
    pub match_seq_num: MatchSeqNum,
    // pub tower_status_radiant: u32,
    // pub tower_status_dire: u32,
    // pub barracks_status_radiant: u32,
    // pub barracks_status_dire: u32,
    // pub cluster: u32,
    // pub first_blood_time: u16,
    pub lobby_type: Lobby,
    // pub human_players: u8,
    pub league: League,
    pub mode: Mode,
    // pub flags: u8,
    // pub engine: u8,
    pub radiant_score: u16,
    pub dire_score: u16,
    // pub tournament_id: u64,
    // pub tournament_round: u64,
    // pub radiant_team_id: u64,
    // pub radiant_name: String,
    // pub radiant_logo: u64,
    // pub radiant_team_complete: u64,
    // pub dire_team_id: u64,
    // pub dire_name: String,
    // pub dire_logo: u64,
    // pub dire_team_complete: u64,
    // pub radiant_captain: u64,
    // pub dire_captain: u64,
    // pub picks_bans: Vec<HeroSelection>,
}

impl From<crate::dota2::get_match_history_by_seq_num::Match> for Match {
    fn from(value: crate::dota2::get_match_history_by_seq_num::Match) -> Self {
        let winner = match value.radiant_win {
            true => Side::Radiant,
            false => Side::Dire,
        };
        Self {
            players: value.players.into_iter().map(Into::into).collect(),
            winner,
            duration: Duration::from_secs(value.duration.into()),
            match_id: value.match_id.into(),
            match_seq_num: value.match_seq_num.into(),
            lobby_type: value.lobby_type.into(),
            league: value.leagueid.into(),
            mode: value.game_mode.into(),
            radiant_score: value.radiant_score,
            dire_score: value.dire_score,
        }
    }
}
