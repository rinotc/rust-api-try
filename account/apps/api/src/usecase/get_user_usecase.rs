use std::sync::Arc;
use async_trait::async_trait;
use account_domain::user::domain_error::DomainError;
use account_domain::user::user::{User, UserId};
use account_domain::user::user_repository::UserRepository;
use account_libs::usecase::{Input, Output, UseCase};

pub enum GetUserOutput {
    Success(User),
    NotFound(UserId),
    Error(String)
}
impl Output for GetUserOutput {}

pub struct GetUserInput {
    pub user_id: UserId
}
impl Input<GetUserOutput> for GetUserInput {}

#[derive(Clone)]
pub struct GetUserUseCase {
    user_repository: Arc<dyn UserRepository>
}
impl GetUserUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl UseCase<GetUserInput, GetUserOutput> for GetUserUseCase {
    async fn execute(&self, input: GetUserInput) -> GetUserOutput {
        match self.user_repository.find_by_id(input.user_id.clone()).await {
            Ok(user) => GetUserOutput::Success(user),
            Err(DomainError::NotFound) => GetUserOutput::NotFound(input.user_id),
            Err(_) => GetUserOutput::Error("Internal Infrastructure Error".to_string())
        }
    }
}