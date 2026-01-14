use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::UserId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(string(Users::Name).not_null())
                    .col(string(Users::Status).not_null())
                    .col(string(Users::Role).not_null())
                    .col(string(Users::Email).not_null())
                    .to_owned(),
            )
            .await
    }

    // ロールバック用のメソッド
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    UserId,
    Name,
    Status,
    Role,
    Email,
}
