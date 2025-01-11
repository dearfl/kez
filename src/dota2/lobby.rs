use crate::dota2::define_dota2_enum;

define_dota2_enum! {
    /// Lobby type enum
    pub enum Lobby : u8 {
        Normal = 0,
        Practice = 1,
        Tournament = 2,
        Tutorial = 3,
        CoopBots = 4,
        RankedTeam = 5,
        RankedSolo = 6,
        Ranked = 7,
        SoloMid = 8,
        BattleCup = 9,
        LocalBots = 10,
        Spectator = 11,
        Event = 12,
        Gauntlet = 13,
        NewPlayer = 14,
        Featured = 15,
    }
}
