use account_domain::user::domain_error::DomainError;
use account_domain::user::user::{User, UserId, UserRole};
use account_domain::user::user_repository::UserRepository;

pub struct InMemoryUserRepository;

impl UserRepository for InMemoryUserRepository {
    fn find_by_id(&self, id: UserId) -> Result<User, DomainError> {
        match id.0 {
            _ => Err(DomainError::InfrastructureError),
        }
    }
}