// This module contains many parameter types used to request Steam HTTP API and the types returned by those APIs.
// NOTE: Some types may not be what they really are because I have no real way to find out the real type behind these APIs.
// NOTE: I've intentionaly used narrower type for some field, for example u8 for hero id, while it really should be u32.
// NOTE: Valve may change these types as dota2 update, any update will cause a BREAKING change for this crate.

// pub modules
pub mod get_heroes;
pub mod get_match_history;
pub mod get_match_history_by_seq_num;
pub mod hero;
pub mod mode;
pub mod skill;

pub use hero::Hero;
pub use skill::Skill;
