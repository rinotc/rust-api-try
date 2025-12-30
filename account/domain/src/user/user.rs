use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserId(pub Uuid);

impl UserId {
    fn new() -> Self {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserRole {
    Admin,
    User,
}

impl UserRole {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Admin => "ADMIN",
            Self::User => "USER",
        }
    }

    const VARIANTS: &'static [Self] = &[Self::Admin, Self::User];
}

impl FromStr for UserRole {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::VARIANTS
            .iter()
            .find(|v| v.code() == s)
            .cloned()
            .ok_or_else(|| format!("invalid role: {}", s))
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub role: UserRole,
}

impl User {
    // static メソッド
    pub fn create(name: String) -> Self {
        Self {
            id: UserId::new(),
            name,
            role: UserRole::User,
        }
    }

    pub fn reconstruct(id: UserId, name: String, role: UserRole) -> Self {
        Self { id, name, role }
    }

    // 通常のメソッド
    pub fn is_admin(&self) -> bool {
        matches!(self.role, UserRole::Admin)
    }
}
