use std::time::{Duration, SystemTime};

use reqwest::RequestBuilder;

use crate::{
    dota2::{Draft, Engine, League, Lobby, Mode, Player, Side},
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
    pub pre_game_duration: u16,
    pub start_time: SystemTime,
    pub match_id: MatchId,
    pub match_seq_num: MatchSeqNum,
    // pub tower_status_radiant: u32,
    // pub tower_status_dire: u32,
    // pub barracks_status_radiant: u32,
    // pub barracks_status_dire: u32,
    pub cluster: u32,
    pub first_blood_time: u16,
    pub lobby_type: Lobby,
    pub human_players: u8,
    pub league: League,
    pub mode: Mode,
    pub flags: u8,
    pub engine: Engine,
    pub radiant_score: u16,
    pub dire_score: u16,
    pub tournament_id: u64,
    pub tournament_round: u64,
    pub radiant_team_id: u64,
    pub radiant_name: String,
    pub radiant_logo: u64,
    pub radiant_team_complete: u64,
    pub dire_team_id: u64,
    pub dire_name: String,
    pub dire_logo: u64,
    pub dire_team_complete: u64,
    pub radiant_captain: u64,
    pub dire_captain: u64,
    pub drafts: Vec<Draft>,
}

impl From<crate::dota2::get_match_history_by_seq_num::Match> for Match {
    fn from(mat: crate::dota2::get_match_history_by_seq_num::Match) -> Self {
        let winner = match mat.radiant_win {
            true => Side::Radiant,
            false => Side::Dire,
        };
        Self {
            players: mat.players.into_iter().map(Into::into).collect(),
            winner,
            start_time: SystemTime::UNIX_EPOCH + Duration::from_secs(mat.start_time),
            duration: Duration::from_secs(mat.duration.into()),
            match_id: mat.match_id.into(),
            match_seq_num: mat.match_seq_num.into(),
            lobby_type: mat.lobby_type.into(),
            league: mat.leagueid.into(),
            mode: mat.game_mode.into(),
            radiant_score: mat.radiant_score,
            dire_score: mat.dire_score,
            human_players: mat.human_players,
            drafts: mat.picks_bans.into_iter().map(Into::into).collect(),
            engine: mat.engine.into(),
            pre_game_duration: mat.pre_game_duration.into(),
            cluster: mat.cluster.into(),
            first_blood_time: mat.first_blood_time.into(),
            flags: mat.flags.into(),
            tournament_id: mat.tournament_id.into(),
            tournament_round: mat.tournament_round.into(),
            radiant_team_id: mat.radiant_team_id.into(),
            radiant_name: mat.radiant_name.into(),
            radiant_logo: mat.radiant_logo.into(),
            radiant_team_complete: mat.radiant_team_complete.into(),
            dire_team_id: mat.dire_team_id.into(),
            dire_name: mat.dire_name.into(),
            dire_logo: mat.dire_logo.into(),
            dire_team_complete: mat.dire_team_complete.into(),
            radiant_captain: mat.radiant_captain.into(),
            dire_captain: mat.dire_captain.into(),
        }
    }
}
