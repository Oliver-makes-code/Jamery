use sea_orm_migration::{async_trait::async_trait, prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Challenges::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Challenges::State)
                            .string()
                            .primary_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Challenges::Verifier).string().not_null())
                    .col(ColumnDef::new(Challenges::ExpiresAt).date_time().not_null())
                    .col(ColumnDef::new(Challenges::Consumed).boolean().not_null())
                    .to_owned(),
            )
            .await?;

        return Ok(());
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Challenges::Table).to_owned())
            .await?;

        return Ok(());
    }
}

#[derive(DeriveIden)]
enum Challenges {
    Table,
    State,
    Verifier,
    ExpiresAt,
    Consumed,
}
