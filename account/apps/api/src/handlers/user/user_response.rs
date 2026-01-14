use serde::Serialize;
use account_domain::user::entity::User;

#[derive(Serialize)]
pub struct UserResponse {
    id: String,
    name: String,
    email: String,
    status: String,
    role: String,
    is_admin: bool,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.0.to_string(),
            is_admin: user.is_admin(),
            name: user.name.0,
            email: user.email.0,
            status: user.status.to_string(),
            role: user.role.to_string(),
        }
    }
}