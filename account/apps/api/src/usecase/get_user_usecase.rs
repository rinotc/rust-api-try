use std::sync::Arc;
use tracing::{error, instrument, warn};
use account_domain::user::domain_error::DomainError;
use account_domain::user::user::{User, UserId};
use account_domain::user::user_repository::UserRepository;

pub enum GetUserOutput {
    Success(User),
    NotFound,
    Error
}

#[derive(Debug)]
pub struct GetUserInput {
    pub user_id: UserId
}

#[derive(Clone)]
pub struct GetUserUseCase {
    user_repository: Arc<dyn UserRepository>
}
impl GetUserUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, input: GetUserInput) -> GetUserOutput {
        match self.user_repository.find_by_id(&input.user_id).await {
            Ok(user) => GetUserOutput::Success(user),
            Err(DomainError::NotFound) => {
                warn!("User not found: {:?}", input.user_id);
                GetUserOutput::NotFound
            },
            Err(e) => {
                error!("Database error occurred while fetching user: {:?}", e);
                GetUserOutput::Error
            }
        }
    }
}