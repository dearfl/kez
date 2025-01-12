use crate::dota2::Item;

#[derive(Clone, Debug, Default)]
pub struct Unit {
    pub name: String,
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
}

impl From<crate::dota2::get_match_history_by_seq_num::Unit> for Unit {
    fn from(unit: crate::dota2::get_match_history_by_seq_num::Unit) -> Self {
        Self {
            name: unit.unitname,
            item_0: Item::from(unit.item_0).into_option(),
            item_1: Item::from(unit.item_1).into_option(),
            item_2: Item::from(unit.item_2).into_option(),
            item_3: Item::from(unit.item_3).into_option(),
            item_4: Item::from(unit.item_4).into_option(),
            item_5: Item::from(unit.item_5).into_option(),
            backpack_0: Item::from(unit.backpack_0).into_option(),
            backpack_1: Item::from(unit.backpack_1).into_option(),
            backpack_2: Item::from(unit.backpack_2).into_option(),
            item_neutral: Item::from(unit.item_neutral).into_option(),
        }
    }
}
