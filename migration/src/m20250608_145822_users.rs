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
                    .col(pk_auto(Users::Id))
                    .col(string(Users::Username).not_null().unique_key())
                    .col(string(Users::Email).not_null().unique_key())
                    .col(string(Users::Nickname).not_null())
                    .col(string(Users::PasswordHash).not_null())
                    .col(string(Users::Avatar))
                    .col(text(Users::Description))
                    .col(boolean(Users::IsVerified).not_null().default(false))
                    .col(timestamp_with_time_zone(Users::EmailVerifiedAt))
                    .col(timestamp_with_time_zone(Users::LastLoginAt))
                    .col(
                        timestamp_with_time_zone(Users::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Users::UpdatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Nickname,
    Email,
    PasswordHash,
    Avatar,
    Description,
    IsVerified,
    EmailVerifiedAt,
    LastLoginAt,
    CreatedAt,
    UpdatedAt,
}
