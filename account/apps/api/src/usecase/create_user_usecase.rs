use account_domain::user::user::User;
use account_domain::user::user_repository::UserRepository;
use std::sync::Arc;
use tracing::{error, instrument};

#[derive(Debug)]
pub struct CreateUserInput {
    pub name: String,
}

pub enum CreateUserOutput {
    Success(User),
    Error,
}

#[derive(Clone)]
pub struct CreateUserUseCase {
    user_repository: Arc<dyn UserRepository>,
}

impl CreateUserUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, input: CreateUserInput) -> CreateUserOutput {
        let user = User::create(input.name);
        match self.user_repository.insert(&user).await {
            Ok(_) => CreateUserOutput::Success(user),
            Err(e) => {
                error!("Database error occurred while creating user: {:?}", e);
                CreateUserOutput::Error
            }
        }
    }
}
