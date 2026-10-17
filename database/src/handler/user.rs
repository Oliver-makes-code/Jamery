use jamery_common::user::User;
use sea_orm::{
    ActiveValue, DbErr, EntityTrait,
    sea_query::{OnConflict, prelude::Utc},
};

use crate::{entities, handler::DatabaseHandler};

impl DatabaseHandler {
    pub async fn upsert_user(
        &self,
        user_id: u64,
        username: String,
        display_name: Option<String>,
        avatar_url: Option<String>,
    ) -> Result<(), DbErr> {
        let user = entities::user::ActiveModel {
            id: ActiveValue::Set(user_id as i64),
            username: ActiveValue::Set(username),
            display_name: ActiveValue::Set(display_name),
            avatar_url: ActiveValue::Set(avatar_url),
            created_at: ActiveValue::Set(Utc::now()),
        };

        entities::user::Entity::insert(user)
            .on_conflict(
                OnConflict::column(entities::user::Column::Id)
                    .update_columns([
                        entities::user::Column::Username,
                        entities::user::Column::DisplayName,
                        entities::user::Column::AvatarUrl,
                    ])
                    .to_owned(),
            )
            .exec(&self.conn)
            .await?;

        return Ok(());
    }

    pub async fn fetch_user(&self, user_id: u64) -> Result<Option<User>, DbErr> {
        let user = entities::user::Entity::find_by_id(user_id as i64)
            .one(&self.conn)
            .await?;

        let Some(user) = user else {
            return Ok(None);
        };

        return Ok(Some(User {
            id: user.id as u64,
            username: user.username,
            display_name: user.display_name,
            avatar_url: user.avatar_url,
        }));
    }
}
