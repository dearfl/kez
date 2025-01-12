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
