use std::time::{SystemTime, UNIX_EPOCH};

use crate::entities::{chat, message, user_in_chat};
use migration::OnConflict;
use sea_orm::*;

pub struct ChatDatabase;

impl ChatDatabase {
    pub async fn add_message(
        db: &DbConn,
        text: String,
        user_id: i32,
        chat_id: i32,
    ) -> Result<message::Model, DbErr> {
        let seconds_since_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards!")
            .as_secs() as i32;

        let model = message::ActiveModel {
            text: Set(text.to_owned()),
            user_id: Set(user_id.to_owned()),
            chat_id: Set(chat_id.to_owned()),
            timestamp: Set(seconds_since_epoch.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await?;

        chat::ActiveModel {
            id: ActiveValue::Unchanged(chat_id),
            last_changed_timestamp: Set(seconds_since_epoch),
        }
        .save(db)
        .await?;

        Ok(message::Model::try_from(model)?)
    }

    pub async fn get_chat_messages_by_id(
        db: &DbConn,
        chat_id: i64,
        page: u64,
        messages_per_page: u64,
    ) -> Result<(Vec<message::Model>, u64), DbErr> {
        let paginator = message::Entity::find()
            .filter(message::Column::ChatId.eq(chat_id))
            .order_by_asc(message::Column::Timestamp)
            .paginate(db, messages_per_page);
        let num_page = paginator.num_pages().await?;
        paginator.fetch_page(page - 1).await.map(|p| (p, num_page))
    }

    async fn upseart_chat(db: &DbConn, chat_id: i64) -> Result<(), DbErr> {
        let seconds_since_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards!")
            .as_secs() as i32;
        let new_chat = chat::ActiveModel {
            id: Set(chat_id as i32),
            last_changed_timestamp: Set(seconds_since_epoch),
        };

        let query = chat::Entity::insert(new_chat)
            .on_conflict(OnConflict::columns([chat::Column::Id]))
            .build(DbBackend::Postgres);

        db.execute(query).await?;
        Ok(())
    }

    pub async fn get_chats_by_user_id(
        db: &DbConn,
        user_id: i64,
        page: u64,
        chats_per_page: u64,
    ) -> Result<(Vec<chat::Model>, u64), DbErr> {
        let paginator = chat::Entity::find()
            .join(JoinType::LeftJoin, chat::Relation::UserInChat.def())
            .filter(user_in_chat::Column::UserId.eq(user_id))
            .order_by_asc(chat::Column::LastChangedTimestamp)
            .paginate(db, chats_per_page);
        let num_page = paginator.num_pages().await?;
        paginator.fetch_page(page - 1).await.map(|p| (p, num_page))
    }
}
