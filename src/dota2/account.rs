use reqwest::RequestBuilder;

use crate::Transform;

/// User account repr
#[non_exhaustive]
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Account {
    /// I'm actually not sure 0 => Bot
    Bot,
    /// should be u32::MAX(4294967295) => Anonymous
    Anonymous,
    User(u32),
}

impl Default for Account {
    fn default() -> Self {
        Self::Anonymous
    }
}

impl From<u32> for Account {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Bot,
            u32::MAX => Self::Anonymous,
            id => Self::User(id),
        }
    }
}

impl From<Account> for u32 {
    fn from(value: Account) -> Self {
        match value {
            Account::Bot => 0,
            Account::Anonymous => u32::MAX,
            Account::User(id) => id,
        }
    }
}

impl Transform<Account> for RequestBuilder {
    fn transform(self, value: Account) -> Self {
        self.query(&[("account_id", u32::from(value))])
    }
}
