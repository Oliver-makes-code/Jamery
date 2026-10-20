use sea_orm::*;

use crate::entities::{DatabaseId, submission};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "jam_submission_users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub submission_id: DatabaseId,

    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "submission::Entity",
        from = "Column::SubmissionId",
        to = "submission::Column::Id"
    )]
    Submission,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<submission::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::Submission.def();
    }
}
