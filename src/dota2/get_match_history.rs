use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use crate::{
    dota2::{Account, HeroId, League, MatchId, MatchesRequested, Mode, Skill, StartAt},
    Transform,
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

impl Transform<MinPlayers> for RequestBuilder {
    fn transform(self, value: MinPlayers) -> Self {
        self.query(&[("min_players", value.0)])
    }
}

/// filter for tournament games.
#[derive(Copy, Clone, Debug, Default)]
pub struct TournamentGamesOnly(bool);

impl TournamentGamesOnly {
    pub fn new(cnt: bool) -> Self {
        Self(cnt)
    }
}

impl From<bool> for TournamentGamesOnly {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<TournamentGamesOnly> for bool {
    fn from(value: TournamentGamesOnly) -> Self {
        value.0
    }
}

impl Transform<TournamentGamesOnly> for RequestBuilder {
    fn transform(self, value: TournamentGamesOnly) -> Self {
        self.query(&[("tournament_games_only", u8::from(value.0))])
    }
}

/// These are parameters to API get_match_history
/// This type is fairly complicated so you're advised to use the builder pattern here.
/// The type itself is also a builder so you can do something like these.
/// # Example
/// ```rust,no_run
/// use kez::dota2::get_match_history::MatchHistoryParameter;
/// use kez::dota2::HeroId;
/// let parameter = MatchHistoryParameter::new()
///                     .with_hero(HeroId::Kez) // search for matches contains Kez
///                     .with_min_players(10); // at least 10 human players
/// ```
#[derive(Copy, Clone, Debug, Default)]
pub struct MatchHistoryParameter {
    pub hero: Option<HeroId>,
    pub mode: Option<Mode>,
    pub skill: Option<Skill>,
    pub min_players: Option<MinPlayers>,
    pub account: Option<Account>,
    pub league: Option<League>,
    pub start_at_match_id: Option<StartAt<MatchId>>,
    pub matches_requested: Option<MatchesRequested>,
    pub tournament_games_only: Option<TournamentGamesOnly>,
}

impl Transform<MatchHistoryParameter> for RequestBuilder {
    fn transform(self, value: MatchHistoryParameter) -> Self {
        self.transform(value.hero)
            .transform(value.mode)
            .transform(value.skill)
            .transform(value.min_players)
            .transform(value.account)
            .transform(value.league)
            .transform(value.start_at_match_id)
            .transform(value.matches_requested)
            .transform(value.tournament_games_only)
    }
}

impl MatchHistoryParameter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_hero(mut self, hero: HeroId) -> Self {
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

    pub fn with_account(mut self, account: Account) -> Self {
        self.account = Some(account);
        self
    }

    pub fn with_league(mut self, league: League) -> Self {
        self.league = Some(league);
        self
    }

    pub fn with_tournament_games_only(mut self, tournament_games_only: bool) -> Self {
        self.tournament_games_only = Some(tournament_games_only.into());
        self
    }

    pub fn with_start_at_match_id(mut self, start_at_match_id: MatchId) -> Self {
        self.start_at_match_id = Some(start_at_match_id.into());
        self
    }

    pub fn with_matches_requested(mut self, matches_requested: u8) -> Self {
        self.matches_requested = Some(matches_requested.into());
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
