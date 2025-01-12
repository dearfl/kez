use crate::dota2::define_dota2_enum;

define_dota2_enum! {
    /// represent whether player gives up match/AFK/etc...
    pub enum LeaveStatus : u8 {
        /// Disconnected, no abandon
        Disconnected = 1,
        /// Disconnected for more than 5 minutes, abandoned
        DisconnectedTooLong = 2,
        /// Disconnected and click leave
        Abandoned = 3,
        /// Player have not gain xp for more than 5 minutes, count as abandoned
        AwayFromKeyboard = 4,
        /// Player never connected, no abandon
        NeverConnected = 5,
        /// Player took too long to connect, no abandon
        NeverConnectedTooLong = 6,
    }
}

impl LeaveStatus {
    pub fn into_option(self) -> Option<Self> {
        match self {
            // 0 is normal
            LeaveStatus::Unknown(0) => None,
            status => Some(status),
        }
    }
}

#[derive(Clone, Debug)]
pub enum BuildingStatus {
    Destroyed,
    Stand,
}

#[derive(Clone, Debug)]
pub struct TowerStatus {
    pub ancient_bottom: BuildingStatus,
    pub ancient_top: BuildingStatus,
    pub bottom_tier_3: BuildingStatus,
    pub bottom_tier_2: BuildingStatus,
    pub bottom_tier_1: BuildingStatus,
    pub middle_tier_3: BuildingStatus,
    pub middle_tier_2: BuildingStatus,
    pub middle_tier_1: BuildingStatus,
    pub top_tier_3: BuildingStatus,
    pub top_tier_2: BuildingStatus,
    pub top_tier_1: BuildingStatus,
}

impl From<u32> for TowerStatus {
    fn from(value: u32) -> Self {
        let convert = |bits: u32, index: usize| match bits & (1 << index) != 0 {
            true => BuildingStatus::Stand,
            false => BuildingStatus::Destroyed,
        };
        Self {
            ancient_bottom: convert(value, 10),
            ancient_top: convert(value, 9),
            bottom_tier_3: convert(value, 8),
            bottom_tier_2: convert(value, 7),
            bottom_tier_1: convert(value, 6),
            middle_tier_3: convert(value, 5),
            middle_tier_2: convert(value, 4),
            middle_tier_1: convert(value, 3),
            top_tier_3: convert(value, 2),
            top_tier_2: convert(value, 1),
            top_tier_1: convert(value, 0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BarracksStatus {
    pub bottom_ranged: BuildingStatus,
    pub bottom_melee: BuildingStatus,
    pub middle_ranged: BuildingStatus,
    pub middle_melee: BuildingStatus,
    pub top_ranged: BuildingStatus,
    pub top_melee: BuildingStatus,
}

impl From<u32> for BarracksStatus {
    fn from(value: u32) -> Self {
        let convert = |bits: u32, index: usize| match bits & (1 << index) != 0 {
            true => BuildingStatus::Stand,
            false => BuildingStatus::Destroyed,
        };
        Self {
            bottom_ranged: convert(value, 5),
            bottom_melee: convert(value, 4),
            middle_ranged: convert(value, 3),
            middle_melee: convert(value, 2),
            top_ranged: convert(value, 1),
            top_melee: convert(value, 0),
        }
    }
}
