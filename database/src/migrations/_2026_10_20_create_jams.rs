use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Game jams
        manager
            .create_table(
                Table::create()
                    .table(Jams::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Jams::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Jams::Title).string().not_null())
                    .col(ColumnDef::new(Jams::Description).text().null())
                    .col(ColumnDef::new(Jams::CreatedBy).big_integer().not_null())
                    .col(
                        ColumnDef::new(Jams::Starts)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Jams::Ends)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .check(Expr::col(Jams::Ends).gt(Expr::col(Jams::Starts)))
                    .to_owned(),
            )
            .await?;

        // Game submissions
        manager
            .create_table(
                Table::create()
                    .table(JamSubmissions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(JamSubmissions::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(JamSubmissions::JamId).uuid().not_null())
                    .col(ColumnDef::new(JamSubmissions::Title).string().not_null())
                    .col(ColumnDef::new(JamSubmissions::Description).text().null())
                    .col(ColumnDef::new(JamSubmissions::Icon).uuid().not_null())
                    .col(ColumnDef::new(JamSubmissions::SourceUrl).string().null())
                    .col(
                        ColumnDef::new(JamSubmissions::Updated)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_jam_submissions_jam")
                            .from(JamSubmissions::Table, JamSubmissions::JamId)
                            .to(Jams::Table, Jams::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Submission files
        manager
            .create_table(
                Table::create()
                    .table(JamSubmissionMedia::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(JamSubmissionMedia::SubmissionId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(JamSubmissionMedia::MediaId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(JamSubmissionMedia::Uploader)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(JamSubmissionMedia::SubmissionId)
                            .col(JamSubmissionMedia::MediaId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_submission_media_submission")
                            .from(JamSubmissionMedia::Table, JamSubmissionMedia::SubmissionId)
                            .to(JamSubmissions::Table, JamSubmissions::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_submission_media_media")
                            .from(JamSubmissionMedia::Table, JamSubmissionMedia::MediaId)
                            .to(Media::Table, Media::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Submission authors
        manager
            .create_table(
                Table::create()
                    .table(JamSubmissionUsers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(JamSubmissionUsers::SubmissionId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(JamSubmissionUsers::UserId)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(JamSubmissionUsers::SubmissionId)
                            .col(JamSubmissionUsers::UserId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_submission_users_submission")
                            .from(JamSubmissionUsers::Table, JamSubmissionUsers::SubmissionId)
                            .to(JamSubmissions::Table, JamSubmissions::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        return Ok(());
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(JamSubmissionUsers::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(JamSubmissionMedia::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(JamSubmissions::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Jams::Table).if_exists().to_owned())
            .await?;

        return Ok(());
    }
}

#[derive(DeriveIden)]
enum Jams {
    Table,
    Id,
    Title,
    Description,
    CreatedBy,
    Starts,
    Ends,
}

#[derive(DeriveIden)]
enum JamSubmissions {
    Table,
    Id,
    JamId,
    Title,
    Icon,
    Description,
    SourceUrl,
    Updated,
}

#[derive(DeriveIden)]
enum JamSubmissionMedia {
    Table,
    SubmissionId,
    MediaId,
    Uploader,
}

#[derive(DeriveIden)]
enum JamSubmissionUsers {
    Table,
    SubmissionId,
    UserId,
}

#[derive(DeriveIden)]
enum Media {
    Table,
    Id,
}
