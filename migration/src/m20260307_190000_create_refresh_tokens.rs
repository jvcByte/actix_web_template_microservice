use entity::refresh_tokens::{Column as RefreshTokenColumn, Entity as RefreshToken};
use entity::users::{Column as UserColumn, Entity as User};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create refresh_tokens table
        manager
            .create_table(
                Table::create()
                    .table(RefreshToken)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RefreshTokenColumn::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RefreshTokenColumn::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(RefreshTokenColumn::TokenHash)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(RefreshTokenColumn::ExpiresAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(RefreshTokenColumn::CreatedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(RefreshTokenColumn::Revoked)
                            .boolean()
                            .not_null()
                            .default(Value::Bool(Some(false))),
                    )
                    // Foreign key to users(id)
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_refresh_tokens_user")
                            .from(RefreshToken, RefreshTokenColumn::UserId)
                            .to(User, UserColumn::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Index on token_hash for quick lookup (used when validating presented refresh tokens)
        manager
            .create_index(
                Index::create()
                    .name("idx_refresh_tokens_token_hash")
                    .table(RefreshToken)
                    .col(RefreshTokenColumn::TokenHash)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop index then table
        manager
            .drop_index(
                Index::drop()
                    .name("idx_refresh_tokens_token_hash")
                    .table(RefreshToken)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(RefreshToken).to_owned())
            .await
    }
}
