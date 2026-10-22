use jamery_common::user::User;
use sea_orm::{
    ActiveValue, ColumnTrait, DbErr, EntityTrait, QueryFilter,
    sea_query::{OnConflict, prelude::Utc},
};
use ulid::Ulid;

use crate::{
    entities::{self, DatabaseId},
    handler::DatabaseHandler,
};

impl DatabaseHandler {
    pub async fn upsert_user(
        &self,
        fluxer_id: u64,
        username: String,
        display_name: Option<String>,
        avatar_url: Option<String>,
    ) -> Result<Ulid, DbErr> {
        let user = entities::user::ActiveModel {
            id: ActiveValue::Set(DatabaseId::new()),
            fluxer_id: ActiveValue::Set(fluxer_id as i64),
            username: ActiveValue::Set(username),
            display_name: ActiveValue::Set(display_name),
            avatar_url: ActiveValue::Set(avatar_url),
            created_at: ActiveValue::Set(Utc::now()),
        };

        entities::user::Entity::insert(user)
            .on_conflict(
                OnConflict::column(entities::user::Column::FluxerId)
                    .update_columns([
                        entities::user::Column::Username,
                        entities::user::Column::DisplayName,
                        entities::user::Column::AvatarUrl,
                    ])
                    .to_owned(),
            )
            .exec(&self.conn)
            .await?;

        let user = entities::user::Entity::find()
            .filter(entities::user::Column::FluxerId.eq(fluxer_id))
            .one(&self.conn)
            .await?
            .ok_or(DbErr::RecordNotFound("User not found".to_owned()))?;

        return Ok(user.id.into());
    }

    pub async fn fetch_user(&self, user_id: Ulid) -> Result<Option<User>, DbErr> {
        let user = entities::user::Entity::find_by_id(DatabaseId::from(user_id))
            .one(&self.conn)
            .await?;

        let Some(user) = user else {
            return Ok(None);
        };

        return Ok(Some(User {
            id: user.id.into(),
            username: user.username,
            display_name: user.display_name,
            avatar_url: user.avatar_url,
        }));
    }
}
