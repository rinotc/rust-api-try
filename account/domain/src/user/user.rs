use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserId(pub Uuid);

impl UserId {
    fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_str(s: &str) -> Self {
        let uuid = Uuid::parse_str(s).unwrap();
        Self(uuid)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserRole {
    Admin,
    User,
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub role: UserRole,
}

impl User {
    // static メソッド
    pub fn create(name: &str, role: UserRole) -> Self {
        Self { id: UserId::new(), name: name.to_string(), role }
    }

    pub fn reconstruct(id: UserId, name: String, role: UserRole) -> Self {
        Self { id, name, role }
    }

    // 通常のメソッド
    pub fn is_admin(&self) -> bool {
        matches!(self.role, UserRole::Admin)
    }
}