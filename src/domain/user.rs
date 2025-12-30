#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserId(pub u64);

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
    pub fn new(id: u64, name: &str, role: UserRole) -> Self {
        Self { id: UserId(id), name: name.to_string(), role }
    }

    // 通常のメソッド
    pub fn is_admin(&self) -> bool {
        matches!(self.role, UserRole::Admin)
    }
}