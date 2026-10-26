use sea_orm::{prelude::DateTimeUtc, *};

use crate::entities::{DatabaseId, submission};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "jams")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: DatabaseId,

    pub created_by: i64,

    pub title: String,
    pub short_description: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub description: Option<String>,

    pub starts: DateTimeUtc,
    pub ends: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "submission::Entity")]
    Submissions,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<submission::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::Submissions.def();
    }
}
