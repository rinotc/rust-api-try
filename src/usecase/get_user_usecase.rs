use crate::domain::domain_error::DomainError;
use crate::domain::user::{User, UserId};
use crate::domain::user_repository::UserRepository;
use crate::libs::usecase::{Input, Output, UseCase};
use std::sync::Arc;

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

impl UseCase<GetUserInput, GetUserOutput> for GetUserUseCase {
    fn execute(&self, input: GetUserInput) -> GetUserOutput {
        match self.user_repository.find_by_id(input.user_id.clone()) {
            Ok(user) => GetUserOutput::Success(user),
            Err(DomainError::NotFound) => GetUserOutput::NotFound(input.user_id),
            Err(_) => GetUserOutput::Error("Internal Infrastructure Error".to_string())
        }
    }
}