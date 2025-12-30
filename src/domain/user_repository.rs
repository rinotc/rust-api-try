use crate::domain::domain_error::DomainError;
use crate::domain::user::{User, UserId, UserRole};

pub trait  UserRepository: Send + Sync {
    fn find_by_id(&self, id: UserId) -> Result<User, DomainError>;
}