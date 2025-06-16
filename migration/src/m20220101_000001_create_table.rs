use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Anime::Table)
                    .if_not_exists()
                    .col(pk_auto(Anime::Id))
                    .col(string(Anime::Title).not_null())
                    .col(string(Anime::Poster))
                    .col(text(Anime::Description))
                    .col(integer(Anime::Episodes))
                    .col(json(Anime::Related))
                    .col(json(Anime::ScreenShot))
                    .col(json(Anime::Player))
                    .col(
                        timestamp_with_time_zone(Anime::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Anime::UpdatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Anime::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Anime {
    Table,
    Id,
    Title,
    Poster,
    Description,
    Episodes,
    Related,
    ScreenShot,
    Player,
    CreatedAt,
    UpdatedAt,
}
