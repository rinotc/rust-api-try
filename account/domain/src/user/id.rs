use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserId(pub Uuid);

impl UserId {
    pub(super) fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl FromStr for UserId {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s)
            .map(|uuid| Self(uuid))
            .map_err(|e| format!("invalid UUID: {}", e))
    }
}
