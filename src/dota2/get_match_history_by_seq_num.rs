use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use crate::{
    dota2::{MatchSeqNum, MatchesRequested, StartAt},
    Transform,
};

/// These are parameters to API get_match_history_by_seq_num
/// The first parameter specifies which match the result should start with
/// The second parameter specifies how many matches the result should contains.
/// # Examples
/// ```rust,no_run
/// use kez::dota2::{get_match_history_by_seq_num::MatchHistoryBySeqNumParameter, MatchSeqNum};
/// // 10 matches starting from match sequence number 10000
/// let para: MatchHistoryBySeqNumParameter = (MatchSeqNum::from(10000), 10).into();
/// // 20 matches starting from match sequence number 20000
/// let para = MatchHistoryBySeqNumParameter::new(MatchSeqNum::from(20000), 20);
/// // 100 matches starting from match sequence number 0
/// let para = MatchHistoryBySeqNumParameter::default();
/// // 100 matches starting from match sequence number 30000
/// let para = MatchHistoryBySeqNumParameter::start_at(MatchSeqNum::from(30000));
/// ```
#[derive(Copy, Clone, Debug, Default)]
pub struct MatchHistoryBySeqNumParameter {
    pub start_at_match_seq_num: Option<StartAt<MatchSeqNum>>,
    pub matches_requested: Option<MatchesRequested>,
}

impl MatchHistoryBySeqNumParameter {
    pub fn start_at(match_seq_num: MatchSeqNum) -> Self {
        let start_at_match_seq_num = Some(StartAt::from(match_seq_num));
        let matches_requested = None;
        Self {
            start_at_match_seq_num,
            matches_requested,
        }
    }

    pub fn new(start_at_match_seq_num: MatchSeqNum, matches_requested: u8) -> Self {
        let start_at_match_seq_num = Some(StartAt::from(start_at_match_seq_num));
        let matches_requested = Some(MatchesRequested::from(matches_requested));
        Self {
            start_at_match_seq_num,
            matches_requested,
        }
    }
}

impl From<(MatchSeqNum, u8)> for MatchHistoryBySeqNumParameter {
    fn from((start_at_match_seq_num, matches_requested): (MatchSeqNum, u8)) -> Self {
        let start_at_match_seq_num = Some(StartAt::from(start_at_match_seq_num));
        let matches_requested = Some(MatchesRequested::from(matches_requested));
        Self {
            start_at_match_seq_num,
            matches_requested,
        }
    }
}

impl From<MatchSeqNum> for MatchHistoryBySeqNumParameter {
    fn from(start_at_match_seq_num: MatchSeqNum) -> Self {
        let start_at_match_seq_num = Some(StartAt::from(start_at_match_seq_num));
        let matches_requested = None;
        Self {
            start_at_match_seq_num,
            matches_requested,
        }
    }
}

impl From<()> for MatchHistoryBySeqNumParameter {
    fn from(_: ()) -> Self {
        let start_at_match_seq_num = None;
        let matches_requested = None;
        Self {
            start_at_match_seq_num,
            matches_requested,
        }
    }
}

impl Transform<MatchHistoryBySeqNumParameter> for RequestBuilder {
    fn transform(self, value: MatchHistoryBySeqNumParameter) -> Self {
        self.transform(value.start_at_match_seq_num)
            .transform(value.matches_requested)
    }
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AbilityUpgrade {
    pub ability: u16,
    pub time: u16,
    pub level: u16,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Unit {
    pub unitname: String,
    pub item_0: u16,
    pub item_1: u16,
    pub item_2: u16,
    pub item_3: u16,
    pub item_4: u16,
    pub item_5: u16,
    pub backpack_0: u16,
    pub backpack_1: u16,
    pub backpack_2: u16,
    pub item_neutral: u16,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Player {
    #[serde(default)]
    pub account_id: u32,
    pub player_slot: u8,
    pub team_number: u8,
    pub team_slot: u8,
    pub hero_id: u8,
    pub hero_variant: u8,
    pub item_0: u16,
    pub item_1: u16,
    pub item_2: u16,
    pub item_3: u16,
    pub item_4: u16,
    pub item_5: u16,
    pub backpack_0: u16,
    pub backpack_1: u16,
    pub backpack_2: u16,
    pub item_neutral: u16,
    pub kills: u8,
    pub deaths: u8,
    pub assists: u8,
    #[serde(default)]
    pub leaver_status: u8,
    pub last_hits: u16,
    pub denies: u16,
    pub gold_per_min: u16,
    pub xp_per_min: u16,
    pub level: u8,
    pub net_worth: u32,
    #[serde(default)]
    pub aghanims_scepter: u8,
    #[serde(default)]
    pub aghanims_shard: u8,
    #[serde(default)]
    pub moonshard: u8,
    #[serde(default)]
    pub hero_damage: u32,
    #[serde(default)]
    pub tower_damage: u32,
    #[serde(default)]
    pub hero_healing: u32,
    #[serde(default)]
    pub gold: u32,
    #[serde(default)]
    pub gold_spent: u32,
    #[serde(default)]
    pub scaled_hero_damage: u32,
    #[serde(default)]
    pub scaled_tower_damage: u32,
    #[serde(default)]
    pub scaled_hero_healing: u32,
    #[serde(default)]
    pub ability_upgrades: Vec<AbilityUpgrade>,
    #[serde(default)]
    pub additional_units: Vec<Unit>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Draft {
    pub is_pick: bool,
    pub hero_id: u8,
    pub team: u8,
    pub order: u8,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Match {
    pub players: Vec<Player>,
    pub radiant_win: bool,
    pub duration: u16,
    pub pre_game_duration: u16,
    pub start_time: u64,
    pub match_id: u64,
    pub match_seq_num: u64,
    pub tower_status_radiant: u32,
    pub tower_status_dire: u32,
    pub barracks_status_radiant: u32,
    pub barracks_status_dire: u32,
    pub cluster: u32,
    pub first_blood_time: u16,
    pub lobby_type: u8,
    pub human_players: u8,
    pub leagueid: u32,
    pub game_mode: u8,
    pub flags: u8,
    pub engine: u8,
    pub radiant_score: u16,
    pub dire_score: u16,
    #[serde(default)]
    pub tournament_id: u64,
    #[serde(default)]
    pub tournament_round: u64,
    #[serde(default)]
    pub radiant_team_id: u64,
    #[serde(default)]
    pub radiant_name: String,
    #[serde(default)]
    pub radiant_logo: u64,
    #[serde(default)]
    pub radiant_team_complete: u64,
    #[serde(default)]
    pub dire_team_id: u64,
    #[serde(default)]
    pub dire_name: String,
    #[serde(default)]
    pub dire_logo: u64,
    #[serde(default)]
    pub dire_team_complete: u64,
    #[serde(default)]
    pub radiant_captain: u64,
    #[serde(default)]
    pub dire_captain: u64,
    #[serde(default)]
    pub picks_bans: Vec<Draft>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct MatchHistoryBySeqNum {
    pub status: u8,
    #[serde(rename = "statusDetail", default)]
    pub status_detail: String,
    #[serde(default)]
    pub matches: Vec<Match>,
}
