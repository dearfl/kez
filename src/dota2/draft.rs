use crate::dota2::Hero;

#[derive(Clone, Debug)]
pub enum DraftOp {
    Pick,
    Ban,
}

#[derive(Clone, Debug)]
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
