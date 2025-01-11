/// Lobby type enum
#[non_exhaustive]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default)]
pub enum Lobby {
    Unknown(u8) = 255,
    #[default]
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

impl From<u8> for Lobby {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Normal,
            1 => Self::Practice,
            2 => Self::Tournament,
            3 => Self::Tutorial,
            4 => Self::CoopBots,
            5 => Self::RankedTeam,
            6 => Self::RankedSolo,
            7 => Self::Ranked,
            8 => Self::SoloMid,
            9 => Self::BattleCup,
            10 => Self::LocalBots,
            11 => Self::Spectator,
            12 => Self::Event,
            13 => Self::Gauntlet,
            14 => Self::NewPlayer,
            15 => Self::Featured,
            value => Self::Unknown(value),
        }
    }
}

impl From<Lobby> for u8 {
    fn from(value: Lobby) -> Self {
        match value {
            Lobby::Normal => 0,
            Lobby::Practice => 1,
            Lobby::Tournament => 2,
            Lobby::Tutorial => 3,
            Lobby::CoopBots => 4,
            Lobby::RankedTeam => 5,
            Lobby::RankedSolo => 6,
            Lobby::Ranked => 7,
            Lobby::SoloMid => 8,
            Lobby::BattleCup => 9,
            Lobby::LocalBots => 10,
            Lobby::Spectator => 11,
            Lobby::Event => 12,
            Lobby::Gauntlet => 13,
            Lobby::NewPlayer => 14,
            Lobby::Featured => 15,
            Lobby::Unknown(value) => value,
        }
    }
}
