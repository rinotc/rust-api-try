use account_domain::user::domain_error::DomainError;
use account_domain::user::user::{User, UserId};
use account_domain::user::user_repository::UserRepository;
use account_infra_orm::orm::users as UserEntity;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait};

pub struct PostgresUserRepository {
    db: DatabaseConnection,
}

impl PostgresUserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: UserId) -> Result<User, DomainError> {
        let user_model = UserEntity::Entity::find_by_id(id.0)
            .one(&self.db)
            .await
            .map_err(|_| DomainError::InfrastructureError)?
            .ok_or(DomainError::NotFound)?;

        Ok(User::reconstruct(
            UserId(user_model.user_id),
            user_model.name.to_string(),
            user_model.role.parse().unwrap(),
        ))
    }
}
