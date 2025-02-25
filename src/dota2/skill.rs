use reqwest::RequestBuilder;

use crate::{Transform, dota2::define_dota2_enum};

define_dota2_enum! {
    /// Skill level of match
    pub enum Skill : u8 {
        Normal = 1,
        High = 2,
        VeryHigh = 3
    }
}

impl Transform<Skill> for RequestBuilder {
    fn transform(self, value: Skill) -> Self {
        self.query(&[("skill", u8::from(value))])
    }
}
