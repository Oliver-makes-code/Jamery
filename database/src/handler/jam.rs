use chrono::{DateTime, Utc};
use jamery_common::jam::{Jam, JamCreate};
use sea_orm::{ActiveModelTrait, ActiveValue, DbErr, EntityTrait, QuerySelect};
use ulid::Ulid;

use crate::{
    entities::{self, DatabaseId},
    handler::DatabaseHandler,
};

impl DatabaseHandler {
    fn jam_model_to_api(model: entities::jam::Model) -> Jam {
        return Jam {
            id: model.id.into(),
            created_by: model.created_by.into(),
            title: model.title,
            summary: model.summary,
            description: model.description,
            starts: model.starts,
            ends: model.ends,
        };
    }

    pub async fn create_jam(&self, created_by: Ulid, create: JamCreate) -> Result<Jam, DbErr> {
        let jam = entities::jam::ActiveModel {
            id: ActiveValue::Set(DatabaseId::new()),
            created_by: ActiveValue::Set(DatabaseId::from(created_by)),
            title: ActiveValue::Set(create.title),
            summary: ActiveValue::Set(create.summary),
            description: ActiveValue::Set(create.description),
            starts: ActiveValue::Set(create.starts),
            ends: ActiveValue::Set(create.ends),
        };

        let model = jam.insert(&self.conn).await?;

        return Ok(Self::jam_model_to_api(model));
    }

    pub async fn fetch_jam(&self, id: Ulid) -> Result<Option<Jam>, DbErr> {
        let value = entities::jam::Entity::find_by_id(DatabaseId::from(id))
            .one(&self.conn)
            .await?;

        return Ok(value.map(Self::jam_model_to_api));
    }

    pub async fn fetch_jams(&self, page: u64, page_size: u64) -> Result<Box<[Jam]>, DbErr> {
        let page_size = page_size.clamp(1, 256);
        let offset = page * page_size;

        let values = entities::jam::Entity::find()
            .order_by_id(sea_orm::Order::Desc)
            .offset(offset)
            .limit(page_size)
            .all(&self.conn)
            .await?;

        return Ok(values.into_iter().map(Self::jam_model_to_api).collect());
    }
}
