use crate::TransformRequest;

/// User account repr
#[non_exhaustive]
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Account {
    /// I'm actually not sure 0 => Bot
    Bot,
    /// should be u64::MAX => Anonymous
    Anonymous,
    /// Other id should be non-anonymous human user
    /// u32 actually? but we stick to u64 for now.
    User(u64),
}

impl Default for Account {
    fn default() -> Self {
        Self::Anonymous
    }
}

impl From<u64> for Account {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Bot,
            u64::MAX => Self::Anonymous,
            id => Self::User(id),
        }
    }
}

impl From<Account> for u64 {
    fn from(value: Account) -> Self {
        match value {
            Account::Bot => 0,
            Account::Anonymous => u64::MAX,
            Account::User(id) => id,
        }
    }
}

impl TransformRequest for Account {
    fn transform_request(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        req.query(&[("account_id", u64::from(*self))])
    }
}
