use sea_orm::{prelude::DateTimeUtc, *};

use crate::entities::{DatabaseId, jam, submission_media, submission_user};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "jam_submissions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: DatabaseId,

    pub jam_id: DatabaseId,

    pub icon: Option<DatabaseId>,

    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub description: Option<String>,

    pub source_url: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "submission_media::Entity")]
    SubmissionMedia,
    #[sea_orm(has_many = "submission_user::Entity")]
    SubmissionUsers,
    #[sea_orm(
        belongs_to = "jam::Entity",
        from = "Column::JamId",
        to = "jam::Column::Id"
    )]
    Jams,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<submission_media::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::SubmissionMedia.def();
    }
}

impl Related<submission_user::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::SubmissionUsers.def();
    }
}

impl Related<jam::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::Jams.def();
    }
}
