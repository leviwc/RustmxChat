use crate::entities::message;
use sea_orm::*;

pub struct ChatDatabase;

impl ChatDatabase {
    async fn add_message(
        db: &DbConn,
        form_data: message::Model,
    ) -> Result<message::ActiveModel, DbErr> {
        message::ActiveModel {
            text: Set(form_data.text.to_owned()),
            user_id: Set(form_data.user_id.to_owned()),
            chat_id: Set(form_data.chat_id.to_owned()),
            timestamp: Set(form_data.timestamp.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    async fn get_chat_messages_by_id(
        db: &DbConn,
        chat_id: i64,
        page: u64,
        messages_per_page: u64,
    ) -> Result<(Vec<message::Model>, u64), DbErr> {
        let paginator = message::Entity::find()
            .filter(message::Column::ChatId.eq(chat_id))
            .order_by_desc(message::Column::Timestamp)
            .paginate(db, messages_per_page);
        let num_page = paginator.num_pages().await?;
        paginator.fetch_page(page - 1).await.map(|p| (p, num_page))
    }
}
