use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseTransaction, DbErr, EntityTrait,
    QueryFilter, TransactionTrait,
    prelude::Expr,
    sea_query::prelude::{Utc, chrono::Duration},
};

use crate::{entities, handler::DatabaseHandler};

impl DatabaseHandler {
    pub async fn begin_challenge(&self, state: String, verifier: String) -> Result<(), DbErr> {
        let challenge = entities::challenge::ActiveModel {
            state: ActiveValue::Set(state),
            verifier: ActiveValue::Set(verifier),
            expires_at: ActiveValue::Set(Utc::now() + Duration::minutes(5)),
            consumed: ActiveValue::Set(false),
        };

        challenge.insert(&self.conn).await?;

        return Ok(());
    }

    async fn consume_challenge_internal(
        &self,
        state: &str,
        txn: &DatabaseTransaction,
    ) -> Result<Option<String>, DbErr> {
        let claim = entities::challenge::Entity::update_many()
            .col_expr(entities::challenge::Column::Consumed, Expr::value(true))
            .filter(entities::challenge::Column::State.eq(state))
            .filter(entities::challenge::Column::Consumed.eq(false))
            .filter(entities::challenge::Column::ExpiresAt.gt(Utc::now()))
            .exec(txn)
            .await?;

        if claim.rows_affected != 1 {
            return Ok(None);
        }

        let challenge = entities::challenge::Entity::find_by_id(state.to_owned())
            .one(txn)
            .await?;

        entities::challenge::Entity::delete_by_id(state.to_owned())
            .exec(txn)
            .await?;

        return Ok(challenge.map(|it| it.verifier));
    }

    pub async fn consume_challenge(&self, state: &str) -> Result<Option<String>, DbErr> {
        let txn = self.conn.begin().await?;

        match self.consume_challenge_internal(state, &txn).await {
            Ok(value) => {
                txn.commit().await?;
                return Ok(value);
            }
            Err(err) => {
                txn.rollback().await?;
                return Err(err);
            }
        }
    }
}
