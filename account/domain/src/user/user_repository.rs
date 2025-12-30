use async_trait::async_trait;
use crate::user::domain_error::DomainError;
use crate::user::user::{User, UserId};

#[async_trait]
pub trait  UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &UserId) -> Result<User, DomainError>;

    async fn insert(&self, user: &User) -> Result<(), DomainError>;
}