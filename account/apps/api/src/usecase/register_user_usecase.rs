use account_domain::user::entity::User;
use std::sync::Arc;
use tracing::{error, instrument};
use account_domain::user::email::Email;
use account_domain::user::name::UserName;
use account_domain::user::repository::UserRepository;

#[derive(Debug)]
pub struct RegisterUserInput {
    pub name: UserName,
    pub email: Email
}

#[derive(Debug)]
pub enum RegisterUserOutput {
    Success(User),
    Error,
}

#[derive(Clone)]
pub struct RegisterUserUseCase {
    user_repository: Arc<dyn UserRepository>,
}

impl RegisterUserUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, input: RegisterUserInput) -> RegisterUserOutput {
        let user = User::create(input.name, input.email);
        match self.user_repository.insert(&user).await {
            Ok(_) => RegisterUserOutput::Success(user),
            Err(e) => {
                error!("Database error occurred while creating user: {:?}", e);
                RegisterUserOutput::Error
            }
        }
    }
}
