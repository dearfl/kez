use crate::dota2::Hero;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DraftOp {
    Pick,
    Ban,
}

/// Draft is the process of selecting heroes for a match.
/// Different game mode have different drafting process,
/// but each draft is either a pick or a ban of some specific hero.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Draft {
    pub op: DraftOp,
    pub hero: Hero,
    pub team: u8,
    pub order: u8,
}

impl From<crate::dota2::get_match_history_by_seq_num::Draft> for Draft {
    fn from(value: crate::dota2::get_match_history_by_seq_num::Draft) -> Self {
        let op = match value.is_pick {
            true => DraftOp::Pick,
            false => DraftOp::Ban,
        };
        Self {
            op,
            hero: value.hero_id.into(),
            team: value.team,
            order: value.order,
        }
    }
}
