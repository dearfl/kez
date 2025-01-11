// This module contains many parameter types used to request Steam HTTP API and the types returned by those APIs.
// NOTE: Some types may not be what they really are because I have no real way to find out the real type behind these APIs.
// NOTE: I've intentionaly used narrower type for some field, for example u8 for hero id, while it really should be u32.
// NOTE: Valve may change these types as dota2 update, any update will cause a BREAKING change for this crate.

macro_rules! define_dota2_enum {
    (
        $(#[doc = $doc:expr])*
        pub enum $name:ident : $base:ty {
            $($item:ident = $value:literal),* $(,)?
        }
    ) => {
        $(#[doc = $doc])*
        #[non_exhaustive]
        #[repr($base)]
        #[derive(Clone, Copy, Debug)]
        pub enum $name {
            $($item = $value),*,
            Unknown($base) = <$base>::MAX,
        }

        impl Default for $name {
            fn default() -> Self {
                Self::Unknown(<$base>::MIN)
            }
        }

        impl From<$base> for $name {
            fn from(value: $base) -> Self {
                match value {
                    $($value => Self::$item),*,
                    unknown => Self::Unknown(unknown),
                }
            }
        }

        impl From<$name> for $base {
            fn from(value: $name) -> Self {
                match value {
                    $($name::$item => $value),*,
                    $name::Unknown(value) => value,
                }
            }
        }
    };
}

pub(crate) use define_dota2_enum;

// pub modules for APIs
pub mod get_heroes;
pub mod get_match_history;
pub mod get_match_history_by_seq_num;

// common types related to dota2
pub mod ability;
pub mod account;
pub mod hero;
pub mod item;
pub mod league;
pub mod lobby;
pub mod r#match;
pub mod mode;
pub mod player;
pub mod side;
pub mod skill;

pub use ability::Ability;
pub use account::Account;
pub use hero::Hero;
pub use item::Item;
pub use league::League;
pub use lobby::Lobby;
pub use mode::Mode;
pub use player::Player;
pub use r#match::{MatchId, MatchSeqNum, MatchesRequested, StartAt};
pub use side::Side;
pub use skill::Skill;
