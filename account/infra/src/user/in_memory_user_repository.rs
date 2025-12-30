use account_domain::user::domain_error::DomainError;
use account_domain::user::user::{User, UserId, UserRole};
use account_domain::user::user_repository::UserRepository;

pub struct InMemoryUserRepository;

impl UserRepository for InMemoryUserRepository {
    fn find_by_id(&self, id: UserId) -> Result<User, DomainError> {
        match id.0 {
            1 => Ok(User::new(id.0, "name", UserRole::Admin)),
            2 => Ok(User::new(id.0, "name", UserRole::User)),
            3 => Err(DomainError::NotFound),
            _ => Err(DomainError::InfrastructureError),
        }
    }
}