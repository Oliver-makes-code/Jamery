use jamery_common::media::Media;
use sea_orm::{ActiveModelTrait, ActiveValue, DbErr, EntityTrait};
use ulid::Ulid;

use crate::{
    entities::{DatabaseId, media},
    handler::DatabaseHandler,
};

impl DatabaseHandler {
    pub async fn insert_media(&self, id: Ulid, file_name: String) -> Result<(), DbErr> {
        let content_type = mime_guess::from_path(&file_name).first_or_octet_stream();

        media::ActiveModel {
            id: ActiveValue::Set(DatabaseId::from(id)),
            file_name: ActiveValue::Set(file_name),
            content_type: ActiveValue::Set(content_type.essence_str().to_string()),
        }
        .insert(&self.conn)
        .await?;

        return Ok(());
    }

    pub async fn fetch_media(&self, id: Ulid) -> Result<Option<Media>, DbErr> {
        let media = media::Entity::find_by_id(DatabaseId::from(id))
            .one(&self.conn)
            .await?;

        return Ok(media.map(|it| Media {
            id: it.id.into(),
            file_name: it.file_name,
            content_type: it.content_type,
        }));
    }
}
