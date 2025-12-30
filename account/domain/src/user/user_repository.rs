use crate::user::domain_error::DomainError;
use crate::user::user::{User, UserId};

pub trait  UserRepository: Send + Sync {
    fn find_by_id(&self, id: UserId) -> Result<User, DomainError>;
}