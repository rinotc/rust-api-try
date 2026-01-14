use account_domain::user::email::Email;
use account_domain::user::entity::User;
use account_domain::user::id::UserId;
use account_domain::user::name::UserName;
use account_domain::user::repository::{user_repository_errors, UserRepository};
use account_domain::user::role::UserRole;
use account_domain::user::status::UserStatus;
use account_infra_orm::orm::users as UserEntity;
use account_libs::error::app_error::AppError;
use anyhow::anyhow;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use std::str::FromStr;

pub struct PostgresUserRepository {
    db: DatabaseConnection,
}

impl PostgresUserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn translate_to_entity(model: UserEntity::Model) -> User {
        User::reconstruct(
            UserId(model.user_id),
            UserName::new(model.name),
            UserStatus::from_str(model.status.as_str()).expect("Invalid status"),
            UserRole::from_str(model.role.as_str()).expect("Invalid role"),
            Email::new(model.email),
        )
    }

    fn translate_from_entity(entity: &User) -> UserEntity::ActiveModel {
        let entity = entity.clone();
        UserEntity::ActiveModel {
            user_id: Set(entity.id.0),
            name: Set(entity.name.0),
            status: Set(entity.status.to_string()),
            role: Set(entity.role.to_string()),
            email: Set(entity.email.0),
        }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(
        &self,
        id: &UserId,
    ) -> Result<User, AppError<user_repository_errors::FindById>> {
        let model = UserEntity::Entity::find_by_id(id.0)
            .one(&self.db)
            .await
            .map_err(|e| {
                AppError::System(anyhow!(e).context("Failed to fetch user from Postgres"))
            })?
            .ok_or(AppError::Business(
                user_repository_errors::FindById::NotFound,
            ))?;

        Ok(Self::translate_to_entity(model))
    }

    async fn insert(&self, user: &User) -> Result<(), AppError<user_repository_errors::Insert>> {
        let active_model = Self::translate_from_entity(user);
        active_model.insert(&self.db).await.map_err(|e| {
            AppError::System(anyhow!(e).context("Failed to insert user into Postgres"))
        })?;
        Ok(())
    }
}
