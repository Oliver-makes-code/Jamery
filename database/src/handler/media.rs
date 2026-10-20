use sea_orm::{ActiveModelTrait, ActiveValue, DbErr};
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
}
