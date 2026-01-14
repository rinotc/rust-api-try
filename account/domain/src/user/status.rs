use strum::{Display, EnumIter, EnumString};

/// ユーザーアカウントの状態
#[derive(Debug, Clone, PartialEq, Eq, Display, EnumString, EnumIter)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum UserStatus {
    /// アクティブなユーザー
    Active,
    /// 一時停止されたユーザー
    Suspended,
}

impl UserStatus {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }
}

