use crate::dota2::{AbilityUpgrade, Account, Hero, Item, LeaveStatus, Side, Unit};

/// Converted Player type
#[derive(Clone, Debug)]
pub struct Player {
    pub account: Account,
    /// side and position of this player
    pub slot: (Side, u8),
    pub team_number: u8,
    pub team_slot: u8,
    /// hero and variant(facet?) of hero
    pub hero: (Hero, u8),
    pub item_0: Option<Item>,
    pub item_1: Option<Item>,
    pub item_2: Option<Item>,
    pub item_3: Option<Item>,
    pub item_4: Option<Item>,
    pub item_5: Option<Item>,
    pub backpack_0: Option<Item>,
    pub backpack_1: Option<Item>,
    pub backpack_2: Option<Item>,
    pub item_neutral: Option<Item>,
    pub kills: u8,
    pub deaths: u8,
    pub assists: u8,
    pub leave_status: Option<LeaveStatus>,
    pub last_hits: u16,
    pub denies: u16,
    pub level: u8,
    pub net_worth: u32,
    pub aghanims_scepter: bool,
    pub aghanims_shard: bool,
    pub moonshard: bool,
    pub hero_damage: u32,
    pub tower_damage: u32,
    pub hero_healing: u32,
    pub gold_per_min: u16,
    pub xp_per_min: u16,
    pub gold: u32,
    pub gold_spent: u32,
    pub scaled_hero_damage: u32,
    pub scaled_tower_damage: u32,
    pub scaled_hero_healing: u32,
    pub ability_upgrades: Vec<AbilityUpgrade>,
    pub additional_units: Vec<Unit>,
}

impl From<crate::dota2::get_match_history_by_seq_num::Player> for Player {
    fn from(player: crate::dota2::get_match_history_by_seq_num::Player) -> Self {
        Self {
            account: player.account_id.into(),
            hero: (player.hero_id.into(), player.hero_variant),
            slot: (player.player_slot.into(), player.player_slot & 0x7F),
            item_0: Item::from(player.item_0).into_option(),
            item_1: Item::from(player.item_1).into_option(),
            item_2: Item::from(player.item_2).into_option(),
            item_3: Item::from(player.item_3).into_option(),
            item_4: Item::from(player.item_4).into_option(),
            item_5: Item::from(player.item_5).into_option(),
            backpack_0: Item::from(player.backpack_0).into_option(),
            backpack_1: Item::from(player.backpack_1).into_option(),
            backpack_2: Item::from(player.backpack_2).into_option(),
            item_neutral: Item::from(player.item_neutral).into_option(),
            kills: player.kills,
            deaths: player.deaths,
            assists: player.assists,
            last_hits: player.last_hits,
            denies: player.denies,
            level: player.level,
            net_worth: player.net_worth,
            hero_damage: player.hero_damage,
            tower_damage: player.tower_damage,
            hero_healing: player.hero_healing,
            gold: player.gold,
            gold_spent: player.gold_spent,
            scaled_hero_damage: player.scaled_hero_damage,
            scaled_tower_damage: player.scaled_tower_damage,
            scaled_hero_healing: player.scaled_hero_healing,
            team_number: player.team_number,
            team_slot: player.team_slot,
            aghanims_scepter: player.aghanims_scepter != 0,
            aghanims_shard: player.aghanims_shard != 0,
            moonshard: player.moonshard != 0,
            gold_per_min: player.gold_per_min,
            xp_per_min: player.xp_per_min,
            ability_upgrades: player
                .ability_upgrades
                .into_iter()
                .map(Into::into)
                .collect(),
            additional_units: player
                .additional_units
                .into_iter()
                .map(Into::into)
                .collect(),
            leave_status: LeaveStatus::from(player.leaver_status).into_option(),
        }
    }
}
