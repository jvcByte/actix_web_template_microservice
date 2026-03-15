use entity::users::{Column, Entity as User};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User)
                    .if_not_exists()
                    .col(ColumnDef::new(Column::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Column::Name).string().not_null())
                    .col(ColumnDef::new(Column::Email).string().not_null())
                    .col(
                        ColumnDef::new(Column::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User).to_owned())
            .await
    }
}
