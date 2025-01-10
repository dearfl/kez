use reqwest::RequestBuilder;

use crate::Transform;

#[derive(Debug, Default, Clone, Copy)]
pub struct MatchesRequested(u8);

impl From<u8> for MatchesRequested {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<MatchesRequested> for u8 {
    fn from(value: MatchesRequested) -> Self {
        value.0
    }
}

impl Transform<MatchesRequested> for RequestBuilder {
    fn transform(self, value: MatchesRequested) -> Self {
        self.query(&[("matches_requested", value.0)])
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MatchId(u64);

impl From<u64> for MatchId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<MatchId> for u64 {
    fn from(value: MatchId) -> Self {
        value.0
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MatchSeqNum(u64);

impl From<u64> for MatchSeqNum {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<MatchSeqNum> for u64 {
    fn from(value: MatchSeqNum) -> Self {
        value.0
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StartAt<T>(T);

impl<T> From<T> for StartAt<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl Transform<StartAt<MatchId>> for RequestBuilder {
    fn transform(self, value: StartAt<MatchId>) -> Self {
        self.query(&[("start_at_match_id", u64::from(value.0))])
    }
}

impl Transform<StartAt<MatchSeqNum>> for RequestBuilder {
    fn transform(self, value: StartAt<MatchSeqNum>) -> Self {
        self.query(&[("start_at_match_seq_num", u64::from(value.0))])
    }
}
