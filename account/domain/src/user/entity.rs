use crate::user::email::Email;
use crate::user::id::UserId;
use crate::user::name::UserName;
use crate::user::role::UserRole;
use crate::user::status::UserStatus;

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub name: UserName,
    pub status: UserStatus,
    pub role: UserRole,
    pub email: Email,
}

impl User {
    // static メソッド
    pub fn create(name: UserName, email: Email) -> Self {
        Self {
            id: UserId::new(),
            name,
            status: UserStatus::Suspended,
            role: UserRole::User,
            email,
        }
    }

    pub fn reconstruct(
        id: UserId,
        name: UserName,
        status: UserStatus,
        role: UserRole,
        email: Email,
    ) -> Self {
        Self {
            id,
            name,
            status,
            role,
            email,
        }
    }

    // 通常のメソッド
    pub fn is_admin(&self) -> bool {
        matches!(self.role, UserRole::Admin)
    }
}
