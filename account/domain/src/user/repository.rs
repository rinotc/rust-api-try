use crate::user::entity::User;
use crate::user::id::UserId;
use crate::user::repository::user_repository_errors::{FindById, Insert};
use account_libs::error::app_error::AppError;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &UserId) -> Result<User, AppError<FindById>>;

    async fn insert(&self, user: &User) -> Result<(), AppError<Insert>>;
}

pub mod user_repository_errors {
    #[derive(Debug, thiserror::Error)]
    pub enum FindById {
        #[error("User not found")]
        NotFound
    }

    #[derive(Debug, thiserror::Error)]
    pub enum Insert {
        #[error("User already exists")]
        AlreadyExists
    }
}
