use serde::{Deserialize, Serialize};

use crate::{
    dota2::{Hero, Mode, Skill},
    TransformRequest,
};

/// filter for matches with at least N human players.
#[derive(Copy, Clone, Debug, Default)]
pub struct MinPlayers(u8);

impl MinPlayers {
    pub fn new(cnt: u8) -> Self {
        Self(cnt)
    }
}

impl From<u8> for MinPlayers {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<MinPlayers> for u8 {
    fn from(value: MinPlayers) -> Self {
        value.0
    }
}

impl TransformRequest for MinPlayers {
    fn transform_request(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        req.query(&[("min_players", self.0)])
    }
}

/// These are parameters to API get_match_history
/// This type is fairly complicated so you're advised to use the builder pattern here.
/// The type itself is also a builder so you can do something like these.
/// # Example
/// ```rust,no_run
/// use kez::dota2::get_match_history::MatchHistoryParameter;
/// use kez::dota2::Hero;
/// let parameter = MatchHistoryParameter::new()
///                     .with_hero(Hero::Kez) // search for matches contains Kez
///                     .with_min_players(10); // at least 10 human players
/// ```
#[derive(Copy, Clone, Debug, Default)]
pub struct MatchHistoryParameter {
    pub hero: Option<Hero>,
    pub mode: Option<Mode>,
    pub skill: Option<Skill>,
    pub min_players: Option<MinPlayers>,
    pub account_id: Option<u64>,
    pub league_id: Option<u64>,
    pub start_at_match_id: Option<u64>,
    pub matches_requested: Option<u8>,
    pub tournament_games_only: Option<bool>,
}

impl TransformRequest for MatchHistoryParameter {
    fn transform_request(&self, mut req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        req = self.hero.transform_request(req);
        req = self.mode.transform_request(req);
        req = self.skill.transform_request(req);
        req = self.min_players.transform_request(req);
        if let Some(account_id) = self.account_id {
            req = req.query(&[("account_id", account_id)]);
        }
        if let Some(league_id) = self.league_id {
            req = req.query(&[("league_id", league_id)]);
        }
        if let Some(start_at_match_id) = self.start_at_match_id {
            req = req.query(&[("start_at_match_id", start_at_match_id)]);
        }
        if let Some(matches_requested) = self.matches_requested {
            req = req.query(&[("matches_requested", matches_requested)]);
        }
        if let Some(tournament_games_only) = self.tournament_games_only {
            req = req.query(&[("tournament_games_only", u8::from(tournament_games_only))]);
        }
        req
    }
}

impl MatchHistoryParameter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_hero(mut self, hero: Hero) -> Self {
        self.hero = Some(hero);
        self
    }

    pub fn with_game_mode(mut self, mode: Mode) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn with_skill(mut self, skill: Skill) -> Self {
        self.skill = Some(skill);
        self
    }

    pub fn with_min_players(mut self, min_players: u8) -> Self {
        self.min_players = Some(min_players.into());
        self
    }

    pub fn with_account_id(mut self, account_id: u64) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn with_league_id(mut self, league_id: u64) -> Self {
        self.league_id = Some(league_id);
        self
    }

    pub fn with_tournament_games_only(mut self, tournament_games_only: bool) -> Self {
        self.tournament_games_only = Some(tournament_games_only);
        self
    }

    pub fn with_start_at_match_id(mut self, start_at_match_id: u64) -> Self {
        self.start_at_match_id = Some(start_at_match_id);
        self
    }

    pub fn with_matches_requested(mut self, matches_requested: u8) -> Self {
        self.matches_requested = Some(matches_requested);
        self
    }
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
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Match {
    pub players: Vec<Player>,
    pub start_time: u64,
    pub match_id: u64,
    pub match_seq_num: u64,
    #[serde(default)]
    pub radiant_team_id: u64,
    #[serde(default)]
    pub dire_team_id: u64,
    pub lobby_type: u8,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct MatchHistory {
    pub status: u8,
    #[serde(rename = "statusDetail", default)]
    pub status_detail: String,
    pub num_results: u32,
    pub total_results: u32,
    pub results_remaining: u32,
    pub matches: Vec<Match>,
}
