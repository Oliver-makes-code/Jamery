use sea_orm::{DbErr, DeriveValueType, TryFromU64, prelude::Uuid};
use ulid::Ulid;

pub mod challenge;
pub mod jam;
pub mod media;
pub mod submission;
pub mod submission_media;
pub mod submission_user;
pub mod user;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DeriveValueType)]
pub struct DatabaseId(Uuid);

impl DatabaseId {
    pub fn new() -> Self {
        return Self(Uuid::from_bytes(Ulid::generate().to_bytes()));
    }
}

impl From<DatabaseId> for Ulid {
    fn from(value: DatabaseId) -> Self {
        return Self::from_bytes(value.0.into_bytes());
    }
}

impl From<Ulid> for DatabaseId {
    fn from(value: Ulid) -> Self {
        return Self(Uuid::from_bytes(value.to_bytes()));
    }
}

impl TryFromU64 for DatabaseId {
    fn try_from_u64(_: u64) -> Result<Self, sea_orm::prelude::DbErr> {
        return Err(DbErr::ConvertFromU64(
            "Ulid cannot be converted from a u64.",
        ));
    }
}
